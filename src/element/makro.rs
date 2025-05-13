use crate::attribute::Attrs;
use crate::element::{Element, ElementBuilder};
use crate::tag::Tag;

#[macro_export]
macro_rules! html_element {
    ($($name:ident => $tag:expr),*) => {
        $(
            #[derive(Debug, Clone)]
            pub struct $name {
                pub attrs: Option<Attrs>,
                pub text: Option<String>,
                pub content: Option<String>,
                pub children: Option<Vec<Element>>,
            }

            impl $name {
                pub fn new() -> ElementBuilder {
                    ElementBuilder::new($tag)
                }
            }
        )*
    }
}

html_element! {
    A => Tag::A,
    B => Tag::B,
    Body => Tag::Body,
    Br => Tag::Br,
    Button => Tag::Button,
    Code => Tag::Code,
    Comment => Tag::Comment,
    Div => Tag::Div,
    Em => Tag::Em,
    Footer => Tag::Footer,
    H1 => Tag::H1,
    H2 => Tag::H2,
    H3 => Tag::H3,
    H4 => Tag::H4,
    H5 => Tag::H5,
    H6 => Tag::H6,
    Head => Tag::Head,
    Header => Tag::Header,
    Html => Tag::Html,
    I => Tag::I,
    Input => Tag::Input,
    Img => Tag::Img,
    Kbd => Tag::Kbd,
    Label => Tag::Label,
    Li => Tag::Li,
    Link => Tag::Link,
    Meta => Tag::Meta,
    Nav => Tag::Nav,
    Ol => Tag::Ol,
    P => Tag::P,
    Pre => Tag::Pre,
    Q => Tag::Q,
    Script => Tag::Script,
    Span => Tag::Span,
    Strong => Tag::Strong,
    Sub => Tag::Sub,
    Sup => Tag::Sup,
    Table => Tag::Table,
    Thead => Tag::Thead,
    Tbody => Tag::Tbody,
    Th => Tag::Th,
    Tr => Tag::Tr,
    Td => Tag::Td,
    Title => Tag::Title,
    Ul => Tag::Ul
}
