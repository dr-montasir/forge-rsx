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

/// Formats code blocks by escaping spaces to &nbsp; and newlines to <br>.
/// It automatically trims single leading or trailing newlines to keep layout bounds clean.
pub fn format_code(input: &str) -> String {
    let mut working_str = input;
    if working_str.starts_with('\n') {
        working_str = &working_str[1..];
    }
    if working_str.ends_with('\n') {
        working_str = &working_str[..working_str.len() - 1];
    }

    let mut formatted_html = String::with_capacity(working_str.len() * 2);
    for ch in working_str.chars() {
        match ch {
            ' ' => formatted_html.push_str("&nbsp;"),
            '\n' => formatted_html.push_str("<br>"), 
            _ => formatted_html.push(ch),
        }
    }
    formatted_html
}

/// Beautifies or minifies an HTML string slice based on the provided indentation configuration.
///
/// This utility normalizes unstructured HTML markup into a uniform layout by collapsing extra white spaces. 
/// Processing relies on two structural modes depending on the `indent` value:
///
/// 1. **Beautification Mode (`indent >= 0`)**: Pads child markup layers using the designated space width.
///    The `<html>` element is processed as a root wrapper, allowing tags like `<head>` and `<body>` to remain left-aligned.
/// 2. **Minification Mode (`indent < 0`)**: Collapses the markup, discards formatting breaks, and returns a single-line string.
///
/// # Arguments
///
/// * `indent` - The indentation width pattern (`i8`). Positive integers set space width per depth level. Negative values drop layout margins entirely to trigger minification.
/// * `html` - A raw, unstructured, or single-line HTML string slice to be parsed.
///
/// # Examples
///
/// ```rust
/// use forge_rsx::btfy;
///
/// // 1. Beautify layout using 4 spaces
/// let messy_input = "<html><head><meta charset=\"utf-8\"></head><body><h1>Hi</h1></body></html>";
/// let beautified = btfy(4, messy_input);
/// assert_eq!(beautified, "<html>\n<head>\n    <meta charset=\"utf-8\">\n</head>\n<body>\n    <h1>\n        Hi\n    </h1>\n</body>\n</html>");
///
/// // 2. Minify layout using a negative index
/// let split_input = "<div>\n  <p>Hello World</p>\n</div>";
/// let minified = btfy(-1, split_input);
/// assert_eq!(minified, "<div><p>Hello World</p></div>");
/// ```
pub fn btfy(indent: i8, html: &str) -> String {
    // STEP 1: Linear, single-increment pass to clean HTML layout text safely
    let mut clean_html = String::new();
    let mut last_char_was_space = false;
    let mut inside_code = false;
    let chars_input: Vec<char> = html.chars().collect();
    let mut idx = 0;

    while idx < chars_input.len() {
        if chars_input[idx] == '<' {
            if idx + 5 < chars_input.len() && chars_input[idx+1..idx+5] == ['c', 'o', 'd', 'e'] {
                inside_code = true;
                clean_html = clean_html.trim_end().to_string();
            } else if idx + 6 < chars_input.len() && chars_input[idx+1..idx+6] == ['/', 'c', 'o', 'd', 'e'] {
                inside_code = false;
            }
        }

        let c = chars_input[idx];
        if inside_code {
            // Keep everything inside code blocks exactly intact
            clean_html.push(c);
            last_char_was_space = false;
        } else if c == '<' {
            clean_html = clean_html.trim_end().to_string();
            clean_html.push(c);
            last_char_was_space = false;
        } else if c == '>' {
            clean_html.push(c);
            last_char_was_space = false;
        } else if c.is_whitespace() {
            if !last_char_was_space && !clean_html.is_empty() && !clean_html.ends_with('>') {
                clean_html.push(' ');
                last_char_was_space = true;
            }
        } else {
            clean_html.push(c);
            last_char_was_space = false;
        }
        idx += 1; // Always steps forward precisely once
    }

    let is_minified = indent < 0;
    let actual_indent = indent.max(0) as usize; 

    // STEP 2: Structural layout formatting loop using flat state-tracking flags
    let mut result = String::new();
    let mut depth: usize = 0; 
    let mut i = 0;
    let chars: Vec<char> = clean_html.chars().collect();
    let indent_unit = " ".repeat(actual_indent);

    let void_tags = ["meta", "link", "br", "img", "input", "hr"];
    let ignored_tags = ["html", "code"]; 
    let mut format_blind_code_mode = false;

    while i < chars.len() {
        if chars[i] == '<' {
            let is_closing = i + 1 < chars.len() && chars[i + 1] == '/';
            let is_declaration = i + 1 < chars.len() && (chars[i + 1] == '!' || chars[i + 1] == '?');

            let mut tag_end = i;
            while tag_end < chars.len() && chars[tag_end] != '>' {
                tag_end += 1;
            }
            if tag_end >= chars.len() {
                // Safeguard against malformed closing patterns
                result.push_str(&chars[i..].iter().collect::<String>());
                break;
            }

            let tag: String = chars[i..=tag_end].iter().collect();
            let tag_name = tag
                .trim_start_matches('<')
                .trim_start_matches('/')
                .split_whitespace()
                .next()
                .unwrap_or("")
                .trim_end_matches('>')
                .trim_end_matches('/')
                .to_lowercase();

            // Track state transitions safely without nested loop blocks
            if tag_name == "code" {
                if !is_closing {
                    format_blind_code_mode = true;
                    if !is_minified && !result.is_empty() && !result.ends_with('\n') {
                        result.push('\n');
                    }
                    if !is_minified {
                        result.push_str(&indent_unit.repeat(depth));
                    }
                    result.push_str(&tag);
                    i = tag_end + 1;
                    continue;
                } else {
                    format_blind_code_mode = false;
                    result.push_str(&tag);
                    i = tag_end + 1;
                    continue;
                }
            }

            // Skip layout adjustments for inner contents of code elements
            if format_blind_code_mode {
                result.push_str(&tag);
                i = tag_end + 1;
                continue;
            }

            let is_void = void_tags.contains(&tag_name.as_str());
            let is_ignored = ignored_tags.contains(&tag_name.as_str());

            if is_closing && !is_ignored {
                depth = depth.saturating_sub(1);
            }

            if !is_minified && !result.is_empty() && !result.ends_with('\n') {
                result.push('\n');
            }
            
            if !is_minified {
                result.push_str(&indent_unit.repeat(depth));
            }
            result.push_str(&tag);

            if !is_closing && !tag.ends_with("/>") && !is_declaration && !is_void && !is_ignored {
                depth += 1;
            }

            i = tag_end + 1;
        } else {
            let mut text_end = i;
            while text_end < chars.len() && chars[text_end] != '<' {
                text_end += 1;
            }

            let text: String = chars[i..text_end].iter().collect();

            if format_blind_code_mode {
                result.push_str(&text);
            } else {
                let trimmed = text.trim();
                if !trimmed.is_empty() {
                    if !is_minified && !result.is_empty() && !result.ends_with('\n') {
                        result.push('\n');
                    }
                    result.push_str(&indent_unit.repeat(depth));
                    result.push_str(trimmed);
                }
            }

            i = text_end;
        }
    }

    result
}