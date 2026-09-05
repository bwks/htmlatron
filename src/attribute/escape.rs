use std::fmt;

/// Formatting adapter used by attribute serializers, including exported macros.
#[doc(hidden)]
pub struct EscapedAttributeValue<'a>(pub &'a str);

impl fmt::Display for EscapedAttributeValue<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut start = 0;
        for (index, character) in self.0.char_indices() {
            let entity = match character {
                '&' => "&amp;",
                '"' => "&quot;",
                '\'' => "&#39;",
                '<' => "&lt;",
                '>' => "&gt;",
                _ => continue,
            };
            f.write_str(&self.0[start..index])?;
            f.write_str(entity)?;
            start = index + character.len_utf8();
        }
        f.write_str(&self.0[start..])
    }
}
