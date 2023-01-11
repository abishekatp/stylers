use leptos::*;
use styler_derive::style;

#[component]
fn Hello(cx: Scope, name: &'static str) -> impl IntoView {
    // create user interfaces with the declarative `view!` macro
    //check out this is not allowed. div#one
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
        p.four {
            border-style: solid;
            border-color: #ff0000 #00ff00 #0000ff rgb(250,0,255);
        }
    }
    // h1 .class2 div{
    //     color: red;
    //     font-size: 6rem;
    // }
    // .class1 div{
    //     color: red;
    //     font-size: 6rem;
    // }
    // h1.class2 #item.class3{
    //     color: red;
    //     font-size: 6rem;
    // }
    // let a = build();
    // dbg!("hello",a);
    view! {
        cx,
        <div>
            <div class="one" class={class}>
                <h1 id="two" class={class}>"Blue"</h1>
                <h2 class={class}>"Red"</h2>
                <h2 class={class}>{name}</h2>
            </div>
        </div>

    }
}

#[component]
pub fn Abi(cx: Scope) -> impl IntoView {
    view! {cx,
        <Hello name="hello"/>
        // <Hello name="hai" />
    }
}
