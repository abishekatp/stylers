use stylers_macro::style_test;

//todo: make them into separate test cases.
// Ref: https://www.w3schools.com/cssref/css_selectors.php
#[test]
fn test_style() {
    println!("------------------Test-1------------------");
    let style = style_test! {.two{
            // this comment should be ignored
            color: yellow;
        }
    };
    assert_eq!(style.trim(), ".two.test{color: yellow;}");
    println!("------------------Test-2------------------");
    let style = style_test! {
        .two.one  {
            color: yellow;
        }
    };
    assert_eq!(style.trim(), ".two.one.test{color: yellow;}");

    println!("------------------Test-3------------------");
    let style = style_test! {
        .two  .one{
            color: yellow;
        }
    };
    assert_eq!(style.trim(), ".two.test .one.test{color: yellow;}");

    println!("------------------Test-4------------------");
    let style = style_test! {
        #firstname{
            background-color: yellow;
        }
    };
    assert_eq!(style.trim(), "#firstname.test{background-color: yellow;}");

    // todo: decide weather all element should have the random classname inserted for this.
    println!("------------------Test-5------------------");
    let style = style_test! {
        *{
            background-color: yellow;
        }
    };
    assert_eq!(style.trim(), ".test{background-color: yellow;}");

    println!("------------------Test-6------------------");
    let style = style_test! {
        div{
            border: 1px solid black;
            margin: 25px 50px 75px 100px;
            background-color: lightblue;
        }
    };
    assert_eq!(style.trim(),"div.test{border: 1px solid black;margin: 25px 50px 75px 100px;background-color: lightblue;}");

    println!("------------------Test-7------------------");
    let style = style_test! {
        div .one p{
            color: blue;
        }
    };
    assert_eq!(style.trim(), "div.test .one.test p.test{color: blue;}");

    println!("------------------Test-8------------------");
    let style = style_test! {
        div.one p div{
            color: blue;
        }
    };
    assert_eq!(style.trim(), "div.one.test p.test div.test{color: blue;}");

    println!("------------------Test-9------------------");
    let style = style_test! {
        div #two{
            color: blue;
        }
    };
    assert_eq!(style.trim(), "div.test #two.test{color: blue;}");

    println!("------------------Test-10-----------------");
    let style = style_test! {
        h2 , a{
            color: purple;
        }
    };
    assert_eq!(style.trim(), "h2.test,a.test{color: purple;}");

    println!("------------------Test-11-----------------");
    let style = style_test! {
        div > p{
            background-color: yellow;
        }
    };
    assert_eq!(style.trim(), "div.test>p.test{background-color: yellow;}");

    println!("------------------Test-12-----------------");
    let style = style_test! {
        div + p {
            background-color: yellow;
        }
    };
    assert_eq!(style.trim(), "div.test+p.test{background-color: yellow;}");

    println!("------------------Test-13-----------------");
    let style = style_test! {
        p ~ ul {
            background: #ff0000;
        }
    };
    assert_eq!(style.trim(), "p.test~ul.test{background: #ff0000;}");

    println!("------------------Test-14-----------------");
    let style = style_test! {
        a[target] {
            background-color: yellow;
        }
    };
    assert_eq!(style.trim(), "a[target].test{background-color: yellow;}");

    println!("------------------Test-15-----------------");
    let style = style_test! {
        a[title="I am ,testing"] {
            background-color: yellow;
        }
    };
    assert_eq!(
        style.trim(),
        r#"a[title="I am ,testing"].test{background-color: yellow;}"#
    );

    println!("------------------Test-16-----------------");
    let style = style_test! {
        [title~=flower] {
            background-color: yellow;
        }
    };
    assert_eq!(
        style.trim(),
        "[title~=flower].test{background-color: yellow;}"
    );

    println!("------------------Test-17-----------------");
    let style = style_test! {
        [lang|=en] {
            background-color: yellow;
        }
    };
    assert_eq!(style.trim(), "[lang|=en].test{background-color: yellow;}");

    println!("------------------Test-18-----------------");
    let style = style_test! {
        div[class^="test"] {
            background-color: yellow;
        }
    };
    assert_eq!(
        style.trim(),
        r#"div[class^="test"].test{background-color: yellow;}"#
    );

    println!("------------------Test-19-----------------");
    let style = style_test! {
        div[class$=test] {
            background-color: yellow;
        }
    };
    assert_eq!(
        style.trim(),
        "div[class$=test].test{background-color: yellow;}"
    );

    println!("------------------Test-20-----------------");
    let style = style_test! {
        div [class$=test] {
            background-color: yellow;
        }
    };
    assert_eq!(
        style.trim(),
        "div.test [class$=test].test{background-color: yellow;}"
    );

    println!("------------------Test-21-----------------");
    let style = style_test! {
        div[class*="test"] {
            background-color: yellow;
        }
    };
    assert_eq!(
        style.trim(),
        r#"div[class*="test"].test{background-color: yellow;}"#
    );

    println!("------------------Test-22-----------------");
    let style = style_test! {
        .one:hover{
            background-color: green;
        }
    };
    assert_eq!(style.trim(), ".one.test:hover{background-color: green;}");

    println!("------------------Test-23-----------------");
    let style = style_test! {
        p::before {
            content: raw_str("Read this: ");
        }
    };
    assert_eq!(style.trim(), r#"p.test::before{content: "Read this: ";}"#);

    println!("------------------Test-24-----------------");
    let style = style_test! {
        div:nth-child(2){
            background-color: green;
        }
    };
    assert_eq!(
        style.trim(),
        "div.test:nth-child(2){background-color: green;}"
    );

    println!("------------------Test-25-----------------");
    let style = style_test! {
        p:lang(it){
            background: yellow;
        }
    };
    assert_eq!(style.trim(), "p.test:lang(it){background: yellow;}");

    println!("------------------Test-26-----------------");
    let style = style_test! {
        svg|a {
        }
    };
    assert_eq!(style.trim(), "svg.test|a.test{}");

    //Regular at-rules
    println!("------------------Test-27-----------------");
    let style = style_test! {
        @charset "UTF-8";
    };
    assert_eq!(style.trim(), r#"@charset "UTF-8";"#);

    println!("------------------Test-28-----------------");
    let style = style_test! {
        @import url("landscape.css") screen and (orientation: landscape);
    };
    assert_eq!(
        style.trim(),
        r#"@import url("landscape.css") screen and (orientation: landscape);"#
    );

    //note: this is one of restriction since url contains "//" it cannot be mentioned without double quotes
    println!("------------------Test-29-----------------");
    let style = style_test! {
        @namespace svg url("http://www.w3.org/2000/svg");
    };
    assert_eq!(
        style.trim(),
        r#"@namespace svg url("http://www.w3.org/2000/svg");"#
    );

    //nested at-rules
    println!("------------------Test-30-----------------");
    let style = style_test! {
        @supports (display: flex) {
            @media screen and (min-width: 900px) {
                article {
                    display: flex;
                }
            }
        }
    };
    assert_eq!(
        style.trim(),
        "@supports (display: flex){@media screen and (min-width: 900px){article.test{display: flex;}}}"
    );

    println!("------------------Test-30-----------------");
    let style = style_test! {
        @supports (display: flex) {
            .flex-container > * {
                text-shadow: 0 0 2px blue;
                float: none;
            }

            .flex-container {
                display: flex;
            }
        }
    };
    assert_eq!(
        style.trim(),
        "@supports (display: flex){.flex-container.test>.test{text-shadow: 0 0 2px blue;float: none;}.flex-container.test{display: flex;}}"
    );

    println!("------------------Test-31-----------------");
    let style = style_test! {
        @document url("https://www.example.com/")
        {
            h1 {
                color: green;
            }
        }
    };
    assert_eq!(
        style.trim(),
        r#"@document url("https://www.example.com/"){h1.test{color: green;}}"#
    );

    println!("------------------Test-32-----------------");
    let style = style_test! {
        @page {
            size: A4;
            margin: 10%;

            @top-left-corner {
            content: "Page " counter(page);
            }
        }
    };
    assert_eq!(
        style.trim(),
        r#"@page{size: A4;margin: 10%;@top-left-corner {content: "Page " counter(page);}}"#
    );

    println!("------------------Test-33-----------------");
    let style = style_test! {
        @font-face {
            font-family: "Trickster";
            src: local("Trickster"),
            url("trickster-COLRv1.otf") format("opentype") tech(color-COLRv1), url("trickster-outline.otf")
                format("opentype"), url("trickster-outline.woff") format("woff");
        }
    };
    assert_eq!(
        style.trim(),
        r#"@font-face{font-family: "Trickster";src: local("Trickster"),url("trickster-COLRv1.otf") format("opentype") tech(color-COLRv1), url("trickster-outline.otf")format("opentype"), url("trickster-outline.woff") format("woff");}"#
    );

    // todo: currently we not adding any random string to keyframe identifier.
    //it is users responsibility to make these identifiers unique globaly.
    println!("------------------Test-34-----------------");
    let style = style_test! {
        @keyframes spin1 {
            to {
                -webkit-transform: rotate(360deg);
            }
        }
    };
    assert_eq!(
        style.trim(),
        "@keyframes spin1{to {-webkit-transform: rotate(360deg);}}"
    );

    println!("------------------Test-35-----------------");
    let style = style_test! {
        @-webkit-keyframes spin2 {
            to {
                -webkit-transform: rotate(360deg);
            }
        }
    };
    assert_eq!(
        style.trim(),
        "@-webkit-keyframes spin2{to {-webkit-transform: rotate(360deg);}}"
    );

    //note: here we have to declare raw string because of backslash charactor
    println!("------------------Test-36-----------------");
    let style = style_test! {
        @counter-style thumbs {
            system: cyclic;
            symbols: r"\1F44D";
            suffix: " ";
        }
    };
    assert_eq!(
        style.trim(),
        r#"@counter-style thumbs{system: cyclic;symbols: "\1F44D";suffix: " ";}"#
    );

    println!("------------------Test-37-----------------");
    let style = style_test! {
        @font-feature-values Font One {
            @styleset {
                nice-style: 12;
            }
        }
    };
    assert_eq!(
        style.trim(),
        r#"@font-feature-values Font One{@styleset {nice-style: 12;}}"#
    );

    //note: this is experimental css rule.
    println!("------------------Test-38-----------------");
    let style = style_test! {
        @property --property-name {
            syntax: "<color>";
            inherits: false;
            initial-value: #c0ffee;
        }
    };
    assert_eq!(
        style.trim(),
        r#"@property --property-name{syntax: "<color>";inherits: false;initial-value: #c0ffee;}"#
    );

    //note: when string literal is used as a value internally we will remove that double quotes unless it is wrapped with raw_str().
    println!("------------------Test-39-----------------");
    let style = style_test! {
        @layer framework {
            @layer layout {
                p {
                    margin-block: 1rem;
                    font: "0.9em/1.2" Arial, Helvetica, sans-serif;
                    content: raw_str(r"\hello");
                    content: raw_str(r#"\hello"#);
                }
            }
        }
    };
    assert_eq!(
        style.trim(),
        r#"@layer framework{@layer layout{p.test{margin-block: 1rem;font: 0.9em/1.2 Arial, Helvetica, sans-serif;content: "\hello";content: "\hello";}}}"#
    );

    println!("------------------Test-40-----------------");
    let style = style_test! {
        @layer theme, layout, utilities;
    };
    assert_eq!(style.trim(), r#"@layer theme, layout, utilities;"#);

    println!("------------------Test-41-----------------");
    let style = style_test! {
        :not(body) {
            background: #ff0000;
        }
    };
    assert_eq!(style.trim(), ".test:not(body){background: #ff0000;}");

    println!("------------------Test-42-----------------");
    let style = style_test! {
        :root {
            --blue: #1e90ff;
        }
        body { background-color: var(--blue); }
    };
    assert_eq!(
        style.trim(),
        ":root{--blue: #1e90ff;}body.test{background-color: var(--blue);}"
    );

    println!("------------------Test-43-----------------");
    let style = style_test! {
        #container {
            --first-color: #290;
        }
        #thirdParagraph {
            background-color: var(--first-color);
            color: var(--second-color);
        }
    };
    assert_eq!(
        style.trim(),
        "#container.test{--first-color: #290;}#thirdParagraph.test{background-color: var(--first-color);color: var(--second-color);}"
    );

    println!("------------------Test-44-----------------");
    let style = style_test! {
        table th,
        table td {
            color: red;
        }
    };
    assert_eq!(
        style.trim(),
        "table.test th.test,table.test td.test{color: red;}"
    );

    // Custom pseudo class.
    println!("------------------Test-45-----------------");
    let style = style_test! {
        div :deep(h3) {
            color: orange;
        }
    };
    assert_eq!(style.trim(), "div.test h3{color: orange;}");

    println!("------------------Test-46-----------------");
    let style = style_test! {
        :deep(h3 div) {
            color: orange;
        }
    };
    assert_eq!(style.trim(), "h3 div{color: orange;}");

    println!("------------------Test-47-----------------");
    let style = style_test! {
        div> :deep(h3) {
            color: orange;
        }
    };
    assert_eq!(style.trim(), "div.test>h3{color: orange;}");
}
