use crate::attribute::Attrs;
use crate::element::{Element, ElementBuilder};
use crate::tag::Tags;

#[macro_export]
macro_rules! html_element {
    ($($name:ident => $tag:expr),*) => {
        $(
            #[derive(Debug, Clone)]
            pub struct $name {
                pub attr: Option<Attrs>,
                pub text: Option<&'static str>,
                pub content: Option<&'static str>,
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
    Comment => Tags::Comment,
    Div => Tags::Div,
    Footer => Tags::Footer,
    H1 => Tags::H1,
    H2 => Tags::H2,
    H3 => Tags::H3,
    H4 => Tags::H4,
    H5 => Tags::H5,
    H6 => Tags::H6,
    Header => Tags::Header,
    Html => Tags::Html,
    Img => Tags::Img,
    Li => Tags::Li,
    Link => Tags::Link,
    Meta => Tags::Meta,
    Ol => Tags::Ol,
    P => Tags::P,
    Script => Tags::Script,
    Title => Tags::Title,
    Ul => Tags::Ul
}
