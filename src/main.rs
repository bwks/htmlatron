#![allow(dead_code)]

use std::fmt::Display;

#[derive(Debug, Default, Clone, PartialEq)]
enum Doctype {
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
struct Document {
    doctype: Doctype,
    tags: Vec<Element>,
}
impl Display for Document {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let doctype = Element::open_tag(&Tags::Doctype, &Doctype::Html.to_string());
        let result: String = self.tags.iter().map(|tag| tag.to_string()).collect();
        write!(f, "{}{}", doctype, result)
    }
}

#[derive(Debug, Clone, PartialEq)]
enum Attr {
    Alt,
    Charset,
    Class,
    Href,
    Id,
    Lang,
    Src,
    Rel,
}
impl Display for Attr {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Attr::Alt => write!(f, "alt"),
            Attr::Charset => write!(f, "charset"),
            Attr::Class => write!(f, "class"),
            Attr::Href => write!(f, "href"),
            Attr::Id => write!(f, "id"),
            Attr::Lang => write!(f, "lang"),
            Attr::Src => write!(f, "src"),
            Attr::Rel => write!(f, "rel"),
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
enum Tags {
    A,
    Body,
    Br,
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
    Header,
    Html,
    Img,
    Li,
    Link,
    Meta,
    Ol,
    P,
    Script,
    Title,
    Ul,
}
impl Display for Tags {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Tags::A => write!(f, "a"),
            Tags::Body => write!(f, "body"),
            Tags::Br => write!(f, "br"),
            Tags::Comment => write!(f, "!--"),
            Tags::Div => write!(f, "div"),
            Tags::Doctype => write!(f, "!DOCTYPE"),
            Tags::Footer => write!(f, "footer"),
            Tags::H1 => write!(f, "h1"),
            Tags::H2 => write!(f, "h2"),
            Tags::H3 => write!(f, "h3"),
            Tags::H4 => write!(f, "h4"),
            Tags::H5 => write!(f, "h5"),
            Tags::H6 => write!(f, "h6"),
            Tags::Header => write!(f, "header"),
            Tags::Html => write!(f, "html"),
            Tags::Img => write!(f, "img"),
            Tags::Li => write!(f, "li"),
            Tags::Link => write!(f, "link"),
            Tags::Meta => write!(f, "meta"),
            Tags::Ol => write!(f, "ol"),
            Tags::P => write!(f, "p"),
            Tags::Script => write!(f, "script"),
            Tags::Title => write!(f, "title"),
            Tags::Ul => write!(f, "ul"),
        }
    }
}

#[derive(Debug)]
struct ElementBuilder {
    tag: Tags,
    attr: Option<Attrs>,
    text: Option<&'static str>,
    content: Option<&'static str>,
    children: Option<Vec<Element>>,
}

