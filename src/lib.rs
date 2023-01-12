use leptos::*;
use styler_derive::style;

#[component]
fn Hello(cx: Scope, name: &'static str) -> impl IntoView {
    
    style! {
        div.one{
            color: red;
        }
        div #two{
            color: blue;
        }
        div {
            border: 1px solid black;
            margin: 25px 50px 75px 100px;
            background-color: lightblue;
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
        <div class=format!("one {class_name}")>
            <h1 id="two" class={class_name}>"Hello"</h1>
            <h2 class={class_name}>"World"</h2>
            <h2 class={class_name}>{name}</h2>
            <h3 class={class_name}>"Hello Kanna"</h3>
            <p class={class_name}> "This is example conent"</p>
            <a href="www.google.com">"Visit the link"</a>
        </div>
    }
}

#[component]
pub fn Abi(cx: Scope) -> impl IntoView {
    view! {cx,
        <Hello name="hello"/>
        <h3>"Hai"</h3>
    }
}
