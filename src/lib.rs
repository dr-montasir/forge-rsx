/// ### Rules Module
///
/// A module that encapsulates the rules and functionalities of the `rsx` macro.
///
/// A macro to generate HTML-like markup with different indentation styles.
/// 
/// Usage:
/// ```rust
/// use forge_rsx::{rsx, get_char};
/// fn main() {
///    let apple = "🍎 Apple";
///    let apple_component = rsx!(lined, span { {&apple} });
///    let fruits = vec!["🍇", "mango", "orange"];
///    let div = rsx!(btfy4, div { "..." 
///        {"<!--  How to use attributes with hyphens, like x-show in Alpine.js -->"}
///        span {
///            id: "my-id",
///            class: "my-class",
///            "x-show": "",
///            ":class": "p-4",
///            "..."
///        }
///        "..."
///     });
///     let span = rsx!(btfy0, span { "..." });
///     let empty_p = rsx!(btfy2, p { });
///     let p = rsx!(btfy2, p {"..."});
///     let section = rsx!(btfy4, section { div { ol { 
///         for fruit in &fruits => {
///             li {
///                 span {
///                     {
///                         if fruit == &"🍇" {
///                             &format!("{} {}", fruit.to_string(), "Grapes")
/// 
///                         } else if fruit == &"mango" {
///                             &format!("{} {}", "🥭", fruit.to_lowercase())
///                         } else {
///                             &fruit.to_uppercase()
///                         }
///                     }
///                 }
///             }
///         }
///         li { 
///             {"<!-- How to join RSX component -->"}
///             {&apple_component.to_string()} 
///             {
///                 if get_char(&apple, 1).to_string() == "🍎" {
///                     "🍎".to_string()
///                 } else {
///                     apple_component.to_string()
///                 }
///             }
///         }
///     } } });
///     println!(
///         "{}\n\n{}\n\n{}\n\n{}\n\n{}", 
///         div, span, empty_p, p, section
///     );
///     // Output:
///     // <div>...<span id="my-id" class="my-class" x-show='' :class='p-4'>...</span>...</div>
///     // 
///     // <span>
///     // ...
///     // </span>
///     //
///     // <p></p>
///     //
///     // <p>
///     //   ...
///     // </p>
///     //
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