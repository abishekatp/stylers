use gloo::console;
use leptos::*;
// use style_build::*;

pub fn main() {
    console::log!("Hello, stylers1!");
    mount_to_body(|cx| view! { cx,  <div>"Hello"</div> })
}
