use leptos::*;

#[component]
pub fn Hello(cx: Scope) -> impl IntoView {
    // create user interfaces with the declarative `view!` macro

    view! {
        cx,
        <div>
            <h1>"Hello Styler"</h1>
            <h2>"Hello Styler"</h2>
        </div>
    }

    /* style!{
        .k2341 h1 {
            color: red;
        }

        .k2341 h2 {
            color: green;
        }
    } */
}

// in file
/*
#2341k h1 {
    color: red;
}

*/
