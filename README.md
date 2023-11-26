# Stylers
- Scoped CSS for Rust web frameworks like Leptos.
- `style!` macro is for writing css inside rust functions directly. It will validates css properties as well.
- `style_sheet!` macro is for writing css in external css file and importing that inside rust functions.
- `style_str!` macro is same as `style!` macro but returns the tuple `(class_name, style_val)` instead of saving the style_val to the separate file.
- `style_sheet_str!` this same as `style_sheet!` macro but returns the tuple `(class_name, style_val)` instead of saving the style_val to the separate file.

## Important Note
- This Readme file is for the latest relase of stylers 1.0.0-alpha. You can find the readme for previous versions [here](https://crates.io/crates/stylers/0.3.2)

## Installtion
```cargo add stylers```

## Prerequisite
- If you are using `style` or `style_sheet` macro, then you have to add the `stylers` crate as both dependencies and build-dependencies in your Cargo.toml file.
```
[dependencies]
stylers = { version = "*" }

[build-dependencies]
stylers = {  version = "*" }
```

- Then you have to add `build.rs` file in your root directory and add the below code snippet in it.
```rust
use stylers::build;

fn main() {
    build(Some(String::from("./target/main.css")));
}
```
- In the above case output css file will be generated in the `./target/main.css` path. You can include that `main.css` file in your `index.html` file.(**Or If you are using a build like Trunk.rs you have to follow appropriate methods to include the main.css file to your project**).

You can find the importance of these new changes [here](https://github.com/abishekatp/stylers/issues/35).

## Leptos Example
**Note :Leptos version > 0.4.9 has some new changes. But stylers works the same way in all versions of leptos**

#### style!
```rust
#[component]
fn Hello(cx: Scope, name: &'static str) -> impl IntoView {
    let styler_class = style! {
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

    view! {class = styler_class,
        <div class="one">
            <h1 id="two">"Hello"</h1>
            <h2>"World"</h2>
            <h2>{name}</h2>
            <h3>"Hello Kanna"</h3>
        </div>
    }
}
```
#### style_sheet!
```rust
#[component]
fn Hello(cx: Scope, name: &'static str) -> impl IntoView {
    let class_name = style_sheet!("./hello.css");
    view! {class = class_name,
        <div class="one">
            <h1 id="two">"Hello"</h1>
            <h2>"World"</h2>
        </div>
    }
}
```
- In the above case ```hello.css``` file is inside the `root` directory of the project.

#### style_str!
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

    view! {class = class_name,
        <style>{style_val}</style>
        <button>"I am green button"</button>
    }
}
```

#### style_sheet_str!
```rust
#[component]
pub fn BlueButton() -> impl IntoView {
    let (class_name, style_val) = style_sheet_str!("./src/button.css");

    view! {class = class_name,
        <style>{style_val}</style>
        <button>"I am blue button"</button>
    }
}
```
- In this case ```button.css``` file is inside the `src` directory of the project.

## Custom pseudo classes
- In some situations we may need our css to affect `deep down` the dom tree. To achieve this we have custom pseudo class called `:deep()`. For example below css is valid one.
#### Input
```css
div :deep(h3) {
    color: orange;
}
```
#### Output
```css
div.l-243433 h3{color: orange;}
```

- If you want your particular css to be `global` you can use `:deep()` directive without any  prceding selectors it.
#### Input
```css
:deep(h3 div) {
    color: orange;
}
```
#### Ouput
```css
h3 div{color: orange;}
```

## How it works:
- This `stylers::build` method will parse all the rust files in the path `/src/**/*.rs` during build step to find the places the `style` and `style_sheet` macros has been used and generate single output css file.
- `style_str` and `style_sheet_str` directly returns the tuple (class_name, output_css).


## Edge cases handled for `style!` macros
- By default double quotes ( " ) around css property values will be removed. If user wants to retain the double quotes they have to wrap it using ```raw_str``` as given below:
- these rules apply for both `style!` and `style_str!` macros
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

## Optional build process using Trunk(Only when you use `style!` or `style_sheet!` macro )
- You have to include generated main.css in the index.html
(e.g ```<link rel="stylesheet" href="/main.css">```).

- In ```Trunk.toml``` you have to add the below lines to move the the `main.css` file from `./target/` directory to `./dist/` directory.
```toml
[[hooks]]
stage = "post_build"
command = "sh"
command_arguments = ["-c", "cp ./target/main.css $TRUNK_STAGING_DIR/"]
```
- when you are including external css file using `style_sheet!` macro, whenever you make some changes in your css file you have to save corresponding rust file(*.rs) for css to be updated on the browser. For more info about trunk refer [here](https://trunkrs.dev/commands/).
- if something is odd with styling, delete the `./target/stylers` directory and rebuild your package. If the problem persists please raise an issue here.

