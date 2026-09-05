use htmlatron::prelude::*;

#[test]
fn font_preload_renders_cors_and_resource_hints() {
    let link = Link::new()
        .attrs(
            Attrs::new()
                .rel("preload")
                .az("font")
                .typ("font/woff2")
                .crossorigin("anonymous")
                .href("/font.woff2")
                .build(),
        )
        .build()
        .to_string();
    for attribute in [
        "rel=\"preload\"",
        "as=\"font\"",
        "type=\"font/woff2\"",
        "crossorigin=\"anonymous\"",
        "href=\"/font.woff2\"",
    ] {
        assert!(link.contains(attribute), "{attribute}");
    }
}

#[test]
fn crossorigin_values_are_escaped_and_optional() {
    let attrs = Attrs::new().crossorigin("\"<&").build();
    assert_eq!(
        attrs.get_attrs(&Tag::Link),
        vec!["crossorigin=\"&quot;&lt;&amp;\""]
    );
    assert!(Attrs::new().build().get_attrs(&Tag::Link).is_empty());
    assert!(!Attr::global().contains(&Attr::Crossorigin));
}
