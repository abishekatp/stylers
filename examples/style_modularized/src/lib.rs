use leptos::*;
use stylers::{style, style_str};

#[component]
pub fn GreenButton() -> impl IntoView {
    let (local_class, style_val) = style_str! {
        button{
            background-color: green;
        }
        button:hover{
            background-color: yellow;
            color: green;
        }
    };
    let common_class = button_style();
    let class_name = format!("{} {}", common_class, local_class);

    view! {class = {class_name.clone()},
        <style>{style_val}</style>
        <button>"I am green button"</button>
    }
}

#[component]
pub fn BlueButton() -> impl IntoView {
    let (local_class, style_val) = style_str! {"BlueButton",
        button{
            background-color: blue;
        }
        button:hover{
            background-color: yellow;
            color: blue;
        }
    };
    let common_class = button_style();
    let class_name = format!("{} {}", common_class, local_class);

    view! {class = {class_name.clone()},
        <style>{style_val}</style>
        <button>"I am blue button"</button>
    }
}

pub fn button_style() -> String {
    //note: we can even use style_str and get the style string wherever we use this style
    // but that will populate same style in multiple places at the DOM.
    let class = style! {
        button {
            background-color: #EA4C89;
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
        }
    };

    class.to_string()
}
