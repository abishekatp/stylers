use gloo::console;
use leptos::*;
use style_macro::*;

pub fn main() {
    console::log!("Hello, stylers!");
    mount_to_body(|cx| view! { cx,  <Abi/> });
}
