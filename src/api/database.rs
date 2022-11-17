use std::collections::BTreeMap;

use serde::{Deserialize, Serialize};

#[path ="./properties.rs"]
pub (crate) mod properties;
use properties::*;

#[derive(Default, Debug, Deserialize, Serialize, Hash)]
#[serde(tag = "object", rename="database")]
pub struct DatabaseDefinition {
    pub id : String,
    pub created_time : String,
    pub last_edited_time : String,
    pub properties: BTreeMap<String, PropertyDefinition>
}



#[derive(Default, Debug, Deserialize, Serialize, Hash)]
#[serde(tag = "type")]
#[non_exhaustive]
pub enum PropertyDefinition {
    #[serde(rename = "title")]
    Title(TitleProperty),
    #[serde(rename = "rich_text")]
    RichText(RichTextValue),
    #[serde(rename = "date")]
    Date(DateProperty),
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
    Number(NumberProperty),
    #[serde(rename = "select")]
    Select(SelectProperty),
    #[serde(rename = "multi_select")]
    MultiSelect(MultiSelectProperty),
    #[serde(rename = "people")]
    People(PeopleProperty),
    #[serde(rename = "created_time")]
    CreatedTime(CreatedTimeProperty),
    #[serde(rename = "last_edited_by")]
    LastEditedBy(LastEditedByProperty),
    #[serde(rename = "created_by")]
    CreatedBy(CreatedByProperty),
    #[serde(rename = "last_edited_time")]
    LastEditedTime(LastEditedTimeProperty),
    #[serde(rename = "status")]
    Status(StatusProperty),
    #[default]
    #[serde(skip_deserializing)]
    Null
}


#[cfg(test)]
mod serialization {
    use std::{hash::{Hash, Hasher}, collections::hash_map::DefaultHasher};

    use super::DatabaseDefinition;
    use usiem::serde_json::{self, json};

    #[test]
    fn should_deserialize_database() {
        let _database_obj : DatabaseDefinition = serde_json::from_value(json!({
            "object": "database",
            "id" : "1234",
            "created_time" : "1234",
            "last_edited_time" : "1234",
            "properties": {
              "Grocery item": {
                "id": "fy:{",
                "type": "title",
                "title": {}
              },
              "Price": {
                "id": "dia[",
                "type": "number",
                "number": {
                  "format": "dollar"
                }
              },
              "Last ordered": {
                "id": "]\\R[",
                "type": "date",
                "date": {}
              },
              "something" : {
                "id" : "123",
                "type" : "files",
                "files" : {
                    "something_random" : ["1234"]
                }
              }
            }
          })).unwrap();
    }

    #[test]
    fn should_be_different_database_hashes() {
        let database_obj1 : DatabaseDefinition = serde_json::from_value(json!({
            "object": "database",
            "id" : "1234",
            "created_time" : "1234",
            "last_edited_time" : "1234",
            "properties": {
              "Grocery item": {
                "id": "fy:{",
                "type": "title",
                "title": {}
              },
              "Price": {
                "id": "dia[",
                "type": "number",
                "number": {
                  "format": "dollar"
                }
              },
              "Last ordered": {
                "id": "]\\R[",
                "type": "date",
                "date": {}
              },
              "something" : {
                "id" : "123",
                "type" : "files",
                "files" : {
                    "something_random" : ["1234"]
                }
              }
            }
          })).unwrap();

          let database_obj2 : DatabaseDefinition = serde_json::from_value(json!({
            "object": "database",
            "id" : "1234",
            "created_time" : "1234",
            "last_edited_time" : "1234",
            "properties": {
              "Grocery item": {
                "id": "fy:{",
                "type": "title",
                "title": {}
              },
              "Price": {
                "id": "dia[",
                "type": "number",
                "number": {
                  "format": "dollar"
                }
              },
              "Last ordered": {
                "id": "]\\R[",
                "type": "date",
                "date": {}
              },
              "something2" : {
                "id" : "123",
                "type" : "files",
                "files" : {
                    "something_random" : ["1234"]
                }
              }
            }
          })).unwrap();
          let mut hasher1 = DefaultHasher::new();
          database_obj1.hash(&mut hasher1);
          let mut hasher2 = DefaultHasher::new();
          database_obj2.hash(&mut hasher2);
          assert_ne!(hasher1.finish(), hasher2.finish());
    }

}