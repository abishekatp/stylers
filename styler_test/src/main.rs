use styler::style_str;

fn main() {
    println!("========================Running Tests=======================");
    run_tests();
}

//note: temporarily writing these tests. once find a way to test styler_core module we can discard this.
//run this command to run the test cases: cargo run -p styler_test
//Ref: https://www.w3schools.com/cssref/css_selectors.php
pub fn run_tests() {
    println!("------------------Test-1------------------");
    let style = style_str! {"Hello",.two{
            color: yellow;
        }
    };
    assert_eq!(style.trim(), ".two.test{color: yellow;}");

    println!("------------------Test-2------------------");
    let style = style_str! {"Hello",
        .two.one  {
            color: yellow;
        }
    };
    assert_eq!(style.trim(), ".two.one.test{color: yellow;}");

    println!("------------------Test-3------------------");
    let style = style_str! {"Hello",
        .two  .one{
            color: yellow;
        }
    };
    assert_eq!(style.trim(), ".two.test .one.test{color: yellow;}");

    println!("------------------Test-4------------------");
    let style = style_str! {"Hello",
        #firstname{
            background-color: yellow;
        }
    };
    assert_eq!(style.trim(), "#firstname.test{background-color: yellow;}");

    // todo: decide weather all element should have the random classname inserted for this.
    println!("------------------Test-5------------------");
    let style = style_str! {"Hello",
        *{
            background-color: yellow;
        }
    };
    assert_eq!(style.trim(), ".test{background-color: yellow;}");

    println!("------------------Test-6------------------");
    let style = style_str! {"Hello",
        div{
            border: 1px solid black;
            margin: 25px 50px 75px 100px;
            background-color: lightblue;
        }
    };
    assert_eq!(style.trim(),"div.test{border: 1px solid black;margin: 25px 50px 75px 100px;background-color: lightblue;}");

    println!("------------------Test-7------------------");
    let style = style_str! {"Hello",
        div .one p{
            color: blue;
        }
    };
    assert_eq!(style.trim(), "div.test .one.test p.test{color: blue;}");

    println!("------------------Test-8------------------");
    let style = style_str! {"Hello",
        div.one p div{
            color: blue;
        }
    };
    assert_eq!(style.trim(), "div.one.test p.test div.test{color: blue;}");

    println!("------------------Test-9------------------");
    let style = style_str! {"Hello",
        div #two{
            color: blue;
        }
    };
    assert_eq!(style.trim(), "div.test #two.test{color: blue;}");

    println!("------------------Test-10------------------");
    let style = style_str! {"Hello",
        h2 , a{
            color: purple;
        }
    };
    assert_eq!(style.trim(), "h2.test,a.test{color: purple;}");

    println!("------------------Test-11------------------");
    let style = style_str! {"Hello",
        div > p{
            background-color: yellow;
        }
    };
    assert_eq!(style.trim(), "div.test>p.test{background-color: yellow;}");

    println!("------------------Test-12-----------------");
    let style = style_str! {"Hello",
        div + p {
            background-color: yellow;
        }
    };
    assert_eq!(style.trim(), "div.test+p.test{background-color: yellow;}");

    println!("------------------Test-13-----------------");
    let style = style_str! {"Hello",
        p ~ ul {
            background: #ff0000;
        }
    };
    assert_eq!(style.trim(), "p.test~ul.test{background: #ff0000;}");

    println!("------------------Test-14-----------------");
    let style = style_str! {"Hello",
        a[target] {
            background-color: yellow;
        }
    };
    assert_eq!(style.trim(), "a[target].test{background-color: yellow;}");

    println!("------------------Test-15-----------------");
    let style = style_str! {"Hello",
        a[title="I am ,testing"] {
            background-color: yellow;
        }
    };
    assert_eq!(
        style.trim(),
        r#"a[title="I am ,testing"].test{background-color: yellow;}"#
    );

    println!("------------------Test-16-----------------");
    let style = style_str! {"Hello",
        [title~=flower] {
            background-color: yellow;
        }
    };
    assert_eq!(
        style.trim(),
        "[title~=flower].test{background-color: yellow;}"
    );

    println!("------------------Test-17-----------------");
    let style = style_str! {"Hello",
        [lang|=en] {
            background-color: yellow;
        }
    };
    assert_eq!(style.trim(), "[lang|=en].test{background-color: yellow;}");

    println!("------------------Test-18-----------------");
    let style = style_str! {"Hello",
        div[class^="test"] {
            background-color: yellow;
        }
    };
    assert_eq!(
        style.trim(),
        r#"div[class^="test"].test{background-color: yellow;}"#
    );

    println!("------------------Test-19-----------------");
    let style = style_str! {"Hello",
        div[class$=test] {
            background-color: yellow;
        }
    };
    assert_eq!(
        style.trim(),
        "div[class$=test].test{background-color: yellow;}"
    );

    println!("------------------Test-20-----------------");
    let style = style_str! {"Hello",
        div [class$=test] {
            background-color: yellow;
        }
    };
    assert_eq!(
        style.trim(),
        "div.test [class$=test].test{background-color: yellow;}"
    );

    println!("------------------Test-21-----------------");
    let style = style_str! {"Hello",
        div[class*="test"] {
            background-color: yellow;
        }
    };
    assert_eq!(
        style.trim(),
        r#"div[class*="test"].test{background-color: yellow;}"#
    );

    println!("------------------Test-22------------------");
    let style = style_str! {"Hello",
        .one:hover{
            background-color: green;
        }
    };
    assert_eq!(style.trim(), ".one.test:hover{background-color: green;}");

    println!("------------------Test-23------------------");
    let style = style_str! {"Hello",
        p::before {
            content: "Read this: ";
        }
    };
    assert_eq!(style.trim(), r#"p.test::before{content: "Read this: ";}"#);

    println!("------------------Test-24------------------");
    let style = style_str! {"Hello",
        div:nth-child(2){
            background-color: green;
        }
    };
    assert_eq!(
        style.trim(),
        "div.test:nth-child(2){background-color: green;}"
    );

    println!("------------------Test-25------------------");
    let style = style_str! {"Hello",
        p:lang(it){
            background: yellow;
        }
    };
    assert_eq!(style.trim(), "p.test:lang(it){background: yellow;}");

    println!("------------------Test-26-----------------");
    let style = style_str! {"Hello",
        @keyframes spin {
            to {
                -webkit-transform: rotate(360deg);
            }
        }
    };
    assert_eq!(
        style.trim(),
        "@keyframes spin.test{to {-webkit-transform: rotate(360deg);}}"
    );

    println!("------------------Test-27-----------------");
    let style = style_str! {"Hello",
        @-webkit-keyframes spin {
            to {
                -webkit-transform: rotate(360deg);
            }
        }
    };
    assert_eq!(
        style.trim(),
        "@-webkit-keyframes spin.test{to {-webkit-transform: rotate(360deg);}}"
    );
}
