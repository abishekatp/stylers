use stylers::style_sheet_test;

//note: temporarily writing these tests. once find a way to test styler_core module we can discard this.
//run this command to run the test cases: cargo run -p styler_test
//Ref: https://www.w3schools.com/cssref/css_selectors.php
pub fn run_tests() {
    println!("------------------Basic Tests------------------");
    let style = style_sheet_test!("./stylers_test/src/test_css_files/basics.css");
    assert_eq!(
        style.trim(),
        ".two.test{color: yellow;}.two.one.test{color: yellow;}.two.test .one.test{color: yellow;}#firstname.test{background-color: yellow;}.test{background-color: yellow;}div.test{border: 1px solid black;margin: 25px 50px 75px 100px;background-color: lightblue;}div.test .one.test p.test{color: blue;}div.one.test p.test div.test{color: blue;}div.test #two.test{color: blue;}h2.test,a.test{color: purple;}"
    );

    println!("------------------Relation Tests------------------");
    let style = style_sheet_test!("./stylers_test/src/test_css_files/relations.css");
    assert_eq!(
        style.trim(),
        r#"div.test>p.test{background-color: yellow;}div.test+p.test{background-color: yellow;}p.test~ul.test{background: #ff0000;}a[target].test{background-color: yellow;}a[title="I am ,testing"].test{background-color: yellow;}[title~=flower].test{background-color: yellow;}[lang|=en].test{background-color: yellow;}div[class^="test"].test{background-color: yellow;}div[class$=test].test{background-color: yellow;}div.test [class$=test].test{background-color: yellow;}div[class*="test"].test{background-color: yellow;}"#
    );

    println!("------------------Pseudo Tests------------------");
    let style = style_sheet_test!("./stylers_test/src/test_css_files/pseudo.css");
    assert_eq!(
        style.trim(),
        r#".one.test:hover{background-color: green;}p.test::before{content: "Read this: ";}div.test:nth-child(2){background-color: green;}p.test:lang(it){background: yellow;}svg.test|a.test{}"#
    );

    println!("------------------AtRules Tests------------------");
    let style = style_sheet_test!("./stylers_test/src/test_css_files/at_rules.css");
    assert_eq!(
        style.trim(),
        r#"@charset "UTF-8";@import url("landscape.css") screen and (orientation: landscape);@namespace svg url("http://www.w3.org/2000/svg");@supports (display: flex) {@media screen and (min-width: 900px) {article.test{display: flex;}}}@supports (display: flex) {.flex-container.test>.test{text-shadow: 0 0 2px blue;float: none;}.flex-container.test{display: flex;}}@document url("https://www.example.com/") {h1.test{color: green;}}"#
    );

    println!("------------------Special AtRules Tests------------------");
    let style = style_sheet_test!("./stylers_test/src/test_css_files/special_at_rules.css");
    dbg!(&style);
    assert_eq!(style.trim(), r#""#);

    // println!("------------------Test-32-----------------");
    // let style = style_test! {"Hello",
    //     @page {
    //         size: A4;
    //         margin: 10%;

    //         @top-left-corner {
    //         content: "Page " counter(page);
    //         }
    //     }
    // };
    // assert_eq!(
    //     style.trim(),
    //     r#"@page{size: A4;margin: 10%;@top-left-corner {content: "Page " counter(page);}}"#
    // );

    // println!("------------------Test-33-----------------");
    // let style = style_test! {"Hello",
    //     @font-face {
    //         font-family: "Trickster";
    //         src: local("Trickster"),
    //         url("trickster-COLRv1.otf") format("opentype") tech(color-COLRv1), url("trickster-outline.otf")
    //             format("opentype"), url("trickster-outline.woff") format("woff");
    //     }
    // };
    // assert_eq!(
    //     style.trim(),
    //     r#"@font-face{font-family: "Trickster";src: local("Trickster"),url("trickster-COLRv1.otf") format("opentype") tech(color-COLRv1), url("trickster-outline.otf")format("opentype"), url("trickster-outline.woff") format("woff");}"#
    // );

    // // todo: currently we not adding any random string to keyframe identifier.
    // //it is users responsibility to make these identifiers unique globaly.
    // println!("------------------Test-34-----------------");
    // let style = style_test! {"Hello",
    //     @keyframes spin1 {
    //         to {
    //             -webkit-transform: rotate(360deg);
    //         }
    //     }
    // };
    // assert_eq!(
    //     style.trim(),
    //     "@keyframes spin1{to {-webkit-transform: rotate(360deg);}}"
    // );

    // println!("------------------Test-35-----------------");
    // let style = style_test! {"Hello",
    //     @-webkit-keyframes spin2 {
    //         to {
    //             -webkit-transform: rotate(360deg);
    //         }
    //     }
    // };
    // assert_eq!(
    //     style.trim(),
    //     "@-webkit-keyframes spin2{to {-webkit-transform: rotate(360deg);}}"
    // );

    // //note: here we have to declare raw string because of backslash charactor
    // println!("------------------Test-36-----------------");
    // let style = style_test! {"Hello",
    //     @counter-style thumbs {
    //         system: cyclic;
    //         symbols: r"\1F44D";
    //         suffix: " ";
    //     }
    // };
    // assert_eq!(
    //     style.trim(),
    //     r#"@counter-style thumbs{system: cyclic;symbols: "\1F44D";suffix: " ";}"#
    // );

    // println!("------------------Test-37-----------------");
    // let style = style_test! {"Hello",
    //     @font-feature-values Font One {
    //         @styleset {
    //             nice-style: 12;
    //         }
    //     }
    // };
    // assert_eq!(
    //     style.trim(),
    //     r#"@font-feature-values Font One{@styleset {nice-style: 12;}}"#
    // );

    // //note: this is experimental css rule.
    // println!("------------------Test-38-----------------");
    // let style = style_test! {"Hello",
    //     @property --property-name {
    //         syntax: "<color>";
    //         inherits: false;
    //         initial-value: #c0ffee;
    //     }
    // };
    // assert_eq!(
    //     style.trim(),
    //     r#"@property --property-name{syntax: "<color>";inherits: false;initial-value: #c0ffee;}"#
    // );

    // //note: when string literal is used as a value internally we will remove that double quotes unless it is wrapped with raw_str().
    // println!("------------------Test-39-----------------");
    // let style = style_test! {"Hello",
    //     @layer framework {
    //         @layer layout {
    //             p {
    //                 margin-block: 1rem;
    //                 font: "0.9em/1.2" Arial, Helvetica, sans-serif;
    //                 content: raw_str(r"\hello");
    //                 content: raw_str(r#"\hello"#);
    //             }
    //         }
    //     }
    // };
    // assert_eq!(
    //     style.trim(),
    //     r#"@layer framework{@layer layout{p.test{margin-block: 1rem;font: 0.9em/1.2 Arial, Helvetica, sans-serif;content: "\hello";content: "\hello";}}}"#
    // );

    // println!("------------------Test-40-----------------");
    // let style = style_test! {"Hello",
    //     @layer theme, layout, utilities;
    // };
    // assert_eq!(style.trim(), r#"@layer theme, layout, utilities;"#);

    // println!("------------------Test-41-----------------");
    // let style = style_test! {"Hello",
    //     :not(body) {
    //         background: #ff0000;
    //     }
    // };
    // assert_eq!(style.trim(), ".test:not(body){background: #ff0000;}");

    // println!("------------------Test-42-----------------");
    // let style = style_test! {"Hello",
    //     :root {
    //         --blue: #1e90ff;
    //     }
    //     body { background-color: var(--blue); }
    // };
    // assert_eq!(
    //     style.trim(),
    //     ":root{--blue: #1e90ff;}body.test{background-color: var(--blue);}"
    // );

    // println!("------------------Test-43-----------------");
    // let style = style_test! {"Hello",
    //     #container {
    //         --first-color: #290;
    //     }
    //     #thirdParagraph {
    //         background-color: var(--first-color);
    //         color: var(--second-color);
    //     }
    // };
    // assert_eq!(
    //     style.trim(),
    //     "#container.test{--first-color: #290;}#thirdParagraph.test{background-color: var(--first-color);color: var(--second-color);}"
    // );
}
