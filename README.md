<div align="center">
  <br>
  <a href="https://github.com/dr-montasir/forge-rsx" target="_blank">
      <img src="https://raw.githubusercontent.com/dr-montasir/forge-rsx/refs/heads/main/logo.svg" width="100">
  </a>
  <br>
  <h1>FORGE-RSX</h1>
</div>

> `forge-rsx` is a Rust macro library for declarative, JSX-like HTML generation. It allows you to write HTML structures with embedded logic, attributes, nested tags, loops, and more, directly in Rust code with a concise syntax.

<div style="text-align: center;">
  <a href="https://github.com/dr-montasir/forge-rsx"><img src="https://img.shields.io/badge/github-dr%20montasir%20/%20forge%20rsx%20-8da0cb?style=for-the-badge&labelColor=555555&logo=github" height="24" style="margin-top: 10px;" alt="github" /></a> <a href="https://crates.io/crates/forge-rsx"><img src="https://img.shields.io/crates/v/forge-rsx.svg?style=for-the-badge&color=fc8d62&logo=rust" height="24" style="margin-top: 10px;" alt="crates.io"></a> <a href="https://docs.rs/forge-rsx"><img src="https://img.shields.io/badge/docs.rs-forge%20rsx-66c2a5?style=for-the-badge&labelColor=555555&logo=docs.rs" height="24" style="margin-top: 10px;" alt="docs.rs"></a> <a href="https://choosealicense.com/licenses/mit"><img src="https://img.shields.io/badge/license-mit-4a98f7.svg?style=for-the-badge&labelColor=555555" height="24" style="margin-top: 10px;" alt="license"></a> <a href="https://crates.io/crates/forge-rsx" target="_blank"><img alt="downloads" src="https://img.shields.io/crates/d/forge-rsx.svg?style=for-the-badge&labelColor=555555&logo=&color=428600" height="22"></a>
</div>

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
- `tabed`: Indentation with 2 spaces per level. (tabed = btfy2)

### Examples:

```rust
use forge_rsx::rsx;

rsx!(btfy0, div { "No indentation" });
rsx!(btfy2, div { "Indented with 2 spaces" });
rsx!(tabed, div { "Indented with 2 spaces" });
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
// This example demonstrates:
// 1. Attribute quoting: Forge-RSX uses double quotes. Use 
//    single quotes inside JS/Alpine strings to maintain HTML validity.
// 2. Boolean attributes: Attributes assigned `true` render as keys only. 
//    Attributes assigned `false` are omitted.
// 3. Formatting: The `lined` mode creates compact HTML without extra whitespace.

use forge_rsx::rsx;

fn main () {
    let button_html = rsx! { lined, 
        button {
            class: "btn-primary",
            "data-toggle": "modal",  // Alpine.js
            // ❌ INCORRECT: The "User" double quotes will terminate the attribute early.
            "hx-on:click": r#"const name="User"; alert(`Hello, ${name}!`)"#, // HTMX: incorrect output
            // ✅ CORRECT: Using single quotes 'User' keeps the attribute valid.
            "hx-on:click": "const name='User'; alert(`Hello, ${name}!`)", // HTMX: correct output
            // "hx-on:click": r#"const name='User'; alert(`Hello, ${name}!`)"# // HTMX: correct output
            "Click me!"
    	}
    };
    println!("{}", button_html);
    // output:
    // <button class="btn-primary" data-toggle="modal" hx-on:click="const name="User"; alert(`Hello, ${name}!`)" hx-on:click="const name='User'; alert(`Hello, ${name}!`)">Click me!</button>
    
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

### Code Tag

```rust
use forge_rsx::rsx;

