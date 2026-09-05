# htmlatron

HTML elements and attributes built in Rust.

## Attribute values

Pass raw values to attribute builders. Rendering escapes `&`, `"`, `'`, `<`, and
`>` automatically, including `data-*` values and CSS classes:

```rust
use htmlatron::prelude::*;

let link = A::new()
    .attrs(Attrs::new().href("/search?q=rust&sort=new").build())
    .content("Search")
    .build();
assert_eq!(link.to_string(), "<a href=\"/search?q=rust&amp;sort=new\">Search</a>");
```

Do not pre-escape attributes: a literal `&amp;` is rendered as `&amp;amp;`, so the
browser receives the literal text `&amp;`. Repeated rendering does not change the
stored values. Boolean attributes such as `defer` remain bare attributes.

This is HTML attribute escaping, not URL or JavaScript validation. Attribute
names (including `data-*` keys) and event-handler code must remain trusted.
Element `.content(...)`, `.text(...)`, and raw markup APIs retain their existing
behavior; they do not gain automatic escaping from this change.