impl ElementBuilder {
    fn new(tag: Tags) -> Self {
        ElementBuilder {
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
    fn children(mut self, children: Vec<Element>) -> Self {
        self.children = Some(children);
        self
    }

    fn build(self) -> Element {
        Element {
            tag: self.tag,
            attr: self.attr,
            text: self.text,
            content: self.content,
            children: self.children,
        }
    }
}

macro_rules! html_element {
    ($($name:ident => $tag:expr),*) => {
        $(
            #[derive(Debug, Clone)]
            struct $name {
                attr: Option<Attrs>,
                text: Option<&'static str>,
                content: Option<&'static str>,
                children: Option<Vec<Element>>,
            }

            impl $name {
                fn new() -> ElementBuilder {
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

// blah
#[derive(Debug, Clone)]
struct Element {
    tag: Tags,
    attr: Option<Attrs>,
    text: Option<&'static str>,
    content: Option<&'static str>,
    children: Option<Vec<Element>>,
}
impl Display for Element {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let fmt_str = self.make_tag();
        write!(f, "{fmt_str}")
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

impl Element {
    // region:    ===== Utility Methods ===== //

    fn open_tag(tag: &Tags, value: &str) -> String {
        match tag {
            Tags::Comment => format!("<!-- {} -->", value),
            _ => {
                if !value.is_empty() {
                    format!("<{} {}>", tag, value)
                } else {
                    format!("<{}>", tag)
                }
            }
        }
    }
    fn close_tag(tag: &Tags) -> String {
        match tag {
            Tags::Doctype | Tags::Meta | Tags::Comment | Tags::Br | Tags::Img => "".to_string(),
            _ => format!("</{}>", tag),
        }
    }
    fn make_tag(&self) -> String {
        let content = self
            .content
            .as_ref()
            .map_or(String::new(), |content| content.to_string());

        let children: String = self.children.as_ref().map_or(String::new(), |children| {
            children.iter().map(|child| child.to_string()).collect()
        });

        let mut attributes = vec![];
        if let Some(attr) = self.attr.as_ref() {
            if attr.alt.is_some() {
                attributes.push(attr.alt.as_ref().unwrap().to_string())
            }
            if attr.charset.is_some() {
                attributes.push(attr.charset.as_ref().unwrap().to_string())
            }
            if attr.class.is_some() {
                attributes.push(attr.class.as_ref().unwrap().to_string())
            }
            if attr.href.is_some() {
                attributes.push(attr.href.as_ref().unwrap().to_string())
            }
            if attr.id.is_some() {
                attributes.push(attr.id.as_ref().unwrap().to_string())
            }
            if attr.lang.is_some() {
                attributes.push(attr.lang.as_ref().unwrap().to_string())
            }
            if attr.rel.is_some() {
                attributes.push(attr.rel.as_ref().unwrap().to_string())
            }
            if attr.src.is_some() {
                attributes.push(attr.src.as_ref().unwrap().to_string())
            }
        }
        let open_tag = if !attributes.is_empty() {
            Element::open_tag(&self.tag, &attributes.join(" "))
        } else if self.text.is_some() {
            Element::open_tag(&self.tag, self.text.unwrap())
        } else {
            Element::open_tag(&self.tag, &"".to_string())
        };

        let close_tag = Element::close_tag(&self.tag);

        // write!(f, r#"{open_tag}{content}{children}{close_tag}"#)
        let fmt_str = format!(
            r#"
        {open_tag}{content}{children}{close_tag}
        "#
        );
        fmt_str
    }

    // endregion: ===== Utility Methods ===== //
}

//
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
    rel: Option<Rel>,
}

struct AttrsBuilder {
    id: Option<Id>,
    class: Option<Class>,
    lang: Option<Lang>,
    charset: Option<Charset>,
    src: Option<Src>,
    alt: Option<Alt>,
    href: Option<Href>,
    rel: Option<Rel>,
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
            rel: None,
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

    fn href(mut self, href: &'static str) -> Self {
        self.href = Some(Href(href));
        self
    }

    fn rel(mut self, rel: &'static str) -> Self {
        self.rel = Some(Rel(rel));
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
            rel: self.rel,
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
                write!(f, r#"{}="{}""#, Attr::Id, value)
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
                write!(f, r#"{}="{}""#, Attr::Class, values.join(" "))
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
                write!(f, r#"{}="{}""#, Attr::Lang, value)
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
                write!(f, r#"{}="{}""#, Attr::Charset, value)
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
                write!(f, r#"{}="{}""#, Attr::Src, value)
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
                write!(f, r#"{}="{}""#, Attr::Alt, value)
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
                write!(f, r#"{}="{}""#, Attr::Href, value)
            }
        }
    }
}

#[derive(Debug, Clone)]
struct Rel(&'static str);

impl Display for Rel {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Rel(value) => {
                write!(f, r#"{}="{}""#, Attr::Rel, value)
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
    let meta = Meta::new()
        .attrs(Attrs::new().charset("utf-8").build())
        .build();
    let title = Title::new().content("The is the title").build();
    let comment = Comment::new().text("this is a comment").build();

    let html = Html::new().attrs(Attrs::new().lang(lang).build());
    let js = Script::new()
        .attrs(Attrs::new().src("script.js").build())
        .build();
    let css = Link::new()
        .attrs(Attrs::new().rel("stylesheet").href("styles.css").build())
        .build();

    let image = Img::new()
        .attrs(Attrs::new().src("blah.img").alt("some image").build())
        .build();

    let header = Header::new()
        .attrs(Attrs::new().id("header").build())
        .children(vec![meta, css, js, title])
        .build();

    let footer = Footer::new()
        .attrs(Attrs::new().id("footer").build())
        .build();

    let line_break = Br::new().build();

    let ul = Ul::new()
        .children(vec![
            Li::new().build(),
            Li::new().build(),
            Li::new().build(),
        ])
        .build();
    let ol = Ol::new()
        .children(vec![
            Li::new().build(),
            Li::new().build(),
            Li::new().build(),
        ])
        .build();

    let content = Div::new()
        .attrs(Attrs::new().id(id1).class(class1.clone()).build())
        .children(vec![
            //
            image,
            Div::new()
                .attrs(Attrs::new().id(id2).class(class2.clone()).build())
                .children(vec![
                    H1::new().content("Heading 1").build(),
                    P::new()
                        .content("blah blah blah")
                        .children(vec![
                            //
                            A::new()
                                .attrs(Attrs::new().href("http://stuff.things").build())
                                .content("Stuff and Things")
                                .build(),
                        ])
                        .build(),
                    H2::new().content("Heading 2").build(),
                    ul,
                    H3::new().content("Heading 3").build(),
                    ol,
                ])
                .build(),
        ])
        .build();

    let body = Body::new()
        .attrs(Attrs::new().class(vec!["body"]).build())
        .children(vec![content])
        .build();

    let document = Document {
        doctype: Doctype::Html,
        tags: vec![
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
