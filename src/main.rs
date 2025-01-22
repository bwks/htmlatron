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

#[derive(Debug, Clone, PartialEq)]
enum Tags {
    Doctype,
    Comment,
    Html,
    Header,
    Footer,
    Body,
    Meta,
    Title,
    Div,
    H1,
    H2,
    H3,
    P,
    Br,
    Img,
    A,
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
            Tags::Title => write!(f, "title"),
            Tags::Div => write!(f, "div"),
            Tags::H1 => write!(f, "h1"),
            Tags::H2 => write!(f, "h2"),
            Tags::H3 => write!(f, "h3"),
            Tags::P => write!(f, "p"),
            Tags::Br => write!(f, "br"),
            Tags::Img => write!(f, "img"),
            Tags::A => write!(f, "a"),
        }
    }
}
impl Tags {
    fn close_tag(&self) -> String {
        match self {
            Tags::Doctype | Tags::Meta | Tags::Comment | Tags::Br | Tags::Img => "".to_string(),
            _ => format!("</{}>", self),
        }
    }
}

////////////

#[derive(Debug)]
struct TagBuilder {
    tag: Tags,
    attr: Option<Attrs>,
    text: Option<&'static str>,
    content: Option<&'static str>,
    children: Option<Vec<Tag>>,
}

impl TagBuilder {
    fn new(tag: Tags) -> Self {
        TagBuilder {
            tag,
            attr: None,
            text: None,
            content: None,
            children: None,
        }
    }

    // region:    ===== Global Attributes ===== //
    fn attrs(mut self, attr: Attrs) -> Self {
        self.attr = Some(attr);
        self
    }
    // target

    // endregion: ===== a tag attributes ===== //

    // Text within a tag
    fn text(mut self, text: &'static str) -> Self {
        self.text = Some(text);
        self
    }

    // Content within a opening and closing tags
    fn content(mut self, content: &'static str) -> Self {
        self.content = Some(content);
        self
    }

    // Nested tags
    fn children(mut self, children: Vec<Tag>) -> Self {
        self.children = Some(children);
        self
    }

    fn build(self) -> Tag {
        Tag {
            tag: self.tag,
            attr: self.attr,
            text: self.text,
            content: self.content,
            children: self.children,
        }
    }
}

// blah
#[derive(Debug, Clone)]
struct Tag {
    tag: Tags,
    attr: Option<Attrs>,
    text: Option<&'static str>,
    content: Option<&'static str>,
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
        if let Some(attr) = self.attr.as_ref() {
            if attr.id.is_some() {
                open_tag.push_str(" ");
                open_tag.push_str(&attr.id.as_ref().unwrap().to_string());
            }
            if attr.class.is_some() {
                open_tag.push_str(" ");
                open_tag.push_str(&attr.class.as_ref().unwrap().to_string());
            }
            if attr.lang.is_some() {
                open_tag.push_str(" ");
                open_tag.push_str(&attr.lang.as_ref().unwrap().to_string());
            }
            if attr.charset.is_some() {
                open_tag.push_str(" ");
                open_tag.push_str(&attr.charset.as_ref().unwrap().to_string());
            }
            if attr.src.is_some() {
                open_tag.push_str(" ");
                open_tag.push_str(&attr.src.as_ref().unwrap().to_string());
            }
            if attr.alt.is_some() {
                open_tag.push_str(" ");
                open_tag.push_str(&attr.alt.as_ref().unwrap().to_string());
            }
            if attr.href.is_some() {
                open_tag.push_str(" ");
                open_tag.push_str(&attr.href.as_ref().unwrap().to_string());
            }
        }
        if self.text.is_some() {
            open_tag.push_str(" ");
            open_tag.push_str(&self.text.as_ref().unwrap());
        }

        // Tag logic
        if self.tag == Tags::Comment {
            open_tag.push_str(" --");
        }

        let close_tag = self.tag.close_tag();

        write!(
            f,
            r#"<{open_tag}>
  {content}{children}
{close_tag}
"#
        )
    }
}

// Macro version of generating Tag methods
// macro_rules! create_tag_fn {
//     ($($name:ident => $tag:expr),*) => {
//         $(
//             fn $name() -> TagBuilder {
//                 TagBuilder::new($tag)
//             }
//         )*
//     }
// }

// impl Tag {
//     create_tag_fn! {
//         doctype => Tags::Doctype,
//         comment => Tags::Comment,
//         html => Tags::Html,
//         header => Tags::Header,
//         footer => Tags::Footer,
//         body => Tags::Body,
//         div => Tags::Div,
//         h1 => Tags::H1,
//         p => Tags::P,
//         meta => Tags::Meta,
//         title => Tags::Title
//     }
// }
//

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
    fn title() -> TagBuilder {
        TagBuilder::new(Tags::Title)
    }
    fn br() -> TagBuilder {
        TagBuilder::new(Tags::Br)
    }
    fn img() -> TagBuilder {
        TagBuilder::new(Tags::Img)
    }
    fn a() -> TagBuilder {
        TagBuilder::new(Tags::A)
    }
}

