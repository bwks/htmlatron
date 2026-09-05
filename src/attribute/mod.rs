//! Attribute builders accept raw values. HTML entities are escaped when rendering,
//! so callers should not pre-escape values. Attribute names and executable values
//! such as event handlers must still come from trusted code.

mod escape;
mod html;
mod makro;

pub use html::{Attr, Attrs, HiddenValue, LinkTarget};
pub use makro::*;

#[doc(hidden)]
pub use escape::EscapedAttributeValue;
