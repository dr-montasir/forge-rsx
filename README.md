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
rsx!(btfy0, div { "No indentation" });
rsx!(btfy2, div { "Indented with 2 spaces" });
rsx!(btfy4, div { "Indented with 4 spaces" });
```

---

## Examples

### Basic Tag with Content

```rust
let greeting = rsx!(lined, span { "Hello, World!" });
println!("{}", greeting);
```

### Nested Tags

```rust
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
let button_html = rsx!(lined, button {
    class: "btn-primary",
    "data-toggle": "modal",
    "Click me!"
});
println!("{}", button_html);
```

### Loop Example

```rust
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
    let apple = "🍎 Apple";
    let apple_component = rsx!(lined, span { {&apple} });
    let fruits = vec!["🍇", "mango", "orange"];
    let div = rsx!(btfy4, div { "..." 
        {"<!--  How to use attributes with hyphens, like x-show in Alpine.js -->"}
        span {
            id: "my-id",
            class: "my-class",
            "x-show": "",
            ":class": "p-4",
            "..."
        }
        "..."
    });
    let span = rsx!(btfy0, span { "..." });
    let empty_p = rsx!(btfy2, p { });
    let p = rsx!(btfy2, p {"..."});
    let section = rsx!(btfy4, section { div { ol { 
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
    } } });
    println!(
        "{}\n\n{}\n\n{}\n\n{}\n\n{}", 
        div, span, empty_p, p, section
    );
    // <div>
    //     ...
    //     <!--  How to use attributes with hyphens, like x-show in Alpine.js -->
    //     <span id="my-id" class="my-class" x-show='' :class='p-4'>
    //         ...
    //     </span>
    //     ...
    // </div>
    //
    // <span>
    // ...
    // </span>
    //
    // <p></p>
    //
    // <p>
    //   ...
    // </p>
    //
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

