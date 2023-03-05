use leptos::*;
use style_modularized::*;

fn main() {
    mount_to_body(|cx| view! { cx,  <div> <GreenButton/> <BlueButton/> </div>});
}
