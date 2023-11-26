use leptos::*;
use stylers::style_sheet_str;

#[component]
pub fn BlueButton() -> impl IntoView {
    let (class_name, style_val) = style_sheet_str!("./src/button.css");

    view! {class = class_name,
        <style>{style_val}</style>
        <button>"I am blue button"</button>
    }
}
