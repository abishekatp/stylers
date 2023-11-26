use leptos::*;
use stylers::style;

#[component]
fn Hello(name: &'static str) -> impl IntoView {
    //note: we will trim all double quotes by default unless it is wrapped with raw_str()
    let class_name = style! {
        // this comment will be ignored
        div {
            border: 1px solid black; /* This comment also will be ignored */
            margin: 25px 50px 75px 100px;
            background-color: lightblue;
            // The macro trims all double quotes by default unless it is wrapped with raw_str()
            content: raw_str(r#"\hello"#);
            font: "1.3em/1.2" Arial, Helvetica, sans-serif;
        }
        .two{
            color: orange;
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
            color: red;
        }
        .one:hover{
            background-color: yellow;
        }
        p:lang(it){
            background: yellow;
        }
        p::before {
            content: raw_str("Read this: ");
        }
        .example-url{
            content: r#"url("https://picsum.photos/200/300")"#;
        }
    };

    view! {class = class_name,
        <div class="one">
            <h1 id="two">"Hello"</h1>
            <h2>"World"</h2>
            <h2>{name}</h2>
            <h3>"Hello Kanna"</h3>
            <p> "This is example content"</p>
            <a href="www.google.com">"Visit the link"</a>
        </div>
        <span class="example-url"></span>
    }
}

#[component]
pub fn Abi() -> impl IntoView {
    let class_name = style! {
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
        :deep(.rollUp ) .deep-text{
            color: orange;
            font-size: 1.5rem;
        }
    };
    view! {class = class_name,
        <div class="rollUp">
            <Hello name="hello"/>
            <h3 >"Hai"</h3>

            <span class="deep-text">:deep directive test</span>
        </div>

    }
}
