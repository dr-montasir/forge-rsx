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
    minify_impl(html, false)
}

/// Minifies an HTML string slice into a compact, single-line format while preserving
/// HTML comments.
///
/// Removes unnecessary newlines, tabs, and redundant whitespace between elements.
/// Whitespace inside `<pre>` and `<code>` blocks is preserved. Unlike [`minify`],
/// this function retains HTML comments.
///
/// # Arguments
///
/// * `html` - The HTML string to minify.
///
/// # Example
///
/// ```rust
/// use forge_rsx::minify_with_comments;
///
/// let raw = r####"
///     <div>
///         <!-- Site name -->
///         <span>copyright 2026</span>
///         <a href="/">mysite.com</a>
///     </div>
/// "####;
///
/// let result = minify_with_comments(raw);
///
/// assert_eq!(
///     result,
///     r#"<div> <!-- Site name --><span>copyright 2026</span> <a href="/">mysite.com</a> </div>"#
/// );
/// ```
pub fn minify_with_comments(html: &str) -> String {
    minify_impl(html, true)
}

/// Minifies an HTML string, optionally preserving HTML comments.
///
/// Trims leading and trailing whitespace and removes redundant whitespace from
/// the markup. Whitespace within `<pre><code>...</code></pre>` content is
/// preserved, while script and style content is compacted with whitespace
/// retained where needed to separate tokens. HTML comments are removed unless
/// `keep_comments` is `true`.
///
/// # Arguments
///
/// * `html` - The HTML string to minify.
/// * `keep_comments` - Whether to retain HTML comments.
fn minify_impl(html: &str, keep_comments: bool) -> String {
    let mut result = String::with_capacity(html.len());
    let mut chars = html.trim().chars().peekable();

    let mut in_pre = false;
    let mut in_code = false;

    let mut last_was_whitespace = false;
    let mut in_script = false;
    let mut in_style = false;
    let mut in_block_comment = false;

    while let Some(c) = chars.next() {
        let in_pre_code = in_pre && in_code;

        if (in_script || in_style) && in_block_comment {
            if c == '*' && chars.peek() == Some(&'/') {
                chars.next();
                in_block_comment = false;
            }
            continue;
        }

        if (in_script || in_style) && c == '/' && chars.peek() == Some(&'*') {
            chars.next();
            in_block_comment = true;
            continue;
        }

        if in_script && c == '/' && chars.peek() == Some(&'/') {
            chars.next();
            while let Some(next_c) = chars.next() {
                if next_c == '\n' || next_c == '\r' {
                    break;
                }
            }
            if !result.ends_with(' ') {
                result.push(' ');
            }
            last_was_whitespace = true;
            continue;
        }

        let mut is_real_html_tag = false;

        if c == '<' {
            if in_script || in_style {
                if chars.peek() == Some(&'/') {
                    is_real_html_tag = true;
                }
            } else if in_pre_code {
                if chars.peek() == Some(&'/') {
                    let mut lookahead = chars.clone();
                    lookahead.next();

                    let mut tag_name = String::new();
                    while let Some(nc) = lookahead.next() {
                        if nc.is_alphabetic() {
                            tag_name.push(nc);
                        } else {
                            if nc == '>' {
                                break;
                            }
                            tag_name.clear();
                            break;
                        }
                    }

                    if tag_name.eq_ignore_ascii_case("code") {
                        while let Some(&nc) = lookahead.peek() {
                            if nc.is_whitespace() {
                                lookahead.next();
                            } else {
                                break;
                            }
                        }

                        if lookahead.next() == Some('<') && lookahead.next() == Some('/') {
                            let mut next_tag = String::new();
                            while let Some(nc) = lookahead.next() {
                                if nc.is_alphabetic() {
                                    next_tag.push(nc);
                                } else if nc == '>' {
                                    break;
                                } else {
                                    next_tag.clear();
                                    break;
                                }
                            }
                            if next_tag.eq_ignore_ascii_case("pre") {
                                is_real_html_tag = true;
                            }
                        }
                    }
                }
            } else {
                is_real_html_tag = true;
            }
        }

        if is_real_html_tag {
            if !keep_comments && !in_pre_code && chars.peek() == Some(&'!') {
                let mut lookahead = chars.clone();
                lookahead.next();

                if lookahead.next() == Some('-') && lookahead.next() == Some('-') {
                    chars.next();
                    chars.next();
                    chars.next();

                    while let Some(comment_c) = chars.next() {
                        if comment_c == '-' && chars.peek() == Some(&'-') {
                            let mut end_check = chars.clone();
                            end_check.next();
                            if end_check.next() == Some('>') {
                                chars.next();
                                chars.next();
                                break;
                            }
                        }
                    }

                    if !result.ends_with(char::is_whitespace) {
                        result.push(' ');
                    }
                    last_was_whitespace = true;
                    continue;
                }
            }

            let mut tag_buffer = String::from("<");
            let mut is_closing = false;

            if chars.peek() == Some(&'/') {
                is_closing = true;
                tag_buffer.push(chars.next().unwrap());
            }

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

            let is_pre_tag = tag_name_lower == "pre";
            let is_code_tag = tag_name_lower == "code";
            let is_script_tag = tag_name_lower == "script";
            let is_style_tag = tag_name_lower == "style";

            while let Some(next_c) = chars.next() {
                tag_buffer.push(next_c);
                if next_c == '>' {
                    break;
                }
            }

            if is_pre_tag {
                in_pre = !is_closing;
            } else if is_code_tag {
                in_code = !is_closing;
            }

            if !(in_pre && in_code) {
                if is_script_tag {
                    in_script = !is_closing;
                } else if is_style_tag {
                    in_style = !is_closing;
                }
            }

            result.push_str(&tag_buffer);

            if (is_pre_tag || is_code_tag) && in_pre && in_code {
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

        if in_pre && in_code {
            match c {
                '<' => result.push_str("&lt;"),
                '>' => result.push_str("&gt;"),
                '&' => result.push_str("&amp;"),
                _ => result.push(c),
            }
            last_was_whitespace = false;
        } else if in_script || in_style {
            if c.is_whitespace() {
                if !last_was_whitespace {
                    let syntax_symbols = if in_script {
                        "{}:;(),=+-><!&|"
                    } else {
                        "{}:;(),>+"
                    };
                    let prev_is_symbol = result
                        .chars()
                        .last()
                        .map_or(false, |p| syntax_symbols.contains(p));

                    let mut lookahead_chars = chars.clone();
                    let mut next_non_space = None;
                    while let Some(nc) = lookahead_chars.next() {
                        if !nc.is_whitespace() {
                            next_non_space = Some(nc);
                            break;
                        }
                    }
                    let next_is_symbol =
                        next_non_space.map_or(false, |n| syntax_symbols.contains(n));

                    if !prev_is_symbol && !next_is_symbol && !result.is_empty() {
                        result.push(' ');
                    }
                    last_was_whitespace = true;
                }
            } else {
                let syntax_symbols = if in_script {
                    "{}:;(),=+-><!&|"
                } else {
                    "{}:;(),>+"
                };
                let current_is_symbol = syntax_symbols.contains(c);

                if current_is_symbol && result.ends_with(' ') {
                    result.pop();
                }

                result.push(c);
                last_was_whitespace = false;
            }
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

    while result.ends_with(' ') {
        result.pop();
    }

    result
}