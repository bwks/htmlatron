use crate::attribute::Attrs;
use crate::element::{Element, ElementBuilder};
use crate::tag::Tags;

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
    A => Tags::A,
    Body => Tags::Body,
    Br => Tags::Br,
    Code => Tags::Code,
    Comment => Tags::Comment,
    Div => Tags::Div,
    Footer => Tags::Footer,
    H1 => Tags::H1,
    H2 => Tags::H2,
    H3 => Tags::H3,
    H4 => Tags::H4,
    H5 => Tags::H5,
    H6 => Tags::H6,
    Head => Tags::Head,
    Header => Tags::Header,
    Html => Tags::Html,
    I => Tags::I,
    Img => Tags::Img,
    Li => Tags::Li,
    Link => Tags::Link,
    Meta => Tags::Meta,
    Ol => Tags::Ol,
    P => Tags::P,
    Pre => Tags::Pre,
    Script => Tags::Script,
    Span => Tags::Span,
    Title => Tags::Title,
    Ul => Tags::Ul
}
