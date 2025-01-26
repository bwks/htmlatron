use std::fmt::Display;

use super::{Alt, Az, Charset, Href, Id, Lang, Rel, Src, Target, Type, Width};

#[derive(Debug, Clone, PartialEq)]
pub enum Attr {
    Alt,
    Az,
    Charset,
    Class,
    Defer,
    Href,
    Id,
    Lang,
    Src,
    Target,
    Type,
    Rel,
    Width,
}
impl Display for Attr {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Attr::Alt => write!(f, "alt"),
            Attr::Az => write!(f, "as"),
            Attr::Charset => write!(f, "charset"),
            Attr::Class => write!(f, "class"),
            Attr::Defer => write!(f, "defer"),
            Attr::Href => write!(f, "href"),
            Attr::Id => write!(f, "id"),
            Attr::Lang => write!(f, "lang"),
            Attr::Src => write!(f, "src"),
            Attr::Target => write!(f, "target"),
            Attr::Type => write!(f, "type"),
            Attr::Rel => write!(f, "rel"),
            Attr::Width => write!(f, "width"),
        }
    }
}

#[derive(Debug, Clone)]
pub struct Defer;

impl Display for Defer {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", Attr::Defer)
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
    pub alt: Option<Alt>,
    pub az: Option<Az>,
    pub id: Option<Id>,
    pub class: Option<Class>,
    pub charset: Option<Charset>,
    pub defer: Option<Defer>,
    pub href: Option<Href>,
    pub lang: Option<Lang>,
    pub rel: Option<Rel>,
    pub src: Option<Src>,
    pub target: Option<Target>,
    pub tipe: Option<Type>,
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
        if self.az.is_some() {
            attributes.push(self.az.as_ref().unwrap().to_string())
        }
        if self.charset.is_some() {
            attributes.push(self.charset.as_ref().unwrap().to_string())
        }
        if self.class.is_some() {
            attributes.push(self.class.as_ref().unwrap().to_string())
        }
        if self.defer.is_some() {
            attributes.push(self.defer.as_ref().unwrap().to_string())
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
        if self.target.is_some() {
            attributes.push(self.target.as_ref().unwrap().to_string())
        }
        if self.tipe.is_some() {
            attributes.push(self.tipe.as_ref().unwrap().to_string())
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
    pub alt: Option<Alt>,
    pub az: Option<Az>,
    pub charset: Option<Charset>,
    pub class: Option<Class>,
    pub defer: Option<Defer>,
    pub href: Option<Href>,
    pub id: Option<Id>,
    pub lang: Option<Lang>,
    pub rel: Option<Rel>,
    pub src: Option<Src>,
    pub target: Option<Target>,
    pub tipe: Option<Type>,
    pub width: Option<Width>,
}

impl AttrsBuilder {
    pub fn new() -> Self {
        AttrsBuilder {
            alt: None,
            az: None,
            charset: None,
            class: None,
            defer: None,
            href: None,
            id: None,
            lang: None,
            rel: None,
            src: None,
            target: None,
            tipe: None,
            width: None,
        }
    }
    pub fn alt(mut self, alt: &str) -> Self {
        self.alt = Some(Alt(alt.to_owned()));
        self
    }

    pub fn az(mut self, az: &str) -> Self {
        self.az = Some(Az(az.to_owned()));
        self
    }

    pub fn charset(mut self, charset: &str) -> Self {
        self.charset = Some(Charset(charset.to_owned()));
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

    pub fn defer(mut self) -> Self {
        self.defer = Some(Defer);
        self
    }

    pub fn href(mut self, href: &str) -> Self {
        self.href = Some(Href(href.to_owned()));
        self
    }

    pub fn id(mut self, id: &str) -> Self {
        self.id = Some(Id(id.to_owned()));
        self
    }

    pub fn lang(mut self, lang: &str) -> Self {
        self.lang = Some(Lang(lang.to_owned()));
        self
    }

    pub fn rel(mut self, rel: &str) -> Self {
        self.rel = Some(Rel(rel.to_owned()));
        self
    }

    pub fn src(mut self, src: &str) -> Self {
        self.src = Some(Src(src.to_owned()));
        self
    }

    pub fn target(mut self, target: &str) -> Self {
        self.target = Some(Target(target.to_owned()));
        self
    }

    pub fn tipe(mut self, tipe: &str) -> Self {
        self.tipe = Some(Type(tipe.to_owned()));
        self
    }

    pub fn width(mut self, width: &str) -> Self {
        self.width = Some(Width(width.to_owned()));
        self
    }

    pub fn build(self) -> Attrs {
        Attrs {
            alt: self.alt,
            az: self.az,
            charset: self.charset,
            class: self.class,
            defer: self.defer,
            href: self.href,
            id: self.id,
            lang: self.lang,
            rel: self.rel,
            src: self.src,
            target: self.target,
            tipe: self.tipe,
            width: self.width,
        }
    }
}
