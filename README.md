# Styler
- This repo has draft implementation of style parser. **Note:It does not check any css rules.**
- Currently It has one macro named style! which will parse the css text, add random classname for
all the selectors. this random class name will be same all selectors in same component. As of now we will save all of them into a css file which is named same as component name passed to the style macro.

## Leptos Example

```rust
#[component]
fn Hello(cx: Scope, name: &'static str) -> impl IntoView {
    
    let styler_class = style! {"Hello",
        div.one{
            color: red;
        }
        div #two{
            color: blue;
        }
        div {
            border: 1px solid black;
            margin: 25px 50px 75px 100px;
            background-color: lightblue;
        }
        h2 {
            color: purple;
        }
    }

    view! {cx, class = styler_class,
        <div class="one">
            <h1 id="two">"Hello"</h1>
            <h2>"World"</h2>
            <h2>{name}</h2>
            <h3>"Hello Kanna"</h3>
        </div>
    }
}
```
### For more examples see examples section.
