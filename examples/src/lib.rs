use leptos::*;
use styler::style;

#[component]
fn Hello(cx: Scope, name: &'static str) -> impl IntoView {
    let class_name = style! {"Hello",
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
        h3{
            background-color: yellow;
        }
        @media only screen and (max-width: 1000px) {
            h3 {
                background-color: lightblue;
                color: blue
            }
        }
    };
    view! {cx, class = class_name,
        <Hello name="hello"/>
        <h3 >"Hai"</h3>
    }
}
