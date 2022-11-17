
use std::collections::BTreeMap;
use usiem::chrono::LocalResult;
use usiem::chrono::prelude::{TimeZone, Utc};
use reqwest::header::{HeaderValue};
use reqwest::blocking::{Client, ClientBuilder};
use usiem::prelude::alert::{SiemAlert, AlertSeverity};

use crate::api::block::*;
use crate::api::database::*;
use crate::api::database::properties::*;
use crate::api::page::*;



pub type NotionResult<T> = Result<T, NotionError>;

#[derive(Debug)]
pub enum NotionError {
    Connection(reqwest::Error),
    Serialization(usiem::serde_json::Error),
    Server(String)
}

impl From<reqwest::Error> for NotionError {
    fn from(e: reqwest::Error) -> Self {
        NotionError::Connection(e)
    }
}

impl From<usiem::serde_json::Error> for NotionError {
    fn from(e: usiem::serde_json::Error) -> Self {
        NotionError::Serialization(e)
    }
}

pub struct NotionClient {
    database_id : String,
    client : Client
}

impl NotionClient {
    pub fn new(api_key : &str, database_id : &str) -> Self {
        let mut headers = reqwest::header::HeaderMap::new();
        let bearer_key = format!("Bearer {}", api_key);
        headers.insert("Authorization", HeaderValue::from_str(&bearer_key).unwrap());
        headers.insert("Notion-Version", HeaderValue::from_static("2022-06-28"));
        headers.insert("Content-Type", HeaderValue::from_static("application/json"));
        let client = ClientBuilder::new().default_headers(headers).build().unwrap();
        Self {
            database_id : database_id.to_owned(),
            client
        }
    }

    pub fn check_valid_siem_database(&self) -> NotionResult<bool>{
        let response = self.client.get(&format!("https://api.notion.com/v1/databases/{}",self.database_id)).send()?;
        let response = response.error_for_status()?;
        let body = response.text()?;
        let database_obj : DatabaseDefinition = usiem::serde_json::from_str(&body)?;
        Ok(Self::check_properties(&database_obj))
    }

    pub fn send_alert(&self, alert : &SiemAlert) -> NotionResult<()>{
        let mut properties = BTreeMap::new();
        properties.insert("Name".to_owned(), PropertyValue::Title(TitleValue::new(&alert.title)));
        properties.insert("Priority".to_owned(), PropertyValue::Select(SelectValue{
            select : SelectValueInternal {
                name : alert_severity(&alert.severity)
            }
        }));
        
        properties.insert("MITRE".to_owned(), PropertyValue::MultiSelect(MultiSelectValue {
            multi_select : alert.techniques.iter().map(|v| {
                MultiSelectValueInternal {
                    name : format!("{:?}",v)
                }
            }).collect()
        }));
        properties.insert("Tags".to_owned(), PropertyValue::MultiSelect(MultiSelectValue {
            multi_select : alert.tags.iter().map(|v| {
                MultiSelectValueInternal {
                    name : v.to_string()
                }
            }).collect()
        }));
        let fired = match Utc.timestamp_millis_opt(alert.date) {
            LocalResult::Single(v) => v,
            _ => Utc::now()
        };
        let fired = format!("{:?}",fired);
        properties.insert("Fired".to_owned(), PropertyValue::Date(DateValue::new(fired)));

        let children = vec![
            BlockElement::HeadingOne(RichTextValue {
                rich_text: vec![RichTextObject::new(&alert.title)]
            }),
            BlockElement::HeadingThree(RichTextValue {
                rich_text: vec![RichTextObject::new(&alert.rule)]
            }),
            BlockElement::Paragraph(RichTextValue {
                rich_text: vec![RichTextObject::new(&alert.description)]
            }),
            BlockElement::code(alert.log.message(), None),
            BlockElement::code_owned(usiem::serde_json::to_string_pretty(&alert.log).unwrap_or("Cannot show the log".to_owned()), Some(format!("json")))
        ];
        let new_page : PageElement = PageElement { parent: DatabaseParent {
            database_id : self.database_id.clone()
        }, properties, children};
        let response = self.client.post("https://api.notion.com/v1/pages").json(&new_page).send()?;
        if !response.status().is_success() {
            return Err(NotionError::Server(response.text()?))
        }
        Ok(())
    }

