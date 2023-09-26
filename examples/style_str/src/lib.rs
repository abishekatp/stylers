use leptos::*;
use stylers::style_str;

#[component]
pub fn GreenButton(cx: Scope) -> impl IntoView {
    let (class_name, style_val) = style_str! {
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
        .one .two{
            font-size: 2.5rem;
            font-weight: 900;
        }
    };

    view! {cx, class = class_name,
        <style>{style_val}</style>
        <button>"I am green button"</button>
        <div class="one">
            <span class="two">"This is Large Text"</span>
        </div>
    }
}
