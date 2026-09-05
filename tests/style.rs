use htmlatron::prelude::*;

#[test]
fn style_is_a_global_escaped_attribute() {
    let attrs = Attrs::new().style("width: 70%; --label: \"<&\";").build();
    for tag in [Tag::Img, Tag::A, Tag::Code] {
        assert_eq!(
            attrs.get_attrs(&tag),
            vec!["style=\"width: 70%; --label: &quot;&lt;&amp;&quot;;\""]
        );
    }
    assert!(Attrs::new().build().get_attrs(&Tag::Img).is_empty());
}
