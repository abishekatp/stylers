# Styler
- This repo has draft implementation of style parser. **Note:It does not check any css rules.**
- Currently It has one macro named style! which will parse the css text, add random classname for
all the selectors. this random class name will be same all selectors in same component. As of now we will save all of them into a css file which is named same as component name passed to the style macro.

## Example

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

    // Above style macro returns one unique class_name that needs to be handled by view macro.
    // currently mapped to dom manually.
    view! {cx,
        <div class=format!("one {styler_class}")>
            <h1 id="two" class={styler_class}>"Hello"</h1>
            <h2 class={styler_class}>"World"</h2>
            <h2 class={styler_class}>{name}</h2>
            <h3 class={styler_class}>"Hello Kanna"</h3>
        </div>
    }
}
```

 #### Here style macro gives one unique class name called styler_class that needs to be handled by view macro!
