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

#[derive(Debug)]
struct TagBuilder {
    tag: Tags,
    id: Option<Id>,
    class: Option<Class>,
    lang: Option<Lang>,
    charset: Option<Charset>,
    src: Option<Src>,
    alt: Option<Alt>,
    href: Option<Href>,
    text: Option<&'static str>,
    content: Option<&'static str>,
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
            src: None,
            alt: None,
            href: None,
            text: None,
            content: None,
            children: None,
        }
    }

    // region:    ===== Global Attributes ===== //
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
    // endregion: ===== Global Attributes ===== //

    // meta
    fn charset(mut self, charset: Charset) -> Self {
        self.charset = Some(charset);
        self
    }

    // region:    ===== img tag attributes ===== //
    fn src(mut self, src: Src) -> Self {
        self.src = Some(src);
        self
    }

    fn alt(mut self, alt: Alt) -> Self {
        self.alt = Some(alt);
        self
    }
    // endregion: ===== img tag attributes ===== //

    // region:    ===== a tag attributes ===== //
    fn href(mut self, href: Href) -> Self {
        self.href = Some(href);
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
            id: self.id,
            class: self.class,
            lang: self.lang,
            charset: self.charset,
            src: self.src,
            alt: self.alt,
            href: self.href,
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
    id: Option<Id>,
    class: Option<Class>,
    lang: Option<Lang>,
    charset: Option<Charset>,
    src: Option<Src>,
    alt: Option<Alt>,
    href: Option<Href>,
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
        if self.src.is_some() {
            open_tag.push_str(" ");
            open_tag.push_str(&self.src.as_ref().unwrap().to_string());
        }
        if self.alt.is_some() {
            open_tag.push_str(" ");
            open_tag.push_str(&self.alt.as_ref().unwrap().to_string());
        }
        if self.href.is_some() {
            open_tag.push_str(" ");
            open_tag.push_str(&self.href.as_ref().unwrap().to_string());
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
    let id1 = Id("one");
    let id2 = Id("two");
    let class1 = Class(vec!["three", "four"]);
    let class2 = Class(vec!["seven", "eight"]);
    let lang = Lang("en");
    let meta = Tag::meta().charset(Charset("utf-8")).build();
    let title = Tag::title().content("The is the title").build();
    let comment = Tag::comment().text("this is a comment").build();

    let doctype = Tag::doctype().text("html").build();
    let html = Tag::html().lang(lang);

    let image = Tag::img()
        .src(Src("blah.img"))
        .alt(Alt("some image"))
        .build();

    let header = Tag::header()
        .id(Id("header"))
        .children(vec![meta, title])
        .build();

    let footer = Tag::footer().id(Id("footer")).build();

    let line_break = Tag::br().build();

    let content = Tag::div()
        .id(id1.clone())
        .class(class1.clone())
        .children(vec![
            //
            image,
            Tag::div()
                .id(id2.clone())
                .class(class2.clone())
                .children(vec![
                    Tag::h1().content("Heading").build(),
                    Tag::p().content("blah blah blah").build(),
                    Tag::a().href(Href("http://stuff.things")).build(),
                ])
                .build(),
        ])
        .build();

    let body = Tag::body()
        .class(Class(vec!["body"]))
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
