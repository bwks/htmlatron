#![allow(dead_code)]

use std::fmt::Display;

// use attributes::{Alt, Charset, Href, Id, Lang, Rel, Src};
// use elements::{
//     Body, Br, Comment, Div, Footer, Header, Html, Img, Li, Link, Meta, Ol, Script, Title, Ul, A,
//     H1, H2, H3, H4, H5, H6, P,
// };

//

fn main() {
    let id1 = "one";
    let id2 = "two";
    let class1 = vec!["three", "four"];
    let class2 = vec!["seven", "eight"];
    let lang = "en";
    let meta = Meta::new()
        .attrs(Attrs::new().charset("utf-8").build())
        .build();
    let title = Title::new().content("The is the title").build();
    let comment = Comment::new().text("this is a comment").build();

    let html = Html::new().attrs(Attrs::new().lang(lang).build());
    let js = Script::new()
        .attrs(Attrs::new().src("script.js").build())
        .build();
    let css = Link::new()
        .attrs(Attrs::new().rel("stylesheet").href("styles.css").build())
        .build();

    let image = Img::new()
        .attrs(Attrs::new().src("blah.img").alt("some image").build())
        .build();

    let header = Header::new()
        .attrs(Attrs::new().id("header").build())
        .children(vec![meta, css, js, title])
        .build();

    let footer = Footer::new()
        .attrs(Attrs::new().id("footer").build())
        .build();

    let line_break = Br::new().build();

    let ul = Ul::new()
        .children(vec![
            Li::new().build(),
            Li::new().build(),
            Li::new().build(),
        ])
        .build();
    let ol = Ol::new()
        .children(vec![
            Li::new().build(),
            Li::new().build(),
            Li::new().build(),
        ])
        .build();

    let content = Div::new()
        .attrs(Attrs::new().id(id1).class(class1.clone()).build())
        .children(vec![
            //
            image,
            Div::new()
                .attrs(Attrs::new().id(id2).class(class2.clone()).build())
                .children(vec![
                    H1::new().content("Heading 1").build(),
                    P::new()
                        .content("blah blah blah")
                        .children(vec![
                            //
                            A::new()
                                .attrs(Attrs::new().href("http://stuff.things").build())
                                .content("Stuff and Things")
                                .build(),
                        ])
                        .build(),
                    H2::new().content("Heading 2").build(),
                    ul,
                    H3::new().content("Heading 3").build(),
                    ol,
                ])
                .build(),
        ])
        .build();

    let body = Body::new()
        .attrs(Attrs::new().class(vec!["body"]).build())
        .children(vec![content])
        .build();

    let document = Document {
        doctype: Doctype::Html,
        tags: vec![
            comment,
            html.children(vec![
                header,
                body,
                footer,
                line_break.clone(),
                line_break.clone(),
                line_break.clone(),
            ])
            .build(),
        ],
    };

    println!("{}", document);
}
