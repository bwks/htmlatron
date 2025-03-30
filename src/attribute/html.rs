use std::fmt::Display;

use log::warn;

use crate::tag::Tag;

use super::{
    Alt, Az, Charset, Content, Height, Hidden, Href, HttpEquiv, Id, Lang, Name, Onclick, Rel, Src,
    Tabindex, Target, Type, Width,
};

#[derive(Debug, Clone, PartialEq)]
pub enum LinkTarget {
    Blank,
    Parent,
    Slf,
    Top,
    UnfencedTop,
}
impl Display for LinkTarget {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            LinkTarget::Blank => write!(f, "_blank"),
            LinkTarget::Parent => write!(f, "_parent"),
            LinkTarget::Slf => write!(f, "_self"),
            LinkTarget::Top => write!(f, "_top"),
            LinkTarget::UnfencedTop => write!(f, "_unfencedTop"),
        }
    }
}

impl From<LinkTarget> for String {
    fn from(target: LinkTarget) -> Self {
        target.to_string()
    }
}

pub enum HiddenValue {
    Hidden,
    UntilFound,
}
impl Display for HiddenValue {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            HiddenValue::Hidden => write!(f, "hidden"),
            HiddenValue::UntilFound => write!(f, "until-found"),
        }
    }
}
impl From<HiddenValue> for String {
    fn from(option: HiddenValue) -> Self {
        option.to_string()
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum Attr {
    Alt,
    Az,
    Charset,
    Content,
    Class,
    Data,
    Defer,
    Height,
    Hidden,
    Href,
    HttpEquiv,
    Id,
    Lang,
    Name,
    Onclick,
    Src,
    Tabindex,
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
            Attr::Content => write!(f, "content"),
            Attr::Class => write!(f, "class"),
            Attr::Data => write!(f, "data"),
            Attr::Defer => write!(f, "defer"),
            Attr::Height => write!(f, "height"),
            Attr::Hidden => write!(f, "hidden"),
            Attr::Href => write!(f, "href"),
            Attr::HttpEquiv => write!(f, "http-equiv"),
            Attr::Id => write!(f, "id"),
            Attr::Lang => write!(f, "lang"),
            Attr::Name => write!(f, "name"),
            Attr::Onclick => write!(f, "onclick"),
            Attr::Src => write!(f, "src"),
            Attr::Tabindex => write!(f, "tabindex"),
            Attr::Target => write!(f, "target"),
            Attr::Type => write!(f, "type"),
            Attr::Rel => write!(f, "rel"),
            Attr::Width => write!(f, "width"),
        }
    }
}
impl Attr {
    pub fn all() -> &'static [Attr] {
        &[
            Attr::Alt,
            Attr::Az,
            Attr::Id,
            Attr::Class,
            Attr::Charset,
            Attr::Content,
            Attr::Data,
            Attr::Defer,
            Attr::Height,
            Attr::Href,
            Attr::HttpEquiv,
            Attr::Lang,
            Attr::Name,
            Attr::Onclick,
            Attr::Rel,
            Attr::Src,
            Attr::Tabindex,
            Attr::Target,
            Attr::Type,
            Attr::Width,
        ]
    }
    pub fn global() -> &'static [Attr] {
        &[
            //
            Attr::Id,
            Attr::Class,
            Attr::Data,
            Attr::Hidden,
            Attr::Lang,
            Attr::Tabindex,
        ]
    }
}

// HTML data-* attributes
#[derive(Debug, Clone)]
pub struct Data(pub String, pub String);

impl Display for Data {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, r#"{}-{}="{}""#, Attr::Data, self.0, self.1)
    }
}

// Boolean attributes
#[derive(Debug, Clone)]
pub struct Defer;

impl Display for Defer {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", Attr::Defer)
    }
}

// Vector Attributes
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
    pub content: Option<Content>,
    pub data: Option<Data>,
    pub defer: Option<Defer>,
    pub height: Option<Height>,
    pub hidden: Option<Hidden>,
    pub href: Option<Href>,
    pub http_equiv: Option<HttpEquiv>,
    pub lang: Option<Lang>,
    pub name: Option<Name>,
    pub onclick: Option<Onclick>,
    pub rel: Option<Rel>,
    pub src: Option<Src>,
    pub tabindex: Option<Tabindex>,
    pub target: Option<Target>,
    pub typ: Option<Type>,
    pub width: Option<Width>,
}

