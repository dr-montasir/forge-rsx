#![doc = include_str!("../README.md")]

/// ### Rules Module
///
/// A module that encapsulates the rules and functionalities of the `rsx` macro.
///
/// A macro to generate HTML-like markup with different indentation styles.
/// 
/// Usage:
/// ```rust
/// use forge_rsx::{rsx, get_char};
/// 
/// fn main() {
///     // 1. Component defined with 'lined' (minified single line)
///     let apple = "🍎 Apple";
///     let apple_component = rsx!(lined, span { {&apple} });
/// 
///     // 2. Full HTML Document using 'doctype_html' and 'btfy4'
///     let html_doc = rsx! {
///         btfy4,
///         doctype_html
///         html {
///             head {
///                 meta { charset: "UTF-8" } // No comma needed here
///                 meta { name: "viewport", content: "width=device-width, initial-scale=1.0" }
///                 title { "Forge RSX Demo" }
///             }
///             body {
///                 "x-data": "{ open: false }", ":class": "bg-white",
///                 h1 { "Welcome to Forge RSX" }
///                 br {}
///                 div { 
///                     class: "container",
///                     "x-show": "open",
///                     "Alpine.js integration demo"
///                 }
///                 "id": "my-id", "style": "color: #4f4f4f; font-size: 2rem;" // No comma needed here
///             }
///         }
///     };
/// 
///     // 3. Formatting Samples: Demonstrating 0 and 2-space indentation styles
///     let span = rsx!(btfy0, span { "..." });
///     let empty_p = rsx!(tabed, p { });
///     let p = rsx!(btfy2, p {"..."});
/// 
///     // 4. Complex section with 'for' loops and logic
///     let fruits = vec!["🍇", "mango", "orange"];
///     let section = rsx!(btfy4, section { 
///         div { 
///             ol { 
///                 for fruit in &fruits => {
///                     li {
///                         span {
///                             {
///                                 if fruit == &"🍇" {
///                                     format!("{} {}", fruit.to_string(), "Grapes")
///                                 } else if fruit == &"mango" {
///                                     format!("{} {}", "🥭", fruit.to_lowercase())
///                                 } else {
///                                     fruit.to_uppercase()
///                                 }
///                             }
///                         }
///                     }
///                 }
///                 li { 
///                     {"<!-- How to join RSX component -->"}
///                     {&apple_component.to_string()} 
///                     {
///                         if get_char(&apple, 1).to_string() == "🍎" {
///                             "🍎".to_string()
///                         } else {
///                             apple_component.to_string()
///                         }
///                     }
///                 }
///             } 
///         } 
///     });
/// 
///     // 5. Printing all results
///     println!("--- FULL HTML DOCUMENT ---\n{}\n", html_doc);
///     println!("--- MINIFIED SPAN ---\n{}\n", span);
///     println!("--- EMPTY P ---\n{}\n", empty_p);
///     println!("--- P WITH CONTENT ---\n{}\n", p);
///     println!("--- FRUIT SECTION ---\n{}", section);
/// 
///     // --- FULL HTML DOCUMENT ---
///     // <!DOCTYPE html>
///     // <html>
///     //     <head>
///     //         <meta charset="UTF-8">
///     //         <meta name="viewport" content="width=device-width, initial-scale=1.0">
///     //         <title>
///     //             Forge RSX Demo
///     //         </title>
///     //     </head>
///     //     <body x-data="{ open: false }" :class="bg-white" id="my-id" style="color: #4f4f4f; font-size: 2rem;">
///     //         <h1>
///     //             Welcome to Forge RSX
///     //         </h1>
///     //         <br>
///     //         <div class="container" x-show="open">
///     //             Alpine.js integration demo
///     //         </div>
///     //     </body>
///     // </html>
/// 
///     // --- MINIFIED SPAN ---
///     // <span>
///     // ...
///     // </span>
/// 
///     // --- EMPTY P ---
///     // <p></p>
/// 
///     // --- P WITH CONTENT ---
///     // <p>
///     // ...
///     // </p>
/// 
///     // --- FRUIT SECTION ---
///     // <section>
///     //     <div>
///     //         <ol>
///     //             <li>
///     //                 <span>
///     //                     🍇 Grapes
///     //                 </span>
///     //             </li>
///     //             <li>
///     //                 <span>
///     //                     🥭 mango
///     //                 </span>
///     //             </li>
///     //             <li>
///     //                 <span>
///     //                     ORANGE
///     //                 </span>
///     //             </li>
///     //             <li>
///     //                 <!-- How to join RSX component -->
///     //                 <span>🍎 Apple</span>
///     //                 🍎
///     //             </li>
///     //         </ol>
///     //     </div>
///     // </section>
/// }
/// ```
/// 
/// - `lined`: produces HTML without indentation or line breaks (single-line output)
/// - `btfy0`: uses 0 spaces (no indentation, minified output)
/// - `btfy2`: uses 2 spaces indentation
/// - `btfy4`: uses 4 spaces indentation
pub mod rules;

