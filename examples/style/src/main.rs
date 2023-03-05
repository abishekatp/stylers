use leptos::*;
use style_macro::*;

pub fn main() {
    println!["Hello, stylers!"];
    mount_to_body(|cx| view! { cx,  <Abi/> });
}
