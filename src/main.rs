use leptos::*;
use stylers::*;

pub fn main() {
    println!["Hello, stylers!"];
    mount_to_body(|cx| view! { cx,  <Abi/> });
}
