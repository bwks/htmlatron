use std::fmt::Display;

use super::{Alt, Charset, Href, Id, Lang, Rel, Src};

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

#[derive(Debug, Clone)]
pub struct Class(Vec<&'static str>);

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
        if self.rel.is_some() {
            attributes.push(self.rel.as_ref().unwrap().to_string())
        }
        if self.src.is_some() {
            attributes.push(self.src.as_ref().unwrap().to_string())
        }
        attributes
    }
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
