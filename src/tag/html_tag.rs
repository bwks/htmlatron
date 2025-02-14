use crate::attribute::Attr;
use std::fmt::Display;

#[derive(Debug, Clone, PartialEq)]
pub enum Tag {
    A,
    B,
    Body,
    Br,
    Button,
    Code,
    Comment,
    Div,
    Doctype,
    Footer,
    H1,
    H2,
    H3,
    H4,
    H5,
    H6,
    Head,
    Header,
    Html,
    I,
    Input,
    Img,
    Kbd,
    Label,
    Li,
    Link,
    Meta,
    Nav,
    Ol,
    P,
    Pre,
    Script,
    Span,
    Table,
    Thead,
    Tbody,
    Th,
    Tr,
    Td,
    Title,
    Ul,
}
impl Display for Tag {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Tag::A => write!(f, "a"),
            Tag::B => write!(f, "b"),
            Tag::Body => write!(f, "body"),
            Tag::Br => write!(f, "br"),
            Tag::Button => write!(f, "button"),
            Tag::Code => write!(f, "code"),
            Tag::Comment => write!(f, "!--"),
            Tag::Div => write!(f, "div"),
            Tag::Doctype => write!(f, "!DOCTYPE"),
            Tag::Footer => write!(f, "footer"),
            Tag::H1 => write!(f, "h1"),
            Tag::H2 => write!(f, "h2"),
            Tag::H3 => write!(f, "h3"),
            Tag::H4 => write!(f, "h4"),
            Tag::H5 => write!(f, "h5"),
            Tag::H6 => write!(f, "h6"),
            Tag::Head => write!(f, "head"),
            Tag::Header => write!(f, "header"),
            Tag::Html => write!(f, "html"),
            Tag::Input => write!(f, "input"),
            Tag::I => write!(f, "i"),
            Tag::Img => write!(f, "img"),
            Tag::Kbd => write!(f, "kbd"),
            Tag::Label => write!(f, "label"),
            Tag::Li => write!(f, "li"),
            Tag::Link => write!(f, "link"),
            Tag::Meta => write!(f, "meta"),
            Tag::Nav => write!(f, "nav"),
            Tag::Ol => write!(f, "ol"),
            Tag::P => write!(f, "p"),
            Tag::Pre => write!(f, "pre"),
            Tag::Script => write!(f, "script"),
            Tag::Span => write!(f, "span"),
            Tag::Table => write!(f, "table"),
            Tag::Thead => write!(f, "thead"),
            Tag::Tbody => write!(f, "tbody"),
            Tag::Th => write!(f, "th"),
            Tag::Tr => write!(f, "tr"),
            Tag::Td => write!(f, "td"),
            Tag::Title => write!(f, "title"),
            Tag::Ul => write!(f, "ul"),
        }
    }
}
impl Tag {
    pub fn attributes(tag: &Tag) -> Vec<Attr> {
        match tag {
            // Elements that support global attributes only.
            Tag::B | Tag::Body | Tag::Code => Attr::global().to_vec(),
            Tag::A => {
                let mut attrs = Attr::global().to_vec();
                attrs.extend_from_slice(&[
                    //
                    Attr::Href,
                    Attr::Target,
                    Attr::Rel,
                ]);
                attrs
            }
            Tag::Button => {
                let mut attrs = Attr::global().to_vec();
                attrs.extend_from_slice(&[
                    //
                    Attr::Type,
                    Attr::Data,
                ]);
                attrs
            }
            _ => Attr::all().to_vec(),
        }
    }
}
