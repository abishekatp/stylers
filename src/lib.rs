use leptos::*;
use styler_derive::style;

#[component]
fn Hello(cx: Scope) -> impl IntoView {
    // create user interfaces with the declarative `view!` macro
    style! {
        h1 .class2 div{
            color: red;
            font-size: 6rem;
        }
        .class1 div{
            color: red;
            font-size: 6rem;
        }
        h1.class2 #item.class3{
            color: red;
            font-size: 6rem;
        }
        div {
            border: 1px solid black;
            margin: 25px 50px 75px 100px;
            background-color: lightblue;
        }
        p.four {
            border-style: solid;
            border-color: #ff0000 #00ff00 #0000ff rgb(250,0,255);
        }
    }
    // let a = build();
    // dbg!("hello",a);
    view! {
        cx,
        <div id="one" class={class}>
            <h1 id="two">"Blue"</h1>
            <h2>"Red"</h2>
        </div>
    }
}

#[component]
pub fn Abi(cx: Scope) -> impl IntoView {
    view! {cx,
        <Hello />
        <Hello />
    }
}
