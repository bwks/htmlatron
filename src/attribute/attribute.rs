use std::fmt::Display;

use super::{Alt, Charset, Href, Id, Lang, Rel, Src, Width};

#[derive(Debug, Clone, PartialEq)]
pub enum Attr {
    Alt,
    Charset,
    Class,
    Href,
    Id,
    Lang,
    Src,
    Rel,
    Width,
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
            Attr::Width => write!(f, "width"),
        }
    }
}

#[derive(Debug, Clone)]
pub struct Class(Vec<String>);

impl Display for Class {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Class(values) => {
                write!(f, r#"{}="{}""#, Attr::Class, values.join(" "))
            }
        }
    }
}

#[derive(Debug, Default, Clone)]
pub struct Attrs {
    // Global
    pub id: Option<Id>,
    pub class: Option<Class>,
    pub lang: Option<Lang>,
    pub charset: Option<Charset>,
    pub src: Option<Src>,
    pub alt: Option<Alt>,
    pub href: Option<Href>,
    pub rel: Option<Rel>,
    pub width: Option<Width>,
}

impl Attrs {
    pub fn new() -> AttrsBuilder {
        AttrsBuilder::new()
    }
    pub fn get_attrs(&self) -> Vec<String> {
        let mut attributes = vec![];
        if self.alt.is_some() {
            attributes.push(self.alt.as_ref().unwrap().to_string())
        }
        if self.charset.is_some() {
            attributes.push(self.charset.as_ref().unwrap().to_string())
        }
        if self.class.is_some() {
            attributes.push(self.class.as_ref().unwrap().to_string())
        }
        if self.href.is_some() {
            attributes.push(self.href.as_ref().unwrap().to_string())
        }
        if self.id.is_some() {
            attributes.push(self.id.as_ref().unwrap().to_string())
        }
        if self.lang.is_some() {
            attributes.push(self.lang.as_ref().unwrap().to_string())
        }
        if self.src.is_some() {
            attributes.push(self.src.as_ref().unwrap().to_string())
        }
        if self.rel.is_some() {
            attributes.push(self.rel.as_ref().unwrap().to_string())
        }
        if self.width.is_some() {
            attributes.push(self.width.as_ref().unwrap().to_string())
        }
        attributes
    }
}

pub struct AttrsBuilder {
    pub id: Option<Id>,
    pub class: Option<Class>,
    pub lang: Option<Lang>,
    pub charset: Option<Charset>,
    pub src: Option<Src>,
    pub alt: Option<Alt>,
    pub href: Option<Href>,
    pub rel: Option<Rel>,
    pub width: Option<Width>,
}

impl AttrsBuilder {
    pub fn new() -> Self {
        AttrsBuilder {
            id: None,
            class: None,
            lang: None,
            charset: None,
            src: None,
            alt: None,
            href: None,
            rel: None,
            width: None,
        }
    }

    // region:    ===== Global Attributes ===== //
    pub fn id(mut self, id: &str) -> Self {
        self.id = Some(Id(id.to_owned()));
        self
    }

    pub fn class(mut self, class: Vec<&str>) -> Self {
        let string_vec: Vec<String> = class
            .iter()
            .map(|&s| s.to_string()) // Convert each &str to String
            .collect();
        self.class = Some(Class(string_vec));
        self
    }

    pub fn lang(mut self, lang: &str) -> Self {
        self.lang = Some(Lang(lang.to_owned()));
        self
    }
    // endregion: ===== Global Attributes ===== //

    // meta
    pub fn charset(mut self, charset: &str) -> Self {
        self.charset = Some(Charset(charset.to_owned()));
        self
    }

    // region:    ===== img tag attributes ===== //
    pub fn src(mut self, src: &str) -> Self {
        self.src = Some(Src(src.to_owned()));
        self
    }

    pub fn alt(mut self, alt: &str) -> Self {
        self.alt = Some(Alt(alt.to_owned()));
        self
    }
    // endregion: ===== img tag attributes ===== //

    pub fn href(mut self, href: &str) -> Self {
        self.href = Some(Href(href.to_owned()));
        self
    }

    pub fn rel(mut self, rel: &str) -> Self {
        self.rel = Some(Rel(rel.to_owned()));
        self
    }

    pub fn width(mut self, width: &str) -> Self {
        self.width = Some(Width(width.to_owned()));
        self
    }

    pub fn build(self) -> Attrs {
        Attrs {
            id: self.id,
            class: self.class,
            lang: self.lang,
            charset: self.charset,
            src: self.src,
            alt: self.alt,
            href: self.href,
            rel: self.rel,
            width: self.width,
        }
    }
}
