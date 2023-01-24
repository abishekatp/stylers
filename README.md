# Stylers
- Scoped CSS for Rust web frameworks like Leptos.
- style! macro validates css properties as well.

### Installtion
```cargo add stylers```

## Leptos Example
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

## How it works:

- Style macro generates a css file with the given name inside the css directory.
- For e.g. below code generates mystyle.css in ./css directory and also generates
one combined main.css with all css files.
```rust
style!{"mystyle",
    h2 {
        color: green;
    }
}
```

## Edge cases
- By default double quotes ( " ) around css property values will be removed. If user wants to retain the double quotes they have to wrap it using ```raw_str``` as given below:
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

## Optional build process using Trunk
- You have to include generated main.css in the index.html
(e.g ```<link data-trunk rel="css" href="./main.css">```).

- In ```Trunk.toml``` you have to add the below lines to prevent infinite loop
```toml
[watch]
ignore = ["./css"]
```
- if something is odd, delete the css directory and rebuild your package. If the problem persists please raise an issue here.

