use serde::{Deserialize, Serialize};


#[derive(Default, Debug, Deserialize, Serialize, Hash)]
pub struct StatusProperty {
    pub id : String,
    pub status : StatusInternal
}

#[derive(Default, Debug, Deserialize, Serialize, Hash)]
pub struct StatusInternal {
    pub options : Vec<StatusOption>,
    pub groups : Vec<StatusGroup>
}
#[derive(Default, Debug, Deserialize, Serialize, Hash)]
pub struct StatusValue {
    pub name : String,
}

#[derive(Default, Debug, Deserialize, Serialize, Hash)]
pub struct StatusOption {
    pub id : String,
    pub name : String,
    pub color : String
}

#[derive(Default, Debug, Deserialize, Serialize, Hash)]
pub struct StatusGroup {
    pub id : String,
    pub name : String,
    pub color : String,
    pub option_ids : Vec<String>
}

#[derive(Default, Debug, Deserialize, Serialize, Hash)]
pub struct CreatedTimeProperty {
    pub id : String,
    pub created_time : CreatedTimeInternal
}

#[derive(Default, Debug, Deserialize, Serialize, Hash)]
pub struct CreatedTimeInternal {

}

#[derive(Default, Debug, Deserialize, Serialize, Hash)]
pub struct CreatedTimeValue {
    pub created_time : String
}

#[derive(Default, Debug, Deserialize, Serialize, Hash)]
pub struct CreatedByProperty {
    pub id : String,
    pub created_by : CreatedByInternal
}

#[derive(Default, Debug, Deserialize, Serialize, Hash)]
pub struct CreatedByInternal {

}

#[derive(Default, Debug, Deserialize, Serialize, Hash)]
pub struct LastEditedByProperty {
    pub id : String,
    pub last_edited_by : LastEditedByInternal
}

#[derive(Default, Debug, Deserialize, Serialize, Hash)]
pub struct LastEditedByInternal {

}

#[derive(Default, Debug, Deserialize, Serialize, Hash)]
pub struct LastEditedTimeProperty {
    pub id : String,
    pub last_edited_time : LastEditedTimeInternal
}

#[derive(Default, Debug, Deserialize, Serialize, Hash)]
pub struct LastEditedTimeInternal {

}


#[derive(Default, Debug, Deserialize, Serialize, Hash)]
pub struct SelectProperty {
    pub id : String,
    pub select : SelectInternal
}

#[derive(Default, Debug, Deserialize, Serialize, Hash)]
pub struct SelectInternal {

}

#[derive(Default, Debug, Deserialize, Serialize)]
pub struct SelectValue {
    pub select : SelectValueInternal
}

#[derive(Default, Debug, Deserialize, Serialize)]
pub struct SelectValueInternal {
    pub name : String
}


#[derive(Default, Debug, Deserialize, Serialize, Hash)]
pub struct PeopleProperty {
    pub id : String,
    pub people : PeopleInternal
}

#[derive(Default, Debug, Deserialize, Serialize, Hash)]
pub struct PeopleInternal {

}


#[derive(Default, Debug, Deserialize, Serialize, Hash)]
pub struct MultiSelectProperty {
    pub id : String,
    pub multi_select : MultiSelectInternal
}

#[derive(Default, Debug, Deserialize, Serialize, Hash)]
pub struct MultiSelectInternal {

}

#[derive(Default, Debug, Deserialize, Serialize)]
pub struct MultiSelectValue {
    pub multi_select : Vec<MultiSelectValueInternal>
}

#[derive(Default, Debug, Deserialize, Serialize)]
pub struct MultiSelectValueInternal {
    pub name : String
}

#[derive(Default, Debug, Deserialize, Serialize)]
pub struct DateValue {
    pub date : DateValueInternal
}

#[derive(Default, Debug, Deserialize, Serialize)]
pub struct DateValueInternal {
    pub start : String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub end : Option<String>
}
impl DateValue {
    pub fn new(date : String) -> Self {
        Self {
            date : DateValueInternal { 
                start : date,
                end : None
            }
        }
    }
}

#[derive(Default, Debug, Deserialize, Serialize, Hash)]
pub struct NumberProperty {
    pub id : String,
    pub number : NumberInternal
}

#[derive(Default, Debug, Deserialize, Serialize, Hash)]
pub struct NumberInternal {
    pub format : String
}

#[derive(Default, Debug, Deserialize, Serialize)]
pub struct NumberValue {
    pub number : f64
}

#[derive(Default, Debug, Deserialize, Serialize, Hash)]
pub struct TitleProperty {
    pub id : String,
    pub title : TitleInternal
}

#[derive(Default, Debug, Deserialize, Serialize, Hash)]
pub struct TitleInternal {
}

#[derive(Default, Debug, Deserialize, Serialize, Hash)]
pub struct TitleValue {
    pub title : Vec<RichTextObject>
}

impl TitleValue {
    pub fn new(content : &str) -> Self  {
        Self {
            title : vec![RichTextObject {
                text : TextValueInternal { content: content.to_owned() },
                anotations : None
            }]
        }
    }
}


#[derive(Default, Debug, Deserialize, Serialize, Hash)]
pub struct RichTextValue {
    pub rich_text : Vec<RichTextObject>
}


// We only support Text types
#[derive(Default, Debug, Deserialize, Serialize, Hash)]
pub struct RichTextObject {
    pub text : TextValueInternal,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub anotations : Option<RichTextAnotation>
}

#[derive(Default, Debug, Deserialize, Serialize, Hash)]
pub struct RichTextAnotation {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub italic : Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bold : Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub color : Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub strikethrough : Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub underline : Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub code : Option<bool>
}

impl RichTextObject {
    pub fn new(content : &str) -> Self {
        let mut slf = Self::default();
        slf.text.content = content.to_owned();
        slf
    }
    pub fn new_owned(content : String) -> Self {
        let mut slf = Self::default();
        slf.text.content = content;
        slf
    }
    pub fn bold(content : &str) -> Self {
        let mut slf = Self::default();
        slf.text.content = content.to_owned();
        let mut anotation = RichTextAnotation::default();
        anotation.bold = Some(true);
        slf.anotations = Some(anotation);
        slf
    }
    pub fn code(content : &str) -> Self {
        let mut slf = Self::default();
        slf.text.content = content.to_owned();
        let mut anotation = RichTextAnotation::default();
        anotation.code = Some(true);
        slf.anotations = Some(anotation);
        slf
    }
}

#[derive(Default, Debug, Deserialize, Serialize, Hash)]
pub struct TextValueInternal {
    pub content : String
}


#[derive(Default, Debug, Deserialize, Serialize, Hash)]
pub struct DateProperty {
    pub id : String,
    pub date : DateInternal
}

#[derive(Default, Debug, Deserialize, Serialize, Hash)]
pub struct DateInternal {
}