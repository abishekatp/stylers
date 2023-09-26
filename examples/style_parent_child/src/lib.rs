use leptos::*;
use stylers::{style, style_str};

#[component]
pub fn Parent(cx: Scope) -> impl IntoView {
    let class_name = style! {
        button {
            background-color: green;
            border-radius: 8px;
            border-style: none;
            box-sizing: border-box;
            color: yellow;
            cursor: pointer;
            display: inline-block;
            font-family: r#"Haas Grot Text R Web"#, r#"Helvetica Neue"#, Helvetica, Arial, sans-serif;
            font-size: 14px;
            font-weight: 500;
            height: 40px;
            line-height: 20px;
            list-style: none;
            margin: 0;
            outline: none;
            padding: 10px 16px;
            position: relative;
            text-align: center;
            text-decoration: none;
            transition: color 100ms;
            vertical-align: baseline;
            user-select: none;
            -webkit-user-select: none;
        }
        button:hover{
            background-color: yellow;
            color: green;
        }
    };

    view! {cx, class = {class_name},
        <button>"I am green button"</button>
        <Child class_name={class_name.to_string()}/>
    }
}

#[component]
fn Child(cx: Scope, class_name: String) -> impl IntoView {
    let (local_class, style_val) = style_str! {
        button{
            background-color: blue;
        }
        button:hover{
            background-color: yellow;
            color: blue;
        }
    };
    let class_name = format!("{} {}", class_name, local_class);
    view! {cx, class = {class_name.clone()},
        <style>{style_val}</style>
        <button>"I am blue button"</button>
    }
}
