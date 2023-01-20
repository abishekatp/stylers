# Styler
- This repo has draft implementation of style parser. **Note:It does not check any css rules.**
- Currently It has one macro named style! which will parse the css text, add random classname for
all the selectors. this random class name will be same all selectors in same somponent. As of now we will save all of them into a single css file.
- **Saving all the css in a single file has some problems because during development time**
  - First Rust analyser will call macro for error analysis which will also write into the same file
  - Second, When particular component is changed we will just recompile that component during compile time which will just append the css to the same file.
- One Solution could be creating temporary css file for each component(using that component name as file name) and at the end merging all the files to get final css. In this case we need to get the component name inside macro somehow!.

## Example

```rust
#[component]
fn Hello(cx: Scope, name: &'static str) -> impl IntoView {
    
    style! {"Hello",
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
        <div class=format!("one {__STYLER_CLASS_NAME}")>
            <h1 id="two" class={__STYLER_CLASS_NAME}>"Hello"</h1>
            <h2 class={__STYLER_CLASS_NAME}>"World"</h2>
            <h2 class={__STYLER_CLASS_NAME}>{name}</h2>
            <h3 class={__STYLER_CLASS_NAME}>"Hello Kanna"</h3>
        </div>
    }
}
```

 #### Here style macro gives one unique class name called __STYLER_CLASS_NAME that needs to be handled by view macro!