impl Attrs {
    pub fn new() -> AttrsBuilder {
        AttrsBuilder::new()
    }
    pub fn get_attrs(&self, tag: &Tag) -> Vec<String> {
        let tag_attributes = Tag::attributes(tag);
        let mut attributes = vec![];

        if self.alt.is_some() && validate_attrs(tag, &Attr::Alt, &tag_attributes) {
            attributes.push(self.alt.as_ref().unwrap().to_string())
        }
        if self.az.is_some() && validate_attrs(tag, &Attr::Az, &tag_attributes) {
            attributes.push(self.az.as_ref().unwrap().to_string())
        }
        if self.charset.is_some() && validate_attrs(tag, &Attr::Charset, &tag_attributes) {
            attributes.push(self.charset.as_ref().unwrap().to_string())
        }
        if self.class.is_some() && validate_attrs(tag, &Attr::Class, &tag_attributes) {
            attributes.push(self.class.as_ref().unwrap().to_string())
        }
        if self.content.is_some() && validate_attrs(tag, &Attr::Content, &tag_attributes) {
            attributes.push(self.content.as_ref().unwrap().to_string())
        }
        if self.data.is_some() && validate_attrs(tag, &Attr::Data, &tag_attributes) {
            attributes.push(self.data.as_ref().unwrap().to_string())
        }
        if self.defer.is_some() && validate_attrs(tag, &Attr::Defer, &tag_attributes) {
            attributes.push(self.defer.as_ref().unwrap().to_string())
        }
        if self.height.is_some() && validate_attrs(tag, &Attr::Height, &tag_attributes) {
            attributes.push(self.height.as_ref().unwrap().to_string())
        }
        if self.href.is_some() && validate_attrs(tag, &Attr::Href, &tag_attributes) {
            attributes.push(self.href.as_ref().unwrap().to_string())
        }
        if self.http_equiv.is_some() && validate_attrs(tag, &Attr::HttpEquiv, &tag_attributes) {
            attributes.push(self.http_equiv.as_ref().unwrap().to_string())
        }
        if self.id.is_some() && validate_attrs(tag, &Attr::Id, &tag_attributes) {
            attributes.push(self.id.as_ref().unwrap().to_string())
        }
        if self.lang.is_some() && validate_attrs(tag, &Attr::Lang, &tag_attributes) {
            attributes.push(self.lang.as_ref().unwrap().to_string())
        }
        if self.name.is_some() && validate_attrs(tag, &Attr::Name, &tag_attributes) {
            attributes.push(self.name.as_ref().unwrap().to_string())
        }
        if self.onclick.is_some() && validate_attrs(tag, &Attr::Onclick, &tag_attributes) {
            attributes.push(self.onclick.as_ref().unwrap().to_string())
        }
        if self.src.is_some() && validate_attrs(tag, &Attr::Src, &tag_attributes) {
            attributes.push(self.src.as_ref().unwrap().to_string())
        }
        if self.tabindex.is_some() && validate_attrs(tag, &Attr::Tabindex, &tag_attributes) {
            attributes.push(self.tabindex.as_ref().unwrap().to_string())
        }
        if self.target.is_some() && validate_attrs(tag, &Attr::Target, &tag_attributes) {
            attributes.push(self.target.as_ref().unwrap().to_string())
        }
        if self.typ.is_some() && validate_attrs(tag, &Attr::Type, &tag_attributes) {
            attributes.push(self.typ.as_ref().unwrap().to_string())
        }
        if self.rel.is_some() && validate_attrs(tag, &Attr::Rel, &tag_attributes) {
            attributes.push(self.rel.as_ref().unwrap().to_string())
        }
        if self.width.is_some() && validate_attrs(tag, &Attr::Width, &tag_attributes) {
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
    pub content: Option<Content>,
    pub data: Option<Data>,
    pub defer: Option<Defer>,
    pub height: Option<Height>,
    pub hidden: Option<Hidden>,
    pub href: Option<Href>,
    pub http_equiv: Option<HttpEquiv>,
    pub id: Option<Id>,
    pub lang: Option<Lang>,
    pub name: Option<Name>,
    pub onclick: Option<Onclick>,
    pub rel: Option<Rel>,
    pub src: Option<Src>,
    pub target: Option<Target>,
    pub tabindex: Option<Tabindex>,
    pub typ: Option<Type>,
    pub width: Option<Width>,
}
impl Default for AttrsBuilder {
    fn default() -> Self {
        Self::new()
    }
}
impl AttrsBuilder {
    pub fn new() -> Self {
        AttrsBuilder {
            alt: None,
            az: None,
            charset: None,
            class: None,
            content: None,
            data: None,
            defer: None,
            height: None,
            hidden: None,
            href: None,
            http_equiv: None,
            id: None,
            lang: None,
            name: None,
            onclick: None,
            rel: None,
            src: None,
            tabindex: None,
            target: None,
            typ: None,
            width: None,
        }
    }
    pub fn alt(mut self, alt: impl Into<String>) -> Self {
        self.alt = Some(Alt(alt.into()));
        self
    }

    pub fn az(mut self, az: impl Into<String>) -> Self {
        self.az = Some(Az(az.into()));
        self
    }

    pub fn charset(mut self, charset: impl Into<String>) -> Self {
        self.charset = Some(Charset(charset.into()));
        self
    }

    pub fn content(mut self, content: impl Into<String>) -> Self {
        self.content = Some(Content(content.into()));
        self
    }

    pub fn class(mut self, class: Vec<impl Into<String>>) -> Self {
        let string_vec: Vec<String> = class.into_iter().map(|s| s.into()).collect();
        self.class = Some(Class(string_vec));
        self
    }

    pub fn data(mut self, key: impl Into<String>, value: impl Into<String>) -> Self {
        self.data = Some(Data(key.into(), value.into()));
        self
    }

    pub fn defer(mut self) -> Self {
        self.defer = Some(Defer);
        self
    }

    pub fn height(mut self, height: impl Into<String>) -> Self {
        self.height = Some(Height(height.into()));
        self
    }

    pub fn hidden(mut self, hidden: HiddenValue) -> Self {
        self.hidden = Some(Hidden(hidden.into()));
        self
    }

    pub fn href(mut self, href: impl Into<String>) -> Self {
        self.href = Some(Href(href.into()));
        self
    }

    pub fn http_equiv(mut self, http_equiv: impl Into<String>) -> Self {
        self.http_equiv = Some(HttpEquiv(http_equiv.into()));
        self
    }

    pub fn id(mut self, id: impl Into<String>) -> Self {
        self.id = Some(Id(id.into()));
        self
    }

    pub fn lang(mut self, lang: impl Into<String>) -> Self {
        self.lang = Some(Lang(lang.into()));
        self
    }

    pub fn name(mut self, name: impl Into<String>) -> Self {
        self.name = Some(Name(name.into()));
        self
    }

    pub fn onclick(mut self, onclick: impl Into<String>) -> Self {
        self.onclick = Some(Onclick(onclick.into()));
        self
    }

    pub fn rel(mut self, rel: impl Into<String>) -> Self {
        self.rel = Some(Rel(rel.into()));
        self
    }

    pub fn src(mut self, src: impl Into<String>) -> Self {
        self.src = Some(Src(src.into()));
        self
    }

    pub fn tabindex(mut self, tabindex: i16) -> Self {
        self.tabindex = Some(Tabindex(tabindex.to_string()));
        self
    }

    pub fn target(mut self, target: LinkTarget) -> Self {
        self.target = Some(Target(target.into()));
        self
    }

    pub fn typ(mut self, typ: impl Into<String>) -> Self {
        self.typ = Some(Type(typ.into()));
        self
    }

    pub fn width(mut self, width: impl Into<String>) -> Self {
        self.width = Some(Width(width.into()));
        self
    }

    pub fn build(self) -> Attrs {
        Attrs {
            alt: self.alt,
            az: self.az,
            charset: self.charset,
            class: self.class,
            content: self.content,
            data: self.data,
            defer: self.defer,
            height: self.height,
            hidden: self.hidden,
            href: self.href,
            http_equiv: self.http_equiv,
            id: self.id,
            lang: self.lang,
            name: self.name,
            onclick: self.onclick,
            rel: self.rel,
            src: self.src,
            tabindex: self.tabindex,
            target: self.target,
            typ: self.typ,
            width: self.width,
        }
    }
}

// Check if the attributes supplied are valid for the HTML tag variant.
fn validate_attrs(tag: &Tag, check_attribute: &Attr, valid_attributes: &[Attr]) -> bool {
    if valid_attributes.contains(check_attribute) {
        true
    } else {
        warn!("HTML tag '{tag}' does not support the '{check_attribute}' attribute");
        false
    }
}
