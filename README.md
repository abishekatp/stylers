# Stylers
- Scoped CSS for Rust web frameworks like Leptos.
- `style!` macro is for writing css inside rust functions directly. It will validates css properties as well.
- `style_sheet!` macro is for writing css in external css file and importing that inside rust functions.
- `style_str!` macro is same as `style!` macro but it will return the tuple `(class_name, style_val)` instead of saving the style_val to the separate file.
- `style_sheet_str!` this same as `style_sheet!` macro but returns `(class_name, style_val)` instead of saving the style_val to the separate file.

## Installtion
```cargo add stylers```

## Leptos Example
### style!
```rust
#[component]
fn Hello(cx: Scope, name: &'static str) -> impl IntoView {
    let styler_class = style! {"Hello",
        #two{
            color: blue;
        }
        div.one{
            color: red;
            content: raw_str(r#"\hello"#);
            font: "1.3em/1.2" Arial, Helvetica, sans-serif;
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
    };

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
### style_sheet!
```rust
#[component]
fn Hello(cx: Scope, name: &'static str) -> impl IntoView {
    let class_name = style_sheet!("./hello.css");
    view! {cx, class = class_name,
        <div class="one">
            <h1 id="two">"Hello"</h1>
            <h2>"World"</h2>
        </div>
    }
}
```
- In this case you should be place the ```hello.css``` file inside the `root` directory of the project.

### style_str!
- Note that in `style_str!` macro we don't need to pass the component name as we do in `style!` macro.
```rust
#[component]
pub fn GreenButton(cx: Scope) -> impl IntoView {
    let (class_name, style_val) = style_str! {
        button {
            background-color: green;
            border-radius: 8px;
            border-style: none;
            box-sizing: border-box;
            color: yellow;
            cursor: pointer;
            display: inline-block;
            font-family: r#"Haas Grot Text R Web"#, r#"Helvetica Neue"#, Helvetica, Arial, sans-serif;
            font-size: 14px;
            font-weight: 500;
            height: 40px;
            line-height: 20px;
            list-style: none;
            margin: 0;
            outline: none;
            padding: 10px 16px;
            position: relative;
            text-align: center;
            text-decoration: none;
            transition: color 100ms;
            vertical-align: baseline;
            user-select: none;
            -webkit-user-select: none;
        }
        button:hover{
            background-color: yellow;
            color: green;
        }
    };

    view! {cx, class = class_name,
        <style>{style_val}</style>
        <button>"I am green button"</button>
    }
}
```

### style_sheet_str!
```rusr
#[component]
pub fn BlueButton(cx: Scope) -> impl IntoView {
    let (class_name, style_val) = style_sheet_str!("./src/button.css");

    view! {cx, class = class_name,
        <style>{style_val}</style>
        <button>"I am blue button"</button>
    }
}
```
- In this case you should be place the ```button.css``` file inside the `src` directory of the project.

## How it works:

- Both `style!` and `style_sheet!` macros generate a css file with the given name inside the `./target/stylers/css` directory.
- For e.g. below code generates mystyle.css in `./target/stylers/css` directory and also generates one combined `./target/stylers/main.css` with all css files.
```rust
style!{"mystyle",
    h2 {
        color: green;
    }
}
```

## Edge cases handled for `style` and `style_sheet` macros
- By default double quotes ( " ) around css property values will be removed. If user wants to retain the double quotes they have to wrap it using ```raw_str``` as given below:
- these rules apply for both `style` and `style_sheet` macros
#### Input
```rust
style!(
    div{
        content: raw_str(r#"\hello"#);
        font: "1.3em/1.2" Arial;
    }
)
```
#### Output
```css
    div.l-23432{
        content: "\hello";
        font: 1.3em/1.2 Arial;
    }
```

## Optional build process using Trunk(Only when you use `style` or `style_sheet` macro )
- You have to include generated main.css in the index.html
(e.g ```<link rel="stylesheet" href="/main.css">```).

- In ```Trunk.toml``` you have to add the below lines to move the the `main.css` file from `./target/stylers/` directory to `./dist/` directory.
```toml
[[hooks]]
stage = "post_build"
command = "sh"
command_arguments = ["-c", "cp ./target/stylers/main.css $TRUNK_STAGING_DIR/"]
```
- if something is odd with styling, delete the `./target/stylers` directory and rebuild your package. If the problem persists please raise an issue here.

