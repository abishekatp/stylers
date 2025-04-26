// note: temporarily writing these tests. once find a way to test styler_core module we can discard this.
// run this command cargo run inside styler_test folder.
// Ref: https://www.w3schools.com/cssref/css_selectors.php

use stylers_macro::style_test;

#[test]
fn test_1() {
  let style = style_test! {.two{
          // this comment should be ignored
          color: yellow;
      }
  };
  assert_eq!(style, ".two.test{color: yellow;}");
}

#[test]
fn test_2() {
  let style = style_test! {
      .two.one  {
          color: yellow;
      }
  };
  assert_eq!(style, ".two.one.test{color: yellow;}");
}

#[test]
fn test_3() {
  let style = style_test! {
      .two  .one{
          color: yellow;
      }
  };
  assert_eq!(style, ".two.test .one.test{color: yellow;}");
}

#[test]
fn test_4() {
  let style = style_test! {
      #firstname{
          background-color: yellow;
      }
  };
  assert_eq!(style, "#firstname.test{background-color: yellow;}");
}

#[test]
fn test_5() {
  // todo: decide weather all element should have the random classname inserted for this.
  let style = style_test! {
      *{
          background-color: yellow;
      }
  };
  assert_eq!(style, ".test{background-color: yellow;}");
}

#[test]
fn test_6() {
  let style = style_test! {
      div{
          border: 1px solid black;
          margin: 25px 50px 75px 100px;
          background-color: lightblue;
      }
  };
  assert_eq!(
    style,
    "div.test{border: 1px solid black;margin: 25px 50px 75px 100px;background-color: lightblue;}"
  );
}
#[test]
fn test_7() {
  let style = style_test! {
      div .one p{
          color: blue;
      }
  };
  assert_eq!(style, "div.test .one.test p.test{color: blue;}");
}

#[test]
fn test_8() {
  let style = style_test! {
      div.one p div{
          color: blue;
      }
  };
  assert_eq!(style, "div.one.test p.test div.test{color: blue;}");
}

#[test]
fn test_9() {
  let style = style_test! {
      div #two{
          color: blue;
      }
  };
  assert_eq!(style, "div.test #two.test{color: blue;}");
}

#[test]
fn test_10() {
  let style = style_test! {
      h2 , a{
          color: purple;
      }
  };
  assert_eq!(style, "h2.test,a.test{color: purple;}");
}

#[test]
fn test_11() {
  let style = style_test! {
      div > p{
          background-color: yellow;
      }
  };
  assert_eq!(style, "div.test>p.test{background-color: yellow;}");
}

#[test]
fn test_12() {
  let style = style_test! {
      div + p {
          background-color: yellow;
      }
  };
  assert_eq!(style, "div.test+p.test{background-color: yellow;}");
}

#[test]
fn test_13() {
  let style = style_test! {
      p ~ ul {
          background: #ff0000;
      }
  };
  assert_eq!(style, "p.test~ul.test{background: #ff0000;}");
}

#[test]
fn test_14() {
  let style = style_test! {
      a[target] {
          background-color: yellow;
      }
  };
  assert_eq!(style, "a[target].test{background-color: yellow;}");
}

#[test]
fn test_15() {
  let style = style_test! {
      a[title="I am ,testing"] {
          background-color: yellow;
      }
  };
  assert_eq!(
    style,
    r#"a[title="I am ,testing"].test{background-color: yellow;}"#
  );
}

#[test]
fn test_16() {
  let style = style_test! {
      [title~=flower] {
          background-color: yellow;
      }
  };
  assert_eq!(style, "[title~=flower].test{background-color: yellow;}");
}

#[test]
fn test_17() {
  let style = style_test! {
      [lang|=en] {
          background-color: yellow;
      }
  };
  assert_eq!(style, "[lang|=en].test{background-color: yellow;}");
}

#[test]
fn test_18() {
  let style = style_test! {
      div[class^="test"] {
          background-color: yellow;
      }
  };
  assert_eq!(
    style,
    r#"div[class^="test"].test{background-color: yellow;}"#
  );
}

#[test]
fn test_19() {
  let style = style_test! {
      div[class$=test] {
          background-color: yellow;
      }
  };
  assert_eq!(style, "div[class$=test].test{background-color: yellow;}");
}

#[test]
fn test_20() {
  let style = style_test! {
      div [class$=test] {
          background-color: yellow;
      }
  };
  assert_eq!(
    style,
    "div.test [class$=test].test{background-color: yellow;}"
  );
}

