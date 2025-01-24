pub mod attribute;
pub mod document;
pub mod element;
pub mod tag;

pub mod prelude {
    pub use super::attribute::*;
    pub use super::document::*;
    pub use super::element::*;
    pub use super::tag::*;
}
