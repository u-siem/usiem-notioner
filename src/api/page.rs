use std::collections::BTreeMap;

use serde::{Deserialize, Serialize};

use super::{database::properties::*, block::{BlockElement}};


#[derive(Default, Debug, Deserialize, Serialize)]
pub struct PageElement {
    pub parent : DatabaseParent,
    pub properties : BTreeMap<String, PropertyValue>,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub children : Vec<BlockElement>
}
#[derive(Default, Debug, Deserialize, Serialize)]
pub struct DatabaseParent {
    pub database_id : String
}

#[derive(Default, Debug, Deserialize, Serialize)]
#[serde(untagged)]
#[non_exhaustive]
pub enum PropertyValue {
    #[serde(rename = "title")]
    Title(TitleValue),
    #[serde(rename = "rich_text")]
    RichText(RichTextValue),
    #[serde(rename = "date")]
    Date(DateValue),
    #[serde(rename = "files")]
    Files,
    #[serde(rename = "checkbox")]
    CheckBox,
    #[serde(rename = "url")]
    Url,
    #[serde(rename = "email")]
    Email,
    #[serde(rename = "phone_number")]
    PhoneNumber,
    #[serde(rename = "formula")]
    Formula,
    #[serde(rename = "relation")]
    Relation,
    #[serde(rename = "Rollup")]
    Rollup,
    #[serde(rename = "number")]
    Number(NumberValue),
    #[serde(rename = "select")]
    Select(SelectValue),
    #[serde(rename = "multi_select")]
    MultiSelect(MultiSelectValue),
    #[serde(rename = "people")]
    People(PeopleProperty),
    #[serde(rename = "created_time")]
    CreatedTime(CreatedTimeValue),
    #[serde(rename = "last_edited_by")]
    LastEditedBy(LastEditedByProperty),
    #[serde(rename = "created_by")]
    CreatedBy(CreatedByProperty),
    #[serde(rename = "last_edited_time")]
    LastEditedTime(LastEditedTimeProperty),
    #[serde(rename = "status")]
    Status(StatusValue),
    #[default]
    #[serde(skip_deserializing)]
    Null
}


#[cfg(test)]
mod serialization {
    use crate::api::{block::*};

    use super::{*};
    use usiem::serde_json::{self, json};

    #[test]
    fn should_deserialize_page_element() {
        let _database_obj : PageElement = serde_json::from_value(json!({
            "parent": { "database_id": "d9824bdc84454327be8b5b47500af6ce" },
            "properties": {
                "Name": {
                    "title": [
                        {
                            "text": {
                                "content": "Tuscan Kale"
                            }
                        }
                    ]
                },
                "Price": { "number": 2.5 },
                "Food group": {
                    "select": {
                        "name": "Vegetable"
                    }
                },
                "Description": {
                    "rich_text": [
                        {
                            "text": {
                                "content": "A dark green leafy vegetable"
                            }
                        }
                    ]
                },
            },
            "children" : []
          })).unwrap();
    }

    #[test]
    fn check_serialization() {
        let mut properties = BTreeMap::new();
    properties.insert("Name".to_owned(), PropertyValue::Title(TitleValue::new("Testing Component")));
    properties.insert("Priority".to_owned(), PropertyValue::Select(SelectValue{
        select : SelectValueInternal {
            name : "Critical".to_owned()
        }
    }));
    properties.insert("MITRE".to_owned(), PropertyValue::MultiSelect(MultiSelectValue {
        multi_select : vec![MultiSelectValueInternal {
            name : "T1548.001 ".to_owned()
        }]
    }));
    let children = vec![
        BlockElement::HeadingOne(RichTextValue {
            rich_text: vec![RichTextObject::new("This is the content of an alert."), RichTextObject::new("In multiple lines?")]
        }),
        BlockElement::HeadingOne(RichTextValue {
            rich_text: vec![RichTextObject::new("In multiple lines"), RichTextObject::new("In multiple lines?")]
        })
    ];
    let new_page : PageElement = PageElement { parent: DatabaseParent {
        database_id : String::new()
    }, properties, children};

    let page_obj = serde_json::to_string(&new_page).unwrap();
    println!("{}", page_obj)
    }
}