/// Returns the character at the specified 1-based index `n` from the input string `s`.
///
/// If `n` exceeds the number of characters in `s`, the function returns an empty string.
///
/// # Arguments
///
/// * `s` - A string slice from which to extract a character.
/// * `n` - The 1-based position of the character to retrieve.
///
/// # Examples
///
/// ```rust
/// use forge_rsx::get_char;
/// let s = "Hello, forge-rsx!";
/// assert_eq!(get_char(s, 0), ""); // Out of bounds, returns empty
/// assert_eq!(get_char(s, 1), "H"); // First character
/// assert_eq!(get_char(s, 50), ""); // Out of bounds, returns empty
/// ```
///
/// Note: `n` starts at 1 for the first character.
pub fn get_char(s: &str, index: usize) -> String {
    if index == 0 || index > s.chars().count() {
        "".to_string()
    } else {
        // Convert 1-based index to 0-based
        let char_index = index - 1;
        // Get the char at the position
        s.chars().nth(char_index).unwrap().to_string()
    }
}

/// Minifies an HTML string slice into a compressed, single-line format while preserving formatting inside specific blocks.
///
/// This utility optimizes raw HTML markup by stripping out unneeded newlines, tab characters, and 
/// redundant white spaces, collapsing structural content down into a continuous line. 
///
/// The parsing engine tracks nested elements to ensure that all contents, text layouts, newlines, and 
/// spaces residing inside `<pre>` and `<code>` tags are left completely untouched. 
///
/// # Arguments
///
/// * `html` - A raw, multi-line, or unstructured HTML string slice to be parsed and compressed.
///
/// # Examples
///
/// ```rust
/// use forge_rsx::minify;
///
/// // 1. Compress a multi-line HTML block into a single line
/// let raw_input = r####"
///     <div>
///         <h1>Hello World!</h1>
///     </div>
/// "####;
/// let minified = minify(raw_input);
/// assert_eq!(minified, "<div> <h1>Hello World!</h1> </div>");
///
/// // 2. Compress a multi-line HTML block into a single line while keeping inline spaces
/// let raw_input = r####"
///     <div>
///         <div>
///             <span>copyright 2026</span>
///             <a href="/">https://crates.io/crates/forge-rsx</a>
///         </div>
///     </div>
/// "####;
/// let minified = minify(raw_input);
/// assert_eq!(minified, r#"<div> <div><span>copyright 2026</span> <a href="/">https://crates.io/crates/forge-rsx</a> </div></div>"#);
///
/// // 3. Preserve precise formatting inside pre and code tag blocks
/// let code_input = r####"
///     <div>
///         <div>
/// <pre><code>fn main() {
///     let mut app = WebIo::new();
///     println!("WebIO running!");
/// }</code></pre>
///         </div>
///     </div>
/// "####;
/// 
/// let result = minify(code_input);
/// assert_eq!(result, "<div> <div><pre><code>fn main() {\n    let mut app = WebIo::new();\n    println!(\"WebIO running!\");\n}</code></pre> </div></div>");
/// ```
pub fn minify(html: &str) -> String {
    let mut result = String::with_capacity(html.len());
    let mut chars = html.trim().chars().peekable();
    
    let mut in_pre_code: i32 = 0; 
    let mut last_was_whitespace = false;

    while let Some(c) = chars.next() {
        // Handle HTML Tags properly
        if c == '<' {
            let mut tag_buffer = String::from("<");
            let mut is_closing = false;

            if chars.peek() == Some(&'/') {
                is_closing = true;
                tag_buffer.push(chars.next().unwrap());
            }

            // Extract the full tag name safely
            let mut tag_name = String::new();
            while let Some(&next_c) = chars.peek() {
                if next_c.is_alphabetic() {
                    tag_name.push(chars.next().unwrap());
                } else {
                    break;
                }
            }
            tag_buffer.push_str(&tag_name);
            let tag_name_lower = tag_name.to_lowercase();
            let is_target_tag = tag_name_lower == "pre" || tag_name_lower == "code";

            // Consume everything until the tag completely closes '>'
            while let Some(next_c) = chars.next() {
                tag_buffer.push(next_c);
                if next_c == '>' {
                    break;
                }
            }

            // Track nesting levels of formatting-sensitive blocks (<pre> and <code>)
            // to dynamically toggle layout whitespace preservation mode.
            if is_target_tag {
                if is_closing {
                    in_pre_code = in_pre_code.saturating_sub(1);
                } else {
                    in_pre_code += 1;
                }
            }

            // Append the fully processed HTML tag string to the final result buffer.
            result.push_str(&tag_buffer);
            
            // Clear formatting layout boundaries inside fresh <pre> or <code> blocks.
            if is_target_tag && in_pre_code > 0 {
                while let Some(&next_c) = chars.peek() {
                    if next_c.is_whitespace() {
                        chars.next();
                    } else {
                        break;
                    }
                }
            }
            continue;
        }

        // Handle content inside / outside target tags
        if in_pre_code > 0 {
            result.push(c);
            last_was_whitespace = false;
        } else if c.is_whitespace() {
            if !last_was_whitespace {
                result.push(' ');
                last_was_whitespace = true;
            }
        } else {
            result.push(c);
            last_was_whitespace = false;
        }
    }

    // Strip terminal ending spaces
    while result.ends_with(' ') {
        result.pop();
    }

    result
}