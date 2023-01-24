# Styler
- Scoped CSS solution for Rust web frameworks which follows component like architecture (e.g Leptos).
- style! macro will check for commas and valid property keys. It will also give suggestions for property key which has syntax errors.

## Leptos Example
- by
```rust
#[component]
fn Hello(cx: Scope, name: &'static str) -> impl IntoView {
    
    let styler_class = style! {"Hello",
        div.one{
            color: red;
            content: raw_str(r#"\hello"#);
            font: "1.3em/1.2" Arial, Helvetica, sans-serif;
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
        @media only screen and (max-width: 1000px) {
            h3 {
                background-color: lightblue;
                color: blue
            }
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

## Note
- **By default double quotes around string literal which is part of property value will be removed. if user wanted to keep the double quote they have use the syntax given below**
```rust
style!(
    div{
        content: raw_str(r#"\hello"#);
        font: "1.3em/1.2" Arial, Helvetica, sans-serif;
    }
)
```
- here style! macro will remove double quote around "1.3em/1.2", but it will change raw_str(r#"\hello"#) into "\hello".

- Currently It has one macro named style! which will parse the css text, add random classname for
all the selectors. this random class name will be same all selectors in same component. As of now we will save all of them into a css file which is named same as component name passed to the style macro. **Note:All the component names in one package must be unique(Or user have to use unique string literal in the style macro for each component.**

## Build
- We are trying to improve the build process. Till then user have to consider given below things when they are building their app.
- When we build the app we will generate two things. One is css directory which contains all css files for each component. At the end we will merge all these css file and create one main.css file. These css file in the directory can be used to debug css of each component.
- Whenever component is recompiled two files will be fully overrided one is {component_name}.css and main.css. If your build tool gives error like "main.css file not found", then create main.css in the root directory of your project.
- **You have to include this main.css in the index.html** in case if you are using build tools like Trunk to build your package (e.g ```<link data-trunk rel="css" href="./main.css">```). **You have to add the css directory in watch ignore options to avoid infinite recompiling**. In Trunk you will add this build ignore in the Trunk.toml file like given below.
```toml
[watch]
ignore = ["./css"]
```

- When there are some unexpected behaviour delete the css directory and rebuild your package.
- style! macro will return the unique class name generated for that component user have to pass it in the view macro as given in the below example.
- If you are having some suggestion or there are some bugs Please feel free to create an issue. 