    fn check_properties(properties : &DatabaseDefinition) -> bool {
        let name = properties.properties.get("Name");
        let priority = properties.properties.get("Priority");
        let mitre = properties.properties.get("MITRE");
        let tags = properties.properties.get("Tags");
        let status = properties.properties.get("Status");
        let fired = properties.properties.get("Fired");

        if let (Some(name), Some(priority),Some(mitre),Some(tags),Some(status), Some(fired)) = (name, priority, mitre, tags, status, fired) {
            if let (PropertyDefinition::Title(_),PropertyDefinition::Select(_),PropertyDefinition::MultiSelect(_),PropertyDefinition::MultiSelect(_),PropertyDefinition::Status(_), PropertyDefinition::Date(_)) = (name, priority, mitre, tags, status, fired) {
                return true;
            }else {
                return false
            }
        }else{
            return false;
        }
    }
}

fn alert_severity(severity : &AlertSeverity) -> String {
    match severity {
        AlertSeverity::INFORMATIONAL => "Informational".to_string(),
        AlertSeverity::LOW => "Low".to_string(),
        AlertSeverity::MEDIUM => "Medium".to_string(),
        AlertSeverity::HIGH => "High".to_string(),
        AlertSeverity::CRITICAL => "Critical".to_string(),
    }
}

#[cfg(test)]
mod client {
    use std::{borrow::Cow, time::UNIX_EPOCH};
    use reqwest::header::{HeaderValue};
    use usiem::prelude::{alert::{SiemAlert, AlertSeverity}, mitre::MitreTechniques, SiemLog, SiemEvent, auth::{AuthEvent, AuthLoginType, LoginOutcome, RemoteLogin}};
    use crate::api::{database::{DatabaseDefinition}};

    #[test]
    fn test_connection(){
        let db_id : String = match std::env::var("USIEM_NOTION_DB") {
            Ok(v) => v,
            Err(_) => return
        };
        let api_key = std::env::var("USIEM_NOTION_APIKEY").expect("USIEM_NOTION_DB is defined but not USIEM_NOTION_APIKEY");
        
        let api_key = format!("Bearer {}", api_key);
        let mut headers = reqwest::header::HeaderMap::new();
        headers.insert("Authorization", HeaderValue::from_str(&api_key).unwrap());
        headers.insert("Notion-Version", HeaderValue::from_static("2022-06-28"));
        let client = reqwest::blocking::ClientBuilder::new().default_headers(headers).build().unwrap();
        let response = client.get(&format!("https://api.notion.com/v1/databases/{}",db_id)).send().unwrap();
        assert!(response.status().is_success());
        let body = response.text().unwrap();
        let database_obj : DatabaseDefinition = usiem::serde_json::from_str(&body).unwrap();
    
        assert!(database_obj.properties.get("Name").is_some());
        assert!(database_obj.properties.get("Priority").is_some());
        assert!(database_obj.properties.get("MITRE").is_some());
        assert!(database_obj.properties.get("Tags").is_some());
        assert!(database_obj.properties.get("Status").is_some());
    }

    #[test]
    fn test_client_alert() {
        let db_id : String = match std::env::var("USIEM_NOTION_DB") {
            Ok(v) => v,
            Err(_) => return
        };
        let api_key = std::env::var("USIEM_NOTION_APIKEY").expect("USIEM_NOTION_DB is defined but not USIEM_NOTION_APIKEY");
        let client = super::NotionClient::new(&api_key, &db_id);
        let mut log = SiemLog::new(String::from("This is a log example"), 0, "localhost");
        log.set_tenant(Cow::Borrowed("Contoso"));
        log.set_event(SiemEvent::Auth(AuthEvent {
            hostname: Cow::Borrowed("hostname1"),
            outcome: LoginOutcome::FAIL,
            login_type: AuthLoginType::Remote(RemoteLogin {
                domain: Cow::Borrowed("CNMS"),
                source_address: Cow::Borrowed("10.10.10.10"),
                user_name: Cow::Borrowed("cancamusa"),
            }),
        }));
        let alert = SiemAlert {
            title: format!("(TEST) test_client_alert {:?}", std::time::SystemTime::now().duration_since(UNIX_EPOCH).unwrap_or_default().as_secs()),
            description: String::from("This is a test of the NotionAlerter component"),
            severity: AlertSeverity::CRITICAL,
            date: usiem::chrono::Utc::now().timestamp_millis(),
            tags: vec![String::from("Critical")],
            techniques : vec![MitreTechniques::T1001, MitreTechniques::T1003_001],
            rule: String::from("ruleset::example::rule1"),
            log: log.clone(),
            aggr_limit: 0,
            aggr_key: String::from("example::rule"),
        };
        client.send_alert(&alert).unwrap();
    }
}