////////
#[derive(Debug, Default, Clone)]
struct Attrs {
    // Global
    id: Option<Id>,
    class: Option<Class>,
    lang: Option<Lang>,
    charset: Option<Charset>,
    src: Option<Src>,
    alt: Option<Alt>,
    href: Option<Href>,
}

struct AttrsBuilder {
    id: Option<Id>,
    class: Option<Class>,
    lang: Option<Lang>,
    charset: Option<Charset>,
    src: Option<Src>,
    alt: Option<Alt>,
    href: Option<Href>,
}

impl AttrsBuilder {
    fn new() -> Self {
        AttrsBuilder {
            id: None,
            class: None,
            lang: None,
            charset: None,
            src: None,
            alt: None,
            href: None,
        }
    }

    // region:    ===== Global Attributes ===== //
    fn id(mut self, id: &'static str) -> Self {
        self.id = Some(Id(id));
        self
    }

    fn class(mut self, class: Vec<&'static str>) -> Self {
        self.class = Some(Class(class));
        self
    }

    fn lang(mut self, lang: &'static str) -> Self {
        self.lang = Some(Lang(lang));
        self
    }
    // endregion: ===== Global Attributes ===== //

    // meta
    fn charset(mut self, charset: &'static str) -> Self {
        self.charset = Some(Charset(charset));
        self
    }

    // region:    ===== img tag attributes ===== //
    fn src(mut self, src: &'static str) -> Self {
        self.src = Some(Src(src));
        self
    }

    fn alt(mut self, alt: &'static str) -> Self {
        self.alt = Some(Alt(alt));
        self
    }
    // endregion: ===== img tag attributes ===== //

    // region:    ===== a tag attributes ===== //
    fn href(mut self, href: &'static str) -> Self {
        self.href = Some(Href(href));
        self
    }

    fn build(self) -> Attrs {
        Attrs {
            id: self.id,
            class: self.class,
            lang: self.lang,
            charset: self.charset,
            src: self.src,
            alt: self.alt,
            href: self.href,
        }
    }
}

impl Attrs {
    fn new() -> AttrsBuilder {
        AttrsBuilder::new()
    }
}

fn attrs() -> AttrsBuilder {
    AttrsBuilder::new()
}

#[derive(Debug, Clone)]
struct Id(&'static str);

impl Display for Id {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Id(value) => {
                write!(f, r#"id="{value}""#)
            }
        }
    }
}
#[derive(Debug, Clone)]
struct Class(Vec<&'static str>);

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
struct Lang(&'static str);

impl Display for Lang {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Lang(value) => {
                write!(f, r#"lang="{value}""#)
            }
        }
    }
}

#[derive(Debug, Clone)]
struct Charset(&'static str);

impl Display for Charset {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Charset(value) => {
                write!(f, r#"charset="{value}""#)
            }
        }
    }
}

#[derive(Debug, Clone)]
struct Src(&'static str);

impl Display for Src {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Src(value) => {
                write!(f, r#"src="{}""#, value)
            }
        }
    }
}

#[derive(Debug, Clone)]
struct Alt(&'static str);

impl Display for Alt {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Alt(value) => {
                write!(f, r#"alt="{}""#, value)
            }
        }
    }
}

#[derive(Debug, Clone)]
struct Href(&'static str);

impl Display for Href {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Href(value) => {
                write!(f, r#"href="{}""#, value)
            }
        }
    }
}

fn main() {
    let id1 = "one";
    let id2 = "two";
    let class1 = vec!["three", "four"];
    let class2 = vec!["seven", "eight"];
    let lang = "en";
    let meta = Tag::meta()
        .attrs(Attrs::new().charset("utf-8").build())
        .build();
    let title = Tag::title().content("The is the title").build();
    let comment = Tag::comment().text("this is a comment").build();

    let doctype = Tag::doctype().text("html").build();
    let html = Tag::html().attrs(Attrs::new().lang(lang).build());

    let image = Tag::img()
        .attrs(Attrs::new().src("blah.img").alt("some image").build())
        .build();

    let header = Tag::header()
        .attrs(Attrs::new().id("header").build())
        .children(vec![meta, title])
        .build();

    let footer = Tag::footer()
        .attrs(Attrs::new().id("footer").build())
        .build();

    let line_break = Tag::br().build();

    let content = Tag::div()
        .attrs(Attrs::new().id(id1).class(class1.clone()).build())
        .children(vec![
            //
            image,
            Tag::div()
                .attrs(Attrs::new().id(id2).class(class2.clone()).build())
                .children(vec![
                    Tag::h1().content("Heading").build(),
                    Tag::p().content("blah blah blah").build(),
                    Tag::a()
                        .attrs(Attrs::new().href("http://stuff.things").build())
                        .build(),
                ])
                .build(),
        ])
        .build();

    let body = Tag::body()
        .attrs(Attrs::new().class(vec!["body"]).build())
        .children(vec![content])
        .build();

    let document = Document {
        tags: vec![
            doctype,
            comment,
            html.children(vec![
                header,
                body,
                footer,
                line_break.clone(),
                line_break.clone(),
                line_break.clone(),
            ])
            .build(),
        ],
    };

    println!("{}", document);
}