fn main () {
    // The raw data
    let code_source = "fn main() {\n    println!(\"Hello\");\n}";

    // The specific HTML structure for the code
    let code_snippet = rsx!(lined,
        pre {
            code { 
                {code_source}
            }
        }
    );
    
    // The final container (lined)
    let final_html_lined = rsx!(lined,
        div {
            {code_snippet}
        }
    );
    println!("{}", final_html_lined);
    
    // (lined lined) output:
    // <div><pre><code>fn main() {
    //     println!("Hello");
    // }</code></pre></div>
    
    // Browser result:
    // fn main() {
    //     println!("Hello");
    // }

    // The final container (btfy0)
    let final_html_btfy0 = rsx!(btfy0,
        div {
            {code_snippet}
        }
    );
    println!("{}", final_html_btfy0);
    
    // (lined btfy0) output:
    // <div>
    // <pre><code>fn main() {
    //     println!("Hello");
    // }</code></pre>
    // </div>
    
    // Browser result:
    // fn main() {
    //     println!("Hello");
    // }

    // The final container (btfy4)
    let final_html_btfy4 = rsx!(btfy4,
        div {
            {code_snippet}
        }
    );
    println!("{}", final_html_btfy4);

    // (lined btfy4) output:
    // <div>
    //     <pre><code>fn main() {
    //     println!("Hello");
    // }</code></pre>
    // </div>

    // Browser result:
    // fn main() {
    //     println!("Hello");
    // }
}
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
                                    format!("{} {}", fruit.to_string(), "Grapes")
                                } else if fruit == &"mango" {
                                    format!("{} {}", "🥭", fruit.to_lowercase())
                                } else {
                                    fruit.to_uppercase()
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
    //     <body x-data="{ open: false }" :class="bg-white" id="my-id" style="color: #4f4f4f; font-size: 2rem;">
    //         <h1>
    //             Welcome to Forge RSX
    //         </h1>
    //         <br>
    //         <div class="container" x-show="open">
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

## HTML Processing Utilities

`forge-rsx` exposes a standalone, zero-dependency processing utility `minify` to optimize raw, unstructured, or dynamically generated HTML strings into high-performance, single-line layouts.

### HTML Minification (`minify`)

The `minify` function compresses raw string slices by dropping structural formatting rules, newlines, and loose layout spaces to ensure an optimized continuous string output. 

The parsing engine relies on a smart state-tracking sequence that skips compression rules when encountering `<pre>` and `<code>` blocks. This guarantees that code structures, alignment, text spacing, and newlines wrapped inside targeted syntax elements remain perfectly intact and completely unaltered.

#### Signature

```text
pub fn minify(html: &str) -> String
```

#### Examples

##### 1. Collapsing Layout Gaps (Minification)

Passing an unstructured or formatted HTML string stream compresses all extra white spaces into a true single-line payload sequence:

```rust
use forge_rsx::minify;

let structured_input = r####"
    <div>
        <h1>Hello World!</h1>
    </div>
"####;

let compressed_output = minify(structured_input);
assert_eq!(compressed_output, "<div> <h1>Hello World!</h1> </div>");
```

##### 2. Compress a multi-line HTML block into a single line while keeping inline spaces

```rust
use forge_rsx::minify;

let raw_input = r####"
    <div>
        <div>
            <span>copyright 2026</span>
            <a href="/">https://crates.io/crates/forge-rsx</a>
        </div>
    </div>
"####;

let minified = minify(raw_input);
assert_eq!(minified, r#"<div> <div><span>copyright 2026</span> <a href="/">https://crates.io/crates/forge-rsx</a> </div></div>"#);
```

##### 3. 3. Compress a multi-line HTML block into a single line while keeping inline spaces and comments

```rust
use forge_rsx::minify_with_comments;

let raw_input = r####"
    <div>
        <!-- Copyright -->
        <div>
            <span>copyright 2026</span>
            <a href="/">https://crates.io/crates/forge-rsx</a>
        </div>
    </div>
"####;

let minified = minify_with_comments(raw_input);
assert_eq!(minified, r#"<div> <!-- Copyright --><div><span>copyright 2026</span> <a href="/">https://crates.io/crates/forge-rsx</a> </div></div>"#);
```

##### 4. Preserving Formatted Syntax Blocks

The utility automatically detects nested code structures and leaves structural text layouts inside `<pre>` and `<code>` blocks fully untouched:

```rust
use forge_rsx::minify;

let raw_stream = r####"<div>
    <div>
<pre><code>fn main() {
    let mut app = WebIo::new();
    println!("WebIO running!");
}</code></pre>
    </div>
</div>"####;

let result = minify(raw_stream);

assert_eq!(
    result,
    "<div> <div><pre><code>fn main() {\n    let mut app = WebIo::new();\n    println!(\"WebIO running!\");\n}</code></pre> </div></div>"
);
```

##### 5. Frontend Frameworks Safety (such as Alpine.js, HTMX, etc.)

> [!WARNING]
> **Do not use single-line JavaScript comments (`//`) inside raw template attributes.**
> Since the minifier collapses structural layouts down into a single line, any trailing or inline `//` token will comment out the remaining contents of that HTML block, causing breaking bugs. 

Always use standard JavaScript block comments (`/* ... */`) or extract heavy scripts into separate files instead:

```rust
use forge_rsx::minify;

// ❌ DANGEROUS: Single-line comment comments out subsequent tags after minification
let broken_alpine = r#"<div x-init="console.log('init'); // Single-line comment breaks here "><p>Hidden</p></div>"#;

// 🟢 SAFE: Use JavaScript block comments inside HTML attribute scopes
let safe_alpine = r#"<div x-init="console.log('init'); /* Safe block comment context */ "><p>Visible</p></div>"#;

let result = minify(safe_alpine);
assert!(result.contains("<p>Visible</p>"));
```

---

## Development vs. Production: Working with `<pre>` & `<code>`

When building code blocks or developer documentation with `forge-rsx`, it is important to understand how the macro styling rules (`btfy2`, `btfy4`, etc.) interact with the runtime minifier functions (`minify`, `minify_with_comments`).

### * During Development (Macro Layer)

The macro indentation rules (`btfy0` through `btfy4` and `tabed`) are designed to organize the final layout string so the structural tree can be debugged easily.

- **No Macro Escaping:** The macro **does not** modify or escape characters like `<`, `>`, or `&` into HTML entities at compile time. It keeps them as raw characters.
- **Frontend Compatibility:** This behavior is intentional. If you use client-side syntax highlighters like **Highlight.js** or **Prism.js**, they often expect raw code formats inside code blocks to parse tokens correctly using JavaScript or CSS.

### In Production (Minifier Layer)

When your project is ready for production, you should pass your macro output string directly through the `minify` or `minify_with_comments` functions.

```rust
use forge_rsx::{rsx, minify};

// Recommended Production Workflow (minify: lined, btfy2, btfy4, etc.)
let raw_html = rsx!(lined, div { "..." });
let production_html = forge_rsx::minify(&raw_html);
```

#### Full Usage Example with Code Blocks

```rust
use forge_rsx::{rsx, minify};

fn main() {
    let code_source = r##"fn main() {
    	println!("<h1>Hello World!</h1>");
    }"##;
    
    // 1. Generate structural HTML layout using the macro
    let raw_html = rsx!(lined, div {
        h2 { "Rust Example" }
        pre { class: "code-demo__block",
            code {
                {code_source}
            }
        }
    });

    // 2. Compress and optimize for production in a single pass
    let production_html = minify(&raw_html);
    
    // Hand over `production_html` to your web framework response!
}
```

- **Automated Optimization:** The `minify_impl` engine automatically handles the transition for you. It safely tracks nested `<pre><code>` blocks, compresses whitespace properly, and escapes control characters (`<` to `<`, etc.) to prevent layout breaks.
- **Flawless Integration:** Even after minification and character replacement, code boxes will still render beautifully on screen and remain fully compatible with syntax highlighters like **Highlight.js**.

### Architectural Choice: Why Escaping Happens at the Function Level (Not the Macro)

The choice to handle HTML escaping inside the function level rather than inside the `rsx!` macro rules during compilation maximizes runtime performance, ensures code flexibility, and prevents redundant character transformation loops.

1. Avoiding Double-Escaping Redundancy

   If the macro handles escaping at compile time, it transforms a character like `<` into the HTML entity `<`.
   When that generated string is passed into a global utility like `minify` or `minify_with_comments` later, the minifier scans the entire string character-by-character. It encounters the standalone `&` symbol inside the already-escaped `<` and transforms it a second time, producing `&lt;`.

   While some modern frontend engines handle this automatically, it introduces a duplicate parsing cycle. Keeping the macro layer raw guarantees that content translates **exactly once** at the function level without any redundant modifications.

2. CPU and Runtime Performance

   Declarative macros (`macro_rules!`) are highly optimized for structural layout token mapping. Forcing them to conditionally evaluate variables at runtime introduces unnecessary performance costs:

   * **Repeated Allocations:** Using custom utility wrappers inside macro arms forces the CPU to constantly evaluate conditions (e.g., checking `if parent == "code"`) and repeatedly trigger runtime allocations (`String::with_capacity` and `format!`) on every single block component.
   * **The "One-Pass" Optimization:** By relying entirely on `minify()` or `minify_with_comments()` for production builds, the server processes the entire page layout in **a single, sequential pass**. It cleans up spacing, compresses code, and escapes code blocks simultaneously inside a highly efficient loop, minimizing overall CPU usage per HTTP request.
   * Frontend Independence

3. Different frontend stacks treat code blocks differently. Keeping the macro layer lightweight and agnostic ensures that applications requiring raw, unescaped character outputs (for example, specific client-side JavaScript syntax engines or unique custom CSS setups) can use the macro on its own without being forced into an irreversible character-mapping pipeline.

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

