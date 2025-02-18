use std::fmt::Display;

use crate::element::Element;
use crate::tag::Tag;

#[derive(Debug, Default, Clone, PartialEq)]
pub enum Doctype {
    #[default]
    Html,
}
impl Display for Doctype {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Doctype::Html => write!(f, "html"),
        }
    }
}

#[derive(Debug, Clone)]
pub struct Document {
    pub doctype: Doctype,
    pub elements: Vec<Element>,
}
impl Display for Document {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let doctype = Element::open_tag(&Tag::Doctype, Doctype::Html.to_string());
        let result: String = self.elements.iter().map(|tag| tag.to_string()).collect();
        write!(f, "{}{}", doctype, result)
    }
}
