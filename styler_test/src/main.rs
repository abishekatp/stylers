use styler::style_str;

fn main() {
    println!("========================Running Tests=======================");
    run_tests();
}

//note: temporarily writing these tests. once find a way to test styler_core module we can discard this.
//run this command to run the test cases: cargo run -p styler_test
pub fn run_tests() {
    println!("------------------Test-1------------------");
    let style = style_str! {
        div {
            border: 1px solid black;
            margin: 25px 50px 75px 100px;
            background-color: lightblue;
        }
    };
    assert_eq!(style.trim(),"div.test {border: 1px solid black;margin: 25px 50px 75px 100px;background-color: lightblue;}");

    println!("------------------Test-2------------------");
    let style = style_str! {
        .two{
            color: yellow;
        }
    };
    assert_eq!(style.trim(), ".two.test{color: yellow;}");

    println!("------------------Test-3------------------");
    let style = style_str! {
        div .one p{
            color: blue;
        }
    };
    assert_eq!(style.trim(), "div.test .one.test p.test{color: blue;}");

    println!("------------------Test-4------------------");
    let style = style_str! {
        div.one{
            color: red;
        }
    };
    assert_eq!(style.trim(), "div.one.test{color: red;}");

    println!("------------------Test-5------------------");
    let style = style_str! {
        div #two{
            color: blue;
        }
    };
    assert_eq!(style.trim(), "div.test #two.test{color: blue;}");

    println!("------------------Test-6------------------");
    let style = style_str! {
        h2,a {
            color: purple;
        }
    };
    assert_eq!(style.trim(), "h2.test,a.test {color: purple;}");

    println!("------------------Test-7------------------");
    let style = style_str! {
        .one:hover{
            background-color: green;
        }
    };
    assert_eq!(style.trim(), ".one.test:hover{background-color: green;}");

    println!("------------------Test-8------------------");
    let style = style_str! {
        p:lang(it){
            background: yellow;
        }
    };
    assert_eq!(style.trim(), "p.test:lang(it){background: yellow;}");

    println!("------------------Test-9------------------");
    let style = style_str! {
        p::before {
            content: "Read this: ";
        }
    };
    assert_eq!(style.trim(), r#"p.test::before {content: "Read this: ";}"#);

    println!("------------------Test-10-----------------");
    let style = style_str! {
        @keyframes spin {
            to {
                -webkit-transform: rotate(360deg);
            }
        }
    };
    assert_eq!(
        style.trim(),
        "@keyframes spin.test {to {-webkit-transform: rotate(360deg);}}"
    );

    println!("------------------Test-11-----------------");
    let style = style_str! {
        @-webkit-keyframes spin {
            to {
                -webkit-transform: rotate(360deg);
            }
        }
    };
    assert_eq!(
        style.trim(),
        "@-webkit-keyframes spin.test {to {-webkit-transform: rotate(360deg);}}"
    );
}
