use leptos::*;
use stylers::*;
use styler_derive::style;


pub fn main() {
    println!["Hello, stylers!"];
    mount_to_body(|cx| view! { cx,  <Hello/> });
}