#![allow(dead_code)]

use std::fmt::Display;

#[derive(Debug)]
struct Document {
    tags: Vec<Tag>,
}
impl Display for Document {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let result: String = self.tags.iter().map(|tag| tag.to_string()).collect();
        write!(f, "{}", result)
    }
}

#[derive(Debug, PartialEq)]
enum Tags {
    Doctype,
    Comment,
    Html,
    Header,
    Footer,
    Body,
    Meta,
    Div,
    H1,
    H2,
    H3,
    P,
}
impl Display for Tags {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Tags::Doctype => write!(f, "!DOCTYPE"),
            Tags::Comment => write!(f, "!--"),
            Tags::Html => write!(f, "html"),
            Tags::Header => write!(f, "header"),
            Tags::Footer => write!(f, "footer"),
            Tags::Body => write!(f, "body"),
            Tags::Meta => write!(f, "meta"),
            Tags::Div => write!(f, "div"),
            Tags::H1 => write!(f, "h1"),
            Tags::H2 => write!(f, "h2"),
            Tags::H3 => write!(f, "h3"),
            Tags::P => write!(f, "p"),
        }
    }
}
impl Tags {
    fn close_tag(&self) -> String {
        match self {
            Tags::Doctype | Tags::Meta | Tags::Comment => "".to_string(),
            _ => format!("</{}>", self),
        }
    }
}

#[derive(Debug)]
struct TagBuilder {
    tag: Tags,
    id: Option<Id>,
    class: Option<Class>,
    lang: Option<Lang>,
    charset: Option<Charset>,
    text: Option<String>,
    content: Option<String>,
    children: Option<Vec<Tag>>,
}

impl TagBuilder {
    fn new(tag: Tags) -> Self {
        TagBuilder {
            tag,
            id: None,
            class: None,
            lang: None,
            charset: None,
            text: None,
            content: None,
            children: None,
        }
    }

    fn id(mut self, id: Id) -> Self {
        self.id = Some(id);
        self
    }

    fn class(mut self, class: Class) -> Self {
        self.class = Some(class);
        self
    }

    fn lang(mut self, lang: Lang) -> Self {
        self.lang = Some(lang);
        self
    }

    fn charset(mut self, charset: Charset) -> Self {
        self.charset = Some(charset);
        self
    }

    fn text(mut self, text: String) -> Self {
        self.text = Some(text);
        self
    }

    fn content(mut self, content: String) -> Self {
        self.content = Some(content);
        self
    }

    fn children(mut self, children: Vec<Tag>) -> Self {
        self.children = Some(children);
        self
    }

    fn build(self) -> Tag {
        Tag {
            tag: self.tag,
            id: self.id,
            class: self.class,
            lang: self.lang,
            charset: self.charset,
            text: self.text,
            content: self.content,
            children: self.children,
        }
    }
}

// blah
#[derive(Debug)]
struct Tag {
    tag: Tags,
    id: Option<Id>,
    class: Option<Class>,
    lang: Option<Lang>,
    charset: Option<Charset>,
    text: Option<String>,
    content: Option<String>,
    children: Option<Vec<Tag>>,
}
impl Display for Tag {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let tag = self.tag.to_string();

        let content = self
            .content
            .as_ref()
            .map_or(String::new(), |content| content.to_string());

        let children: String = self.children.as_ref().map_or(String::new(), |children| {
            children.iter().map(|child| child.to_string()).collect()
        });

        let mut open_tag = String::new();
        open_tag.push_str(&tag);
        if self.id.is_some() {
            open_tag.push_str(" ");
            open_tag.push_str(&self.id.as_ref().unwrap().to_string());
        }
        if self.class.is_some() {
            open_tag.push_str(" ");
            open_tag.push_str(&self.class.as_ref().unwrap().to_string());
        }
        if self.lang.is_some() {
            open_tag.push_str(" ");
            open_tag.push_str(&self.lang.as_ref().unwrap().to_string());
        }
        if self.charset.is_some() {
            open_tag.push_str(" ");
            open_tag.push_str(&self.charset.as_ref().unwrap().to_string());
        }
        if self.text.is_some() {
            open_tag.push_str(" ");
            open_tag.push_str(&self.text.as_ref().unwrap());
        }

