use leptos::*;
use styler_derive::style;

#[component]
pub fn Hello(cx: Scope) -> impl IntoView {
    // create user interfaces with the declarative `view!` macro

    let _a=style!(
        h1 .class2 div{
            color: red;
            font-size: 6rem;
        }
        .class1 div{
            color: red;
            font-size: 6rem;
        }
        h1.class2 #item.class3{
            color: red;
            font-size: 6rem;
        }
    );

    view! {
        cx,
        <div id="one">
            <h1 id="two">"Blue"</h1>
            <h2>"Red"</h2>
        </div>
    }
}


#[cfg(test)]
mod tests{
    use styler_derive::style;
    #[test]
    fn check_output(){
        let _t = style!(
            h1 .class2 div{
                color: red;
                font-size: 6rem;
            }
            .class1 div{
                color: red;
                font-size: 6rem;
            }
            h1.class2 #item.class3{
                color: red;
                font-size: 6rem;
            }
        );
    }
}
