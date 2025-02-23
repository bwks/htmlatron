use std::fmt::Display;

use super::html::Attr;

#[macro_export]
macro_rules! html_attributes {
    ($name:ident => $attr:expr) => {
        #[derive(Debug, Clone)]
        pub struct $name(pub String);

        impl Display for $name {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                write!(f, r#"{}="{}""#, $attr, self.0)
            }
        }
    };
}

// Usage
html_attributes!(Az => Attr::Az);
html_attributes!(Alt => Attr::Alt);
html_attributes!(Charset => Attr::Charset);
html_attributes!(Content => Attr::Content);
html_attributes!(Height => Attr::Height);
html_attributes!(Href => Attr::Href);
html_attributes!(HttpEquiv => Attr::HttpEquiv);
html_attributes!(Id => Attr::Id);
html_attributes!(Lang => Attr::Lang);
html_attributes!(Name => Attr::Name);
html_attributes!(Onclick => Attr::Onclick);
html_attributes!(Rel => Attr::Rel);
html_attributes!(Src => Attr::Src);
html_attributes!(Tabindex => Attr::Tabindex);
html_attributes!(Target => Attr::Target);
html_attributes!(Type => Attr::Type);
html_attributes!(Width => Attr::Width);