#[test]
fn test_21() {
  let style = style_test! {
      div[class*="test"] {
          background-color: yellow;
      }
  };
  assert_eq!(
    style,
    r#"div[class*="test"].test{background-color: yellow;}"#
  );
}

#[test]
fn test_22() {
  let style = style_test! {
      .one:hover{
          background-color: green;
      }
  };
  assert_eq!(style, ".one.test:hover{background-color: green;}");
}

#[test]
fn test_23() {
  let style = style_test! {
      p::before {
          content: raw_str("Read this: ");
      }
  };
  assert_eq!(style, r#"p.test::before{content: "Read this: ";}"#);
}

#[test]
fn test_24() {
  let style = style_test! {
      div:nth-child(2){
          background-color: green;
      }
  };
  assert_eq!(style, "div.test:nth-child(2){background-color: green;}");
}

#[test]
fn test_25() {
  let style = style_test! {
      p:lang(it){
          background: yellow;
      }
  };
  assert_eq!(style, "p.test:lang(it){background: yellow;}");
}

#[test]
fn test_26() {
  let style = style_test! {
      svg|a {
      }
  };
  assert_eq!(style, "svg.test|a.test{}");

  //Regular at-rules
}

#[test]
fn test_27() {
  let style = style_test! {
      @charset "UTF-8";
  };
  assert_eq!(style, r#"@charset "UTF-8";"#);
}

#[test]
fn test_28() {
  let style = style_test! {
      @import url("landscape.css") screen and (orientation: landscape);
  };
  assert_eq!(
    style,
    r#"@import url("landscape.css") screen and (orientation: landscape);"#
  );

  //note: this is one of restriction since url contains "//" it cannot be mentioned without double quotes
}

#[test]
fn test_29() {
  let style = style_test! {
      @namespace svg url("http://www.w3.org/2000/svg");
  };
  assert_eq!(
    style,
    r#"@namespace svg url("http://www.w3.org/2000/svg");"#
  );

  //nested at-rules
}

#[test]
fn test_30() {
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
    style,
    "@supports (display: flex){@media screen and (min-width: 900px){article.test{display: flex;}}}"
  );
}

#[test]
fn test_31() {
  let style = style_test! {
      @document url("https://www.example.com/")
      {
          h1 {
              color: green;
          }
      }
  };
  assert_eq!(
    style,
    r#"@document url("https://www.example.com/"){h1.test{color: green;}}"#
  );
}

#[test]
fn test_32() {
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
    style,
    r#"@page{size: A4;margin: 10%;@top-left-corner {content: "Page " counter(page);}}"#
  );
}

#[test]
fn test_33() {
  let style = style_test! {
      @font-face {
          font-family: "Trickster";
          src: local("Trickster"),
          url("trickster-COLRv1.otf") format("opentype") tech(color-COLRv1), url("trickster-outline.otf")
              format("opentype"), url("trickster-outline.woff") format("woff");
      }
  };
  assert_eq!(
    style,
    r#"@font-face{font-family: "Trickster";src: local("Trickster"),url("trickster-COLRv1.otf") format("opentype") tech(color-COLRv1), url("trickster-outline.otf")format("opentype"), url("trickster-outline.woff") format("woff");}"#
  );

  // todo: currently we not adding any random string to keyframe identifier.
  //it is users responsibility to make these identifiers unique globaly.
}

#[test]
fn test_34() {
  let style = style_test! {
      @keyframes spin1 {
          to {
              -webkit-transform: rotate(360deg);
          }
      }
  };
  assert_eq!(
    style,
    "@keyframes spin1{to {-webkit-transform: rotate(360deg);}}"
  );
}

#[test]
fn test_35() {
  let style = style_test! {
      @-webkit-keyframes spin2 {
          to {
              -webkit-transform: rotate(360deg);
          }
      }
  };
  assert_eq!(
    style,
    "@-webkit-keyframes spin2{to {-webkit-transform: rotate(360deg);}}"
  );

  //note: here we have to declare raw string because of backslash charactor
}

#[test]
fn test_36() {
  let style = style_test! {
      @counter-style thumbs {
          system: cyclic;
          symbols: r"\1F44D";
          suffix: " ";
      }
  };
  assert_eq!(
    style,
    r#"@counter-style thumbs{system: cyclic;symbols: "\1F44D";suffix: " ";}"#
  );
}

