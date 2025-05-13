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
    Em,
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
    Q,
    Script,
    Span,
    Strong,
    Sub,
    Sup,
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
            Tag::Em => write!(f, "!DOCTYPE"),
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
            Tag::Q => write!(f, "q"),
            Tag::Script => write!(f, "script"),
            Tag::Span => write!(f, "span"),
            Tag::Strong => write!(f, "strong"),
            Tag::Sub => write!(f, "sub"),
            Tag::Sup => write!(f, "sup"),
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
                    Attr::Onclick,
                    Attr::Rel,
                    Attr::Target,
                ]);
                attrs
            }
            Tag::Button => {
                let mut attrs = Attr::global().to_vec();
                attrs.extend_from_slice(&[
                    //
                    Attr::Type,
                ]);
                attrs
            }
            _ => Attr::all().to_vec(),
        }
    }
}

// ...existing code...

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_tag_display() {
        assert_eq!(Tag::A.to_string(), "a");
        assert_eq!(Tag::Div.to_string(), "div");
        assert_eq!(Tag::Doctype.to_string(), "!DOCTYPE");
        assert_eq!(Tag::Comment.to_string(), "!--");
    }

    // #[test]
    // fn test_global_attributes() {
    //     let global_attrs = vec![Attr::Az, Attr::Hidden, Attr::Id, Attr::Lang, Attr::Tabindex];

    //     // Test tags that should only have global attributes
    //     let b_attrs = Tag::attributes(&Tag::B);
    //     let body_attrs = Tag::attributes(&Tag::Body);
    //     let code_attrs = Tag::attributes(&Tag::Code);

    //     assert_eq!(b_attrs, global_attrs);
    //     assert_eq!(body_attrs, global_attrs);
    //     assert_eq!(code_attrs, global_attrs);
    // }

    #[test]
    fn test_a_tag_attributes() {
        let mut expected = Attr::global().to_vec();
        expected.extend_from_slice(&[Attr::Href, Attr::Onclick, Attr::Rel, Attr::Target]);

        assert_eq!(Tag::attributes(&Tag::A), expected);
    }

    #[test]
    fn test_button_tag_attributes() {
        let mut expected = Attr::global().to_vec();
        expected.extend_from_slice(&[Attr::Type]);

        assert_eq!(Tag::attributes(&Tag::Button), expected);
    }

    #[test]
    fn test_default_attributes() {
        // Test a tag that falls into the default case
        let all_attrs = Attr::all().to_vec();
        assert_eq!(Tag::attributes(&Tag::Div), all_attrs);
        assert_eq!(Tag::attributes(&Tag::Span), all_attrs);
    }
}
