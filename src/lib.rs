use leptos::*;
use styler_derive::style;

#[component]
pub fn Hello(cx: Scope) -> impl IntoView {
    // create user interfaces with the declarative `view!` macro
    let _k = style!(
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
    );

    // let k = css();
    // dbg!("hello",k);
    view! {
        cx,
        <div id="one">
            <h1 id="two">"Blue"</h1>
            <h2>"Red"</h2>
        </div>
    }
}