        // Tag logic
        if self.tag == Tags::Doctype {
            open_tag.push_str(" html");
        }
        if self.tag == Tags::Comment {
            open_tag.push_str(" --");
        }

        let close_tag = self.tag.close_tag();

        write!(
            f,
            r#"
    <{}>
      {}{}
    {}"#,
            open_tag, content, children, close_tag,
        )
    }
}

impl Tag {
    fn doctype() -> TagBuilder {
        TagBuilder::new(Tags::Doctype)
    }
    fn comment() -> TagBuilder {
        TagBuilder::new(Tags::Comment)
    }
    fn html() -> TagBuilder {
        TagBuilder::new(Tags::Html)
    }
    fn header() -> TagBuilder {
        TagBuilder::new(Tags::Header)
    }
    fn footer() -> TagBuilder {
        TagBuilder::new(Tags::Footer)
    }
    fn body() -> TagBuilder {
        TagBuilder::new(Tags::Body)
    }
    fn div() -> TagBuilder {
        TagBuilder::new(Tags::Div)
    }
    fn h1() -> TagBuilder {
        TagBuilder::new(Tags::H1)
    }
    fn p() -> TagBuilder {
        TagBuilder::new(Tags::P)
    }
    fn meta() -> TagBuilder {
        TagBuilder::new(Tags::Meta)
    }
}

#[derive(Debug, Clone)]
struct Id(Vec<String>);

impl Display for Id {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Id(values) => {
                write!(f, r#"id="{}""#, values.join(" "))
            }
        }
    }
}
#[derive(Debug, Clone)]
struct Class(Vec<String>);

impl Display for Class {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Class(values) => {
                write!(f, r#"class="{}""#, values.join(" "))
            }
        }
    }
}

#[derive(Debug, Clone)]
struct Lang(Vec<String>);

impl Display for Lang {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Lang(values) => {
                write!(f, r#"lang="{}""#, values.join(" "))
            }
        }
    }
}

#[derive(Debug, Clone)]
struct Charset(Vec<String>);

impl Display for Charset {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Charset(values) => {
                write!(f, r#"charset="{}""#, values.join(" "))
            }
        }
    }
}

fn main() {
    let id1 = Id(vec!["one".to_string(), "two".to_string()]);
    let id2 = Id(vec!["five".to_string(), "six".to_string()]);
    let class1 = Class(vec!["three".to_string(), "four".to_string()]);
    let class2 = Class(vec!["seven".to_string(), "eight".to_string()]);
    let lang = Lang(vec!["en".to_string()]);
    let meta = Tag::meta()
        .charset(Charset(vec!["utf-8".to_string()]))
        .build();
    let comment = Tag::comment().text("this is a comment".to_string()).build();

    let header = Tag::header().id(Id(vec!["header".to_string()])).build();
    let footer = Tag::footer().id(Id(vec!["footer".to_string()])).build();

    let content = Tag::div()
        .id(id1.clone())
        .class(class1.clone())
        .children(vec![
            //
            Tag::div()
                .id(id2.clone())
                .class(class2.clone())
                .children(vec![
                    Tag::h1().content("Heading".to_owned()).build(),
                    Tag::p().content("blah blah blah".to_owned()).build(),
                ])
                .build(),
        ])
        .build();

    let document = Document {
        tags: vec![
            Tag::doctype().build(),
            comment,
            Tag::html()
                .lang(lang)
                .children(vec![
                    meta,
                    header,
                    Tag::body()
                        .class(Class(vec!["body".to_string()]))
                        .children(vec![content])
                        .build(),
                    footer,
                ])
                .build(),
        ],
    };

    println!("{}", document);
}
