use leptos::*;
use stylers::{style, style_sheet};

#[component]
fn Hello(name: &'static str) -> impl IntoView {
    let class_name = style_sheet!("./src/hello.css");

    view! {class = class_name,
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
pub fn Abi() -> impl IntoView {
    let class_name = style! {
        h3{
            background-color: blue;
        }
        @media only screen and (max-width: 1000px) {
            h3 {
                background-color: lightblue;
                color: blue
            }
        }
    };
    view! {class = class_name,
        <Hello name="hello"/>
        <h3 >"Hai"</h3>
    }
}
