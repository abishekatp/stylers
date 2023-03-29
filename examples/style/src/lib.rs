use leptos::*;
use stylers::style;

#[component]
fn Hello(cx: Scope, name: &'static str) -> impl IntoView {
    //note: we will trim all double quotes by default unless it is wrapped with raw_str()
    let class_name = style! {"Hello",
        // this comment will be ignored
        div {
            border: 1px solid black;/*This comment also will be ignored */
            margin: 25px 50px 75px 100px;
            background-color: lightblue;
            content: raw_str(r#"\hello"#);
            font: "1.3em/1.2" Arial, Helvetica, sans-serif;
        }
        .two{
            color: yellow;
        }
        div .one p{
            color: blue;
        }
        div.one{
            color: red;
        }
        div #two{
            color: blue;
        }
        h2,a {
            color: purple;
        }
        .one:hover{
            background-color: green;
        }
        p:lang(it){
            background: yellow;
        }
        p::before {
            content: raw_str("Read this: ");
        }
    };

    view! {cx, class = class_name,
        <div class="one">
            <h1 id="two">"Hello"</h1>
            <h2>"World"</h2>
            <h2>{name}</h2>
            <h3>"Hello Kanna"</h3>
            <p> "This is example conent"</p>
            <a href="www.google.com">"Visit the link"</a>
        </div>
    }
}

#[component]
pub fn Abi(cx: Scope) -> impl IntoView {
    let class_name = style! {"Abi",
        // we can use this :deep() pseudo class in external css file as well.
        div :deep(h3){
            color: orange;
        }
        @media only screen and (max-width: 1000px) {
            h3 {
                background-color: lightblue;
                color: blue
            }
        }
    };
    view! {cx, class = class_name,
        <div>
            <Hello name="hello"/>
            <h3 >"Hai"</h3>
        </div>
    }
}
