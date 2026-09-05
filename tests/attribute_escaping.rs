use htmlatron::prelude::{Alt, Attrs, Div, Href, Img, Onclick, Script};

// Integration tests compile as a downstream crate. No Display or helper imports
// are needed for an exported macro invocation.
htmlatron::html_attribute!(CustomLabel => "aria-label");

#[test]
fn scalar_attributes_escape_raw_values() {
    for (raw, escaped) in [
        ("", ""),
        ("ordinary value", "ordinary value"),
        ("café 🦀 日本語", "café 🦀 日本語"),
        ("&\"'<>", "&amp;&quot;&#39;&lt;&gt;"),
        (
            "a & b < c > d \"quoted\" 'single'",
            "a &amp; b &lt; c &gt; d &quot;quoted&quot; &#39;single&#39;",
        ),
        ("🦀<&日本語\"", "🦀&lt;&amp;日本語&quot;"),
        ("&amp; &#39; &quot;", "&amp;amp; &amp;#39; &amp;quot;"),
    ] {
        assert_eq!(Alt(raw.into()).to_string(), format!("alt=\"{escaped}\""));
        assert_eq!(
            CustomLabel(raw.into()).to_string(),
            format!("aria-label=\"{escaped}\"")
        );
    }
}

#[test]
fn query_parameters_and_handlers_keep_their_raw_values() {
    assert_eq!(
        Href("/search?q=rust&sort=new".into()).to_string(),
        "href=\"/search?q=rust&amp;sort=new\""
    );
    assert_eq!(
        Onclick("openModal('photo'); if (a < b && c > d) alert(\"yes\")".into()).to_string(),
        "onclick=\"openModal(&#39;photo&#39;); if (a &lt; b &amp;&amp; c &gt; d) alert(&quot;yes&quot;)\""
    );
}

#[test]
fn regular_data_and_class_values_cannot_break_out_of_their_quotes() {
    let payload = "\" onmouseover=\"alert('x')\"><script>&";
    let escaped = "&quot; onmouseover=&quot;alert(&#39;x&#39;)&quot;&gt;&lt;script&gt;&amp;";
    let image = Img::new().attrs(Attrs::new().alt(payload).build()).build();
    assert_eq!(image.to_string(), format!("<img alt=\"{escaped}\">"));

    let div = Div::new()
        .attrs(
            Attrs::new()
                .data("example", payload)
                .class(vec!["safe", payload])
                .build(),
        )
        .build();
    assert_eq!(
        div.to_string(),
        format!("<div class=\"safe {escaped}\" data-example=\"{escaped}\"></div>")
    );
}

#[test]
fn repeated_rendering_does_not_mutate_or_double_escape_values() {
    let element = Div::new()
        .attrs(
            Attrs::new()
                .data("action", "click->hello#greet")
                .class(vec!["[&>span]:block", "font-bold"])
                .build(),
        )
        .build();
    let expected = "<div class=\"[&amp;&gt;span]:block font-bold\" data-action=\"click-&gt;hello#greet\"></div>";
    assert_eq!(element.to_string(), expected);
    assert_eq!(element.to_string(), expected);
    assert_eq!(element.clone().to_string(), expected);
    assert_eq!(
        element.attrs.as_ref().unwrap().data.as_ref().unwrap().1,
        "click->hello#greet"
    );
}

#[test]
fn boolean_attributes_and_raw_element_content_are_unchanged() {
    let script = Script::new()
        .attrs(Attrs::new().defer().build())
        .content("if (a < b && c > d) alert('ok');")
        .build();
    assert_eq!(
        script.to_string(),
        "<script defer>if (a < b && c > d) alert('ok');</script>"
    );
}
