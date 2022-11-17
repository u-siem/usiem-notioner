use serde::{Deserialize, Serialize};

use super::database::properties::{RichTextValue, RichTextObject};

#[derive(Default, Debug, Deserialize, Serialize)]
#[serde(tag = "object", rename="block")]
pub struct BlockObject {

}

#[derive(Default, Debug, Deserialize, Serialize)]
//#[serde(tag = "type")]
pub enum BlockElement {
    #[serde(rename = "heading_1")]
    HeadingOne(RichTextValue),
    #[serde(rename = "heading_2")]
    HeadingTwo(RichTextValue),
    #[serde(rename = "heading_3")]
    HeadingThree(RichTextValue),
    #[serde(rename = "paragraph")]
    Paragraph(RichTextValue),
    #[serde(rename = "code")]
    Code(CodeBlock),
    #[default]
    #[serde(skip_deserializing)]
    Null
}

impl BlockElement {
    pub fn heading_one(content : &str) -> Self {
        BlockElement::HeadingOne(RichTextValue {
            rich_text: vec![RichTextObject::new(content)]
        })
    }
    pub fn heading_two(content : &str) -> Self {
        BlockElement::HeadingTwo(RichTextValue {
            rich_text: vec![RichTextObject::new(content)]
        })
    }
    pub fn heading_three(content : &str) -> Self {
        BlockElement::HeadingThree(RichTextValue {
            rich_text: vec![RichTextObject::new(content)]
        })
    }
    pub fn code(content : &str, language : Option<String>) -> Self {
        BlockElement::Code(CodeBlock {
            rich_text : vec![RichTextObject::new(content)],
            language : language.unwrap_or(format!("plain text"))
        })
    }
    pub fn code_owned(content : String, language : Option<String>) -> Self {
        BlockElement::Code(CodeBlock {
            rich_text : vec![RichTextObject::new_owned(content)],
            language : language.unwrap_or(format!("plain text"))
        })
    }
}

#[derive(Default, Debug, Deserialize, Serialize, Hash)]
pub struct CodeBlock {
    pub rich_text : Vec<RichTextObject>,
    pub language : String
}