#[test]
fn test_37() {
  let style = style_test! {
      @font-feature-values Font One {
          @styleset {
              nice-style: 12;
          }
      }
  };
  assert_eq!(
    style,
    r#"@font-feature-values Font One{@styleset {nice-style: 12;}}"#
  );

  //note: this is experimental css rule.
}

#[test]
fn test_38() {
  let style = style_test! {
      @property --property-name {
          syntax: "<color>";
          inherits: false;
          initial-value: #c0ffee;
      }
  };
  assert_eq!(
    style,
    r#"@property --property-name{syntax: "<color>";inherits: false;initial-value: #c0ffee;}"#
  );

  //note: when string literal is used as a value internally we will remove that double quotes unless it is wrapped with raw_str().
}

#[test]
fn test_39() {
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
    style,
    r#"@layer framework{@layer layout{p.test{margin-block: 1rem;font: 0.9em/1.2 Arial, Helvetica, sans-serif;content: "\hello";content: "\hello";}}}"#
  );
}

#[test]
fn test_40() {
  let style = style_test! {
      @layer theme, layout, utilities;
  };
  assert_eq!(style, r#"@layer theme, layout, utilities;"#);
}

#[test]
fn test_41() {
  let style = style_test! {
      :not(body) {
          background: #ff0000;
      }
  };
  assert_eq!(style, ".test:not(body){background: #ff0000;}");
}

#[test]
fn test_42() {
  let style = style_test! {
      :root {
          --blue: #1e90ff;
      }

      body { background-color: var(--blue); }
  };
  assert_eq!(
    style,
    ":root{--blue: #1e90ff;}body.test{background-color: var(--blue);}"
  );
}

#[test]
fn test_43() {
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
    style,
    "#container.test{--first-color: #290;}#thirdParagraph.test{background-color: var(--first-color);color: var(--second-color);}"
  );
}

#[test]
fn test_44() {
  let style = style_test! {
      table th,
      table td {
          color: red;
      }
  };
  assert_eq!(style, "table.test th.test,table.test td.test{color: red;}");

  // Custom pseudo class.
}

#[test]
fn test_45() {
  let style = style_test! {
      div :deep(h3) {
          color: orange;
      }
  };
  assert_eq!(style, "div.test h3{color: orange;}");
}

#[test]
fn test_46() {
  let style = style_test! {
      :deep(h3 div) {
          color: orange;
      }
  };
  assert_eq!(style, "h3 div{color: orange;}");
}

#[test]
fn test_47() {
  let style = style_test! {
      div> :deep(h3) {
          color: orange;
      }
  };
  assert_eq!(style, "div.test>h3{color: orange;}");
}

#[test]
fn test_48() {
  let style = style_test! {
      :deep([data-custom]) {
          color: orange;
      }
  };
  assert_eq!(style, "[data-custom]{color: orange;}");
}

#[test]
fn test_49() {
  let style = style_test! {
      .nested> :deep([data-custom]) {
          color: orange;
      }
  };
  assert_eq!(style, ".nested.test>[data-custom]{color: orange;}");
}

#[test]
fn test_50() {
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
    style,
    "@supports (display: flex){.flex-container.test>.test{text-shadow: 0 0 2px blue;float: none;}.flex-container.test{display: flex;}}"
  );
}

#[test]
fn test_51() {
  let style = style_test! {
      :deep(.rollUp) .unlockSplash {
          max-height: 0;
      }
  };
  assert_eq!(style, ".rollUp .unlockSplash.test{max-height: 0;}");
}

#[test]
fn test_52() {
  let style = style_test! {
      .unitToggle :deep(.onDisplay),
      .unitToggle :deep(.offDisplay) {
          color: black;
      }
  };
  assert_eq!(
    style,
    ".unitToggle.test .onDisplay,.unitToggle.test .offDisplay{color: black;}"
  );
}

#[test]
fn test_53() {
  let style = style_test! {
       .wingman :deep(svg[role=graphics-symbol]) {
         width: 100%;
      }
  };
  assert_eq!(
    style,
    ".wingman.test svg[role=graphics-symbol]{width: 100%;}"
  );
}

#[test]
fn test_54() {
  let style = style_test! {
      .errorSign {
          transform-box: fill-box;
          transform-origin: center;
          scrollbar-width: 1px;
          scrollbar-color: red;
      }
  };
  assert_eq!(
    style,
    ".errorSign.test{transform-box: fill-box;transform-origin: center;scrollbar-width: 1px;scrollbar-color: red;}"
  );
}
