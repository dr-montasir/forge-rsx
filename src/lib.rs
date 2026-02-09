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
///     let empty_p = rsx!(btfy2, p { });
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
///                                     &format!("{} {}", fruit.to_string(), "Grapes")
///                                 } else if fruit == &"mango" {
///                                     &format!("{} {}", "🥭", fruit.to_lowercase())
///                                 } else {
///                                     &fruit.to_uppercase()
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