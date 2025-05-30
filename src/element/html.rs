use std::fmt::Display;

use crate::attribute::Attrs;
use crate::tag::Tag;

#[derive(Debug, Clone)]
pub struct ElementBuilder {
    pub tag: Tag,
    pub attrs: Option<Attrs>,
    pub text: Option<String>,
    pub content: Option<String>,
    pub children: Option<Vec<Element>>,
}

impl ElementBuilder {
    pub fn new(tag: Tag) -> Self {
        ElementBuilder {
            tag,
            attrs: None,
            text: None,
            content: None,
            children: None,
        }
    }

    // Tag attributes
    pub fn attrs(mut self, attrs: Attrs) -> Self {
        self.attrs = Some(attrs);
        self
    }

    // Text within a tag
    pub fn text(mut self, text: impl Into<String>) -> Self {
        self.text = Some(text.into());
        self
    }

    // Content within a opening and closing tag
    pub fn content(mut self, content: impl Into<String>) -> Self {
        self.content = Some(content.into());
        self
    }

    // Nested tag within a tag
    pub fn children(mut self, children: Vec<Element>) -> Self {
        self.children = Some(children);
        self
    }

    pub fn build(self) -> Element {
        Element {
            tag: self.tag,
            attrs: self.attrs,
            text: self.text,
            content: self.content,
            children: self.children,
        }
    }
}

// blah
#[derive(Debug, Clone)]
pub struct Element {
    pub tag: Tag,
    pub attrs: Option<Attrs>,
    pub text: Option<String>,
    pub content: Option<String>,
    pub children: Option<Vec<Element>>,
}
impl Display for Element {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let fmt_str = self.make_tag();
        write!(f, "{fmt_str}")
    }
}

impl Element {
    // Create a HTML opening tag.
    pub fn open_tag(tag: &Tag, value: impl Into<String>) -> String {
        let value = value.into();
        match tag {
            Tag::Comment => format!("<!-- {} -->", value),
            _ => {
                // let value = value.into();
                if !value.is_empty() {
                    format!("<{} {}>", tag, value)
                } else {
                    format!("<{}>", tag)
                }
            }
        }
    }
    // Create a HTML closing tag.
    pub fn close_tag(tag: &Tag) -> String {
        match tag {
            // Void tags are self closing.
            Tag::Doctype | Tag::Meta | Tag::Comment | Tag::Br | Tag::Img => "".to_string(),
            // All other tags have a corresponding closing tag
            _ => format!("</{}>", tag),
        }
    }
    pub fn make_tag(&self) -> String {
        let content = self
            .content
            .as_ref()
            .map_or(String::new(), |content| content.to_string());

        let children: String = self.children.as_ref().map_or(String::new(), |children| {
            children.iter().map(|child| child.to_string()).collect()
        });

        let attributes =
            if let Some(attrs) = self.attrs.as_ref() { attrs.get_attrs(&self.tag) } else { vec![] };

        let open_tag = if !attributes.is_empty() {
            Element::open_tag(&self.tag, attributes.join(" "))
        } else if self.text.is_some() {
            Element::open_tag(&self.tag, self.text.as_ref().unwrap())
        } else {
            Element::open_tag(&self.tag, "")
        };

        let close_tag = Element::close_tag(&self.tag);

        format!(r#"{open_tag}{content}{children}{close_tag}"#)
    }
}

// ...existing code...

#[cfg(test)]
mod tests {
    use super::*;
    use crate::prelude::*;

    #[test]
    fn test_element_builder() {
        let element = ElementBuilder::new(Tag::Div)
            .attrs(Attrs::new().id("test-id").build())
            .text("Hello")
            .build();

        assert_eq!(element.tag, Tag::Div);
        assert!(element.attrs.is_some());
        assert_eq!(element.text.unwrap(), "Hello");
    }

    #[test]
    fn test_void_elements() {
        let br = Element {
            tag: Tag::Br,
            attrs: None,
            text: None,
            content: None,
            children: None,
        };
        assert_eq!(br.to_string(), "<br>");

        let img = ElementBuilder::new(Tag::Img)
            .attrs(Attrs::new().id("test-img").build())
            .build();
        assert_eq!(img.to_string(), r#"<img id="test-img">"#);
    }

    #[test]
    fn test_nested_elements() {
        let child = ElementBuilder::new(Tag::Span).content("child").build();

        let parent = ElementBuilder::new(Tag::Div).children(vec![child]).build();

        assert_eq!(parent.to_string(), "<div><span>child</span></div>");
    }

    #[test]
    fn test_element_with_attributes() {
        let element = ElementBuilder::new(Tag::Div)
            .attrs(Attrs::new().id("test-id").class(vec!["test-class"]).build())
            .build();

        assert!(element.to_string().contains(r#"id="test-id""#));
        assert!(element.to_string().contains(r#"class="test-class""#));
    }

    #[test]
    fn test_comment_element() {
        let comment = ElementBuilder::new(Tag::Comment)
            .text("This is a comment")
            .build();

        assert_eq!(comment.to_string(), "<!-- This is a comment -->");
    }

    #[test]
    fn test_element_with_content() {
        let element = ElementBuilder::new(Tag::P).content("Hello, world!").build();

        assert_eq!(element.to_string(), "<p>Hello, world!</p>");
    }

    #[test]
    fn test_complex_nesting() {
        let inner_span = ElementBuilder::new(Tag::Span).content("inner text").build();

        let div = ElementBuilder::new(Tag::Div)
            .attrs(Attrs::new().id("outer").class(vec!["test-class"]).build())
            .children(vec![inner_span])
            .build();

        assert_eq!(
            div.to_string(),
            r#"<div class="test-class" id="outer"><span>inner text</span></div>"#
        );
    }
}
