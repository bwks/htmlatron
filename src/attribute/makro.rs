use std::fmt::Display;

use super::attribute::Attr;

#[macro_export]
macro_rules! html_attribute {
    ($name:ident => $attr:expr) => {
        #[derive(Debug, Clone)]
        pub struct $name(pub String);

        impl Display for $name {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                match self {
                    $name(value) => {
                        write!(f, r#"{}="{}""#, $attr, value)
                    }
                }
            }
        }
    };
}

// Usage
html_attribute!(Az => Attr::Az);
html_attribute!(Alt => Attr::Alt);
html_attribute!(Charset => Attr::Charset);
html_attribute!(Content => Attr::Content);
html_attribute!(Href => Attr::Href);
html_attribute!(HttpEquiv => Attr::HttpEquiv);
html_attribute!(Id => Attr::Id);
html_attribute!(Lang => Attr::Lang);
html_attribute!(Name => Attr::Name);
html_attribute!(Rel => Attr::Rel);
html_attribute!(Src => Attr::Src);
html_attribute!(Target => Attr::Target);
html_attribute!(Type => Attr::Type);
html_attribute!(Width => Attr::Width);
