# forge-rsx

`forge-rsx` is a Rust macro library for declarative, JSX-like HTML generation. It allows you to write HTML structures with embedded logic, attributes, nested tags, loops, and more, directly in Rust code with a concise syntax.

---

## Features

- Declarative HTML macro: `rsx!` macro
- Supports nested tags, attributes, loops, and embedded expressions
- Indentation-aware formatting
- String literal and identifier attributes
- Flexible syntax for defining complex HTML structures

---

## Usage

Run the following Cargo `command` in your project directory:

```shell
cargo add forge-rsx
```

Or add `forge-rsx` as a dependency in your `Cargo.toml`:

```toml
[dependencies]
forge-rsx = "MAJOR.MINOR.PATCH" # Replace with the latest version
```

In your Rust code, import the macro:

```rust
use forge_rsx::rsx;
```

---

## Macro Variants

- `lined`: produces HTML without indentation or line breaks (single-line output)
- `btfy0`: uses 0 spaces (no indentation, minified output)
- `btfy2`: Indentation with 2 spaces per level.
- `btfy4`: Indentation with 4 spaces per level.

### Examples:

```rust
use forge_rsx::rsx;

rsx!(btfy0, div { "No indentation" });
rsx!(btfy2, div { "Indented with 2 spaces" });
rsx!(btfy4, div { "Indented with 4 spaces" });
```

---

## Examples

### Basic Tag with Content

```rust
use forge_rsx::rsx;

// 1. First style: using parentheses => ()
let greeting1 = rsx!(lined, span { "Hello, World!" });
println!("{}", greeting1); // output: <span>Hello, World!</span>

// 2. Second style: using braces => {}
let greeting2 = rsx! { lined, span { "Hello, World!" } };
println!("{}", greeting2); // output: <span>Hello, World!</span>
```

### Nested Tags

```rust
use forge_rsx::rsx;

let nested_html = rsx!(lined, div {
    header {
        span { "Navigation" }
    }
    section {
        p { "Welcome to the forge-rsx template!" }
    }
});
println!("{}", nested_html);
```

### Attributes

```rust
use forge_rsx::rsx;

fn main () {
    let button_html = rsx! { lined, 
        button {
            class: "btn-primary",
            "data-toggle": "modal",
            "Click me!"
    	}
    };
    println!("{}", button_html);
    // output:
    // <button class="btn-primary" data-toggle="modal">Click me!</button>
    
    let is_loading = true;
	let is_admin = false;

	let script_html = rsx!(lined, 
        script {
             defer: true,
             async: is_loading, // true
             src: "https://example.com/app.js",
             hidden: is_admin   // false (won't show up)
         }
    );
    println!("{}", script_html);
    // Output: <script defer async src="https://example.com/app.js"></script>
}
```

### Loop Example

```rust
use forge_rsx::rsx;

let users = vec!["Ahmed", "Mohamed", "Montasir"];

let list_html = rsx!(lined, ul {
    for user in &users => {
        li { { format!("User: {}", user) } }
    }
});
println!("{}", list_html);
```

### Full Complex Example

```rust
use forge_rsx::{rsx, get_char};

fn main() {
    // 1. Component defined with 'lined' (minified single line)
    let apple = "🍎 Apple";
    let apple_component = rsx!(lined, span { {&apple} });

    // 2. Full HTML Document using 'doctype_html' and 'btfy4'
    let html_doc = rsx! {
        btfy4,
        doctype_html
        html {
            head {
                meta { charset: "UTF-8" } // No comma needed here
                meta { name: "viewport", content: "width=device-width, initial-scale=1.0" }
                title { "Forge RSX Demo" }
            }
            body {
                "x-data": "{ open: false }", ":class": "bg-white",
                h1 { "Welcome to Forge RSX" }
                br {}
                div { 
                    class: "container",
                    "x-show": "open",
                    "Alpine.js integration demo"
                }
                "id": "my-id", "style": "color: #4f4f4f; font-size: 2rem;" // No comma needed here
            }
        }
    };

    // 3. Formatting Samples: Demonstrating 0 and 2-space indentation styles
    let span = rsx!(btfy0, span { "..." });
    let empty_p = rsx!(btfy2, p { });
    let p = rsx!(btfy2, p {"..."});

    // 4. Complex section with 'for' loops and logic
    let fruits = vec!["🍇", "mango", "orange"];
    let section = rsx!(btfy4, section { 
        div { 
            ol { 
                for fruit in &fruits => {
                    li {
                        span {
                            {
                                if fruit == &"🍇" {
                                    &format!("{} {}", fruit.to_string(), "Grapes")
                                } else if fruit == &"mango" {
                                    &format!("{} {}", "🥭", fruit.to_lowercase())
                                } else {
                                    &fruit.to_uppercase()
                                }
                            }
                        }
                    }
                }
                li { 
                    {"<!-- How to join RSX component -->"}
                    {&apple_component.to_string()} 
                    {
                        if get_char(&apple, 1).to_string() == "🍎" {
                            "🍎".to_string()
                        } else {
                            apple_component.to_string()
                        }
                    }
                }
            } 
        } 
    });

    // 5. Printing all results
    println!("--- FULL HTML DOCUMENT ---\n{}\n", html_doc);
    println!("--- MINIFIED SPAN ---\n{}\n", span);
    println!("--- EMPTY P ---\n{}\n", empty_p);
    println!("--- P WITH CONTENT ---\n{}\n", p);
    println!("--- FRUIT SECTION ---\n{}", section);

    // --- FULL HTML DOCUMENT ---
    // <!DOCTYPE html>
    // <html>
    //     <head>
    //         <meta charset="UTF-8">
    //         <meta name="viewport" content="width=device-width, initial-scale=1.0">
    //         <title>
    //             Forge RSX Demo
    //         </title>
    //     </head>
    //     <body x-data='{ open: false }' :class='bg-white' id="my-id" style="color: #4f4f4f; font-size: 2rem;">
    //         <h1>
    //             Welcome to Forge RSX
    //         </h1>
    //         <br>
    //         <div class="container" x-show='open'>
    //             Alpine.js integration demo
    //         </div>
    //     </body>
    // </html>

    // --- MINIFIED SPAN ---
    // <span>
    // ...
    // </span>

    // --- EMPTY P ---
    // <p></p>

    // --- P WITH CONTENT ---
    // <p>
    // ...
    // </p>

    // --- FRUIT SECTION ---
    // <section>
    //     <div>
    //         <ol>
    //             <li>
    //                 <span>
    //                     🍇 Grapes
    //                 </span>
    //             </li>
    //             <li>
    //                 <span>
    //                     🥭 mango
    //                 </span>
    //             </li>
    //             <li>
    //                 <span>
    //                     ORANGE
    //                 </span>
    //             </li>
    //             <li>
    //                 <!-- How to join RSX component -->
    //                 <span>🍎 Apple</span>
    //                 🍎
    //             </li>
    //         </ol>
    //     </div>
    // </section>
}
```

---

## License

MIT License

---

## Notes

- The macro supports attributes with string literals and identifiers.
- Nested tags are handled with recursive macro calls.
- Looping constructs generate repeated content.
- Content inside braces `{}` can contain any Rust expression that implements `Display`.

---

