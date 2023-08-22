use stylers_macro::style_sheet_test;

//todo: make them into separate test cases.
//Ref: https://www.w3schools.com/cssref/css_selectors.php
#[test]
fn test_style_sheet() {
    println!("------------------Basic Tests------------------");
    let style = style_sheet_test!("./stylers_macro/tests/samples/basics.css");
    assert_eq!(
        style.trim(),
        ".two.test{color: yellow;}.two.one.test{color: yellow;}.two.test .one.test{color: yellow;}#firstname.test{background-color: yellow;}.test{background-color: yellow;}div.test{border: 1px solid black;margin: 25px 50px 75px 100px;background-color: lightblue;}div.test .one.test p.test{color: blue;}div.one.test p.test div.test{color: blue;}div.test #two.test{color: blue;}h2.test,a.test{color: purple;}"
    );

    println!("------------------Relation Tests---------------");
    let style = style_sheet_test!("./stylers_macro/tests/samples/relations.css");
    assert_eq!(
        style.trim(),
        r#"div.test>p.test{background-color: yellow;}div.test+p.test{background-color: yellow;}p.test~ul.test{background: #ff0000;}a[target].test{background-color: yellow;}a[title="I am ,testing"].test{background-color: yellow;}[title~=flower].test{background-color: yellow;}[lang|=en].test{background-color: yellow;}div[class^="test"].test{background-color: yellow;}div[class$=test].test{background-color: yellow;}div.test [class$=test].test{background-color: yellow;}div[class*="test"].test{background-color: yellow;}"#
    );

    println!("------------------Pseudo Tests-----------------");
    let style = style_sheet_test!("./stylers_macro/tests/samples/pseudo.css");
    assert_eq!(
        style.trim(),
        r#".one.test:hover{background-color: green;}p.test::before{content: "Read this: ";}div.test:nth-child(2){background-color: green;}p.test:lang(it){background: yellow;}svg.test|a.test{}.test:not(body){background: #ff0000;}:root{--blue: #1e90ff;}body.test{background-color: var(--blue);}#container.test{--first-color: #290;}#thirdParagraph.test{background-color: var(--first-color);color: var(--second-color);}"#
    );

    println!("------------------AtRules Tests----------------");
    let style = style_sheet_test!("./stylers_macro/tests/samples/at_rules.css");
    assert_eq!(
        style.trim(),
        r#"@charset "UTF-8";@import url("landscape.css") screen and (orientation: landscape);@namespace svg url("http://www.w3.org/2000/svg");@layer theme,layout,utilities;@supports (display: flex) {@media screen and (min-width: 900px) {article.test{display: flex;}}}@supports (display: flex) {.flex-container.test>.test{text-shadow: 0 0 2px blue;float: none;}.flex-container.test{display: flex;}}@document url("https://www.example.com/") {h1.test{color: green;}}@layer framework {@layer layout {p.test{margin-block: 1rem;font: 0.9em/1.2 Arial, Helvetica, sans-serif;content: "\hello";content: "\hello";}}}"#
    );

    println!("--------------Special AtRules Tests------------");
    let style = style_sheet_test!("./stylers_macro/tests/samples/special_at_rules.css");
    assert_eq!(
        style.trim(),
        r#"@page {size: A4;margin: 10%;@top-left-corner {content: "Page " counter(page);}}@font-face {font-family: "Trickster";src: local("Trickster"),url("trickster-COLRv1.otf") format("opentype") tech(color-COLRv1), url("trickster-outline.otf") format("opentype"), url("trickster-outline.woff") format("woff");}@keyframes spin1 {to {-webkit-transform: rotate(360deg);}}@-webkit-keyframes spin2 {to {-webkit-transform: rotate(360deg);}}@counter-style thumbs {system: cyclic;symbols: "\1F44D";suffix: " ";}@font-feature-values Font One {@styleset {nice-style: 12;}}@property --property-name {syntax: "<color>";inherits: false;initial-value: #c0ffee;}"#
    );

    println!("--------------Custom Pseudo Class Tests------------");
    let style = style_sheet_test!("./stylers_macro/tests/samples/custom_pseudo.css");
    assert_eq!(
        style.trim(),
        r#"h3 div{color: orange;}div.test h3{color: orange;}div.test>h3{color: orange;}"#
    );
}
