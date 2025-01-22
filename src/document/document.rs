use std::fmt::Display;

use crate::element::Element;
use crate::tag::Tags;

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

#[derive(Debug)]
pub struct Document {
    pub doctype: Doctype,
    pub tags: Vec<Element>,
}
impl Display for Document {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let doctype = Element::open_tag(&Tags::Doctype, &Doctype::Html.to_string());
        let result: String = self.tags.iter().map(|tag| tag.to_string()).collect();
        write!(f, "{}{}", doctype, result)
    }
}
