use leptos::*;
use styler::style;

#[component]
fn Hello(cx: Scope, name: &'static str) -> impl IntoView {
    style! {"Hello",
        div {
            border: 1px solid black;
            margin: 25px 50px 75px 100px;
            background-color: lightblue;
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
            content: "Read this: ";
        }
    }
    // Above style macro returns one unique class_name that needs to be handled by view macro.
    // currently mapped to dom manually.
    view! {cx,
        <div class=format!("one {__STYLER_CLASS_NAME}")>
            <h1 id="two" class={__STYLER_CLASS_NAME}>"Hello"</h1>
            <h2 class={__STYLER_CLASS_NAME}>"World"</h2>
            <h2 class={__STYLER_CLASS_NAME}>{name}</h2>
            <h3 class={__STYLER_CLASS_NAME}>"Hello Kanna"</h3>
            <p class={__STYLER_CLASS_NAME}> "This is example conent"</p>
            <a href="www.google.com">"Visit the link"</a>
        </div>
    }
}

#[component]
pub fn Abi(cx: Scope) -> impl IntoView {
    style! {"Abi",
        h3{
            background-color: yellow;
        }
    }
    view! {cx,
        <Hello name="hello"/>
        <h3 class={__STYLER_CLASS_NAME}>"Hai"</h3>
    }
}
