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
#[macro_export]
macro_rules! rsx {
    ($style:ident, doctype_html $tag:ident { $($content:tt)* }) => {
        format!(
            "<!DOCTYPE html>\n{}", 
            forge_rsx::rsx!($style, $tag { $($content)* })
        )
    };
    (lined, $tag:ident { $($content:tt)* }) => {
        forge_rsx::rsx_muncher!(0, 0, $tag, [], [], $($content)*)
    };
    (btfy0, $tag:ident { $($content:tt)* }) => {
        forge_rsx::rsx_muncher!(1, 0, $tag, [], [], $($content)*)
    };
    (btfy2, $tag:ident { $($content:tt)* }) => {
        forge_rsx::rsx_muncher!(2, 0, $tag, [], [], $($content)*)
    };
    (btfy4, $tag:ident { $($content:tt)* }) => {
        forge_rsx::rsx_muncher!(4, 0, $tag, [], [], $($content)*)
    };
}

/// The core macro responsible for generating HTML-like markup with flexible indentation,
/// attribute handling, nested tags, loops, and expressions.
///
/// # Usage
/// This macro is primarily invoked internally by the `rsx!` macro, which provides a user-friendly interface.
/// It supports various patterns to construct complex nested HTML structures, including attributes, inner content,
/// loops, and conditional content.
///
/// # Pattern Breakdown
/// - **Termination:** Handles empty content (end of children).
/// - **Attributes:** Adds attributes to tags, supporting both identifier and literal patterns.
/// - **Nested tags:** Recursively processes inner tags with increased indentation.
/// - **Loops:** Supports iteration over collections to generate repeated content.
/// - **Braced expressions:** Embeds static text or expressions inside tags.
/// - **String literals:** Inserts string content directly.
/// - **Cleanup:** Handles trailing commas or empty patterns.
///
/// # Examples
/// ```rust
/// use forge_rsx::rsx_muncher;
/// // Basic tag with no attributes or children
/// rsx_muncher!(0, 0, div, [], [], );
///
/// // Tag with attributes
/// rsx_muncher!(0, 0, a, [("href", "https://example.com")], [], );
///
/// // Nested tags
/// rsx_muncher!(0, 0, div, [], [], span { "Hello" } );
///
/// // Loop generating multiple items
/// rsx_muncher!(0, 0, ul, [], [], for item in vec!["One", "Two"] => { li { {item} } } );
/// ```
///
/// # Arguments
/// - `$m`: indentation mode (e.g., 2, 4).
/// - `$d`: current indentation depth.
/// - `$tag`: the HTML tag name (ident).
/// - `$attrs`: list of attributes (tt tokens).
/// - `$children`: list of child content (expressions).
/// - Remaining patterns: inner tags, loops, expressions, etc.
#[macro_export]
macro_rules! rsx_muncher {
    // 1. TERMINATION - Generates the final string
    ($m:expr, $d:expr, $tag:ident, [$($attrs:tt)*], [$($children:expr),*], ) => {{
        #[allow(unused_mut)]
        let mut attr_str = String::new();
        $(
            /// Iterates through collected attributes and formats them into a single HTML attribute string.
            /// 
            /// This block handles three specific scenarios:
            /// a. **Boolean Attributes**: If value is `true`, renders only the key (e.g., `defer`). 
            ///    If `false`, the attribute is omitted entirely.
            /// b. **Special Frameworks**: Uses single quotes `'` if the key starts with `@`, `:`, `x-`, or `hx-` 
            ///    (common in Alpine.js and htmx) to allow JSON-like strings inside.
            /// c. **Standard Attributes**: Renders as `key="value"` using double quotes.
            if let Some((k, v)) = forge_rsx::parse_attr!($attrs) {
                let key = k.trim_matches('"');
                let val_str = format!("{}", v);
                if val_str == "true" {
                    // Handle Boolean: Renders standalone key (e.g., <script defer>)
                    attr_str.push_str(&format!(" {}", key));
                } else if val_str != "false" {
                    // Skip if "false", otherwise determine quoting style
                    if key.starts_with(':') || key.starts_with('@') || key.starts_with("x-") || key.starts_with("hx-") || 
                       val_str.contains('"') || val_str.contains("\\\"") {
                        // Use single quotes for expressions or strings containing double quotes
                        let clean_v = val_str.replace("\\\"", "\"");
                        attr_str.push_str(&format!(" {}='{}'", key, clean_v));
                    } else {
                        // Default: Standard double-quoted attribute
                        attr_str.push_str(&format!(" {}=\"{}\"", key, val_str));
                    }
                }
            }
        )*

        let indent = match $m { 2 => "  ".repeat($d), 4 => "    ".repeat($d), _ => String::new() };
        let nl = if $m > 0 { "\n" } else { "" };

        #[allow(unused_mut)]
        let mut inner_content = String::new();
        $(
            if !inner_content.is_empty() { inner_content.push_str(nl); }
            inner_content.push_str(&format!("{}", $children));
        )*

        let tag_name = stringify!($tag);
        let is_void = matches!(tag_name, "area" | "base" | "br" | "col" | "embed" | "hr" | "img" | "input" | "link" | "meta" | "source" | "track" | "wbr");

        if is_void {
            format!("{}<{}{}>", indent, tag_name, attr_str)
        } else if inner_content.is_empty() {
            format!("{}<{}{}></{}>", indent, tag_name, attr_str, tag_name)
        } else {
            format!("{}<{}{}>{}{}{}{}</{}>", indent, tag_name, attr_str, nl, inner_content, nl, indent, tag_name)
        }
    }};

    // 2a. ATTRIBUTE with COMMA (Identifier key)
    ($m:expr, $d:expr, $tag:ident, [$($attrs:tt)*], [$($children:expr),*], $attr_name:ident : $attr_value:expr, $($rest:tt)+) => {
        forge_rsx::rsx_muncher!($m, $d, $tag, [$($attrs)* (stringify!($attr_name), $attr_value)], [$($children),*], $($rest)*)
    };

    // 2b. ATTRIBUTE with COMMA (Literal key)
    ($m:expr, $d:expr, $tag:ident, [$($attrs:tt)*], [$($children:expr),*], $attr_name:literal : $attr_value:expr, $($rest:tt)+) => {
        forge_rsx::rsx_muncher!($m, $d, $tag, [$($attrs)* (stringify!($attr_name), $attr_value)], [$($children),*], $($rest)*)
    };

    // 2c. TERMINAL ATTRIBUTE NO COMMA (Identifier key)
    ($m:expr, $d:expr, $tag:ident, [$($attrs:tt)*], [$($children:expr),*], $attr_name:ident : $attr_value:expr) => {
        forge_rsx::rsx_muncher!($m, $d, $tag, [$($attrs)* (stringify!($attr_name), $attr_value)], [$($children),*], )
    };

    // 2d. TERMINAL ATTRIBUTE NO COMMA (Literal key)
    ($m:expr, $d:expr, $tag:ident, [$($attrs:tt)*], [$($children:expr),*], $attr_name:literal : $attr_value:expr) => {
        forge_rsx::rsx_muncher!($m, $d, $tag, [$($attrs)* (stringify!($attr_name), $attr_value)], [$($children),*], )
    };

    // 3. NESTED TAGS
    ($m:expr, $d:expr, $tag:ident, [$($attrs:tt)*], [$($children:expr),*], $inner_tag:ident { $($inner_content:tt)* } $($rest:tt)*) => {
        forge_rsx::rsx_muncher!($m, $d, $tag, [$($attrs)*], [$($children,)* forge_rsx::rsx_muncher!($m, $d + 1, $inner_tag, [], [], $($inner_content)*)], $($rest)*)
    };

    // 4. FOR LOOPS
    ($m:expr, $d:expr, $tag:ident, [$($attrs:tt)*], [$($children:expr),*], for $var:ident in $collection:expr => { $it:ident { $($ic:tt)* } } $($rest:tt)*) => {{
        #[allow(unused_mut)]
        let mut s = String::new();
        let nl = if $m > 0 { "\n" } else { "" };
        for $var in $collection {
            if !s.is_empty() { s.push_str(nl); }
            s.push_str(&forge_rsx::rsx_muncher!($m, $d + 1, $it, [], [], $($ic)*));
        }
        forge_rsx::rsx_muncher!($m, $d, $tag, [$($attrs)*], [$($children,)* s], $($rest)*)
    }};

    // 5. BRACED EXPRESSIONS
    ($m:expr, $d:expr, $tag:ident, [$($attrs:tt)*], [$($children:expr),*], { $text:expr } $($rest:tt)*) => {
        forge_rsx::rsx_muncher!($m, $d, $tag, [$($attrs)*], [$($children,)* format!("{}{}", match $m { 2 => "  ".repeat($d + 1), 4 => "    ".repeat($d + 1), _ => String::new() }, $text)], $($rest)*)
    };

    // 6. STRING LITERALS
    ($m:expr, $d:expr, $tag:ident, [$($attrs:tt)*], [$($children:expr),*], $text:literal $($rest:tt)*) => {
        forge_rsx::rsx_muncher!($m, $d, $tag, [$($attrs)*], [$($children,)* format!("{}{}", match $m { 2 => "  ".repeat($d + 1), 4 => "    ".repeat($d + 1), _ => String::new() }, $text)], $($rest)*)
    };

    // 7. CLEANUP
    ($m:expr, $d:expr, $tag:ident, [$($attrs:tt)*], [$($children:expr),*], , $($rest:tt)*) => {
        forge_rsx::rsx_muncher!($m, $d, $tag, [$($attrs)*], [$($children),*], $($rest)*)
    };
}



/// Parses attribute pattern into a key-value tuple, if applicable.
///
/// Supports two patterns:
/// - `( ($key:expr, $val:expr) )`
/// - Any other pattern returns `None`.
///
/// # Usage
/// ```rust
/// use forge_rsx::parse_attr;
/// assert_eq!(parse_attr!( ("href", "https://example.com") ), Some( ("href", "https://example.com") ));
/// assert_eq!(parse_attr!( some_other_pattern ), None::<( &str, &str )>);
/// ```
///
/// # Arguments
/// - `$other`: the token pattern to match, typically a tuple of key-value strings.
#[macro_export]
macro_rules! parse_attr {
    ( ($key:expr, $val:expr) ) => { Some(($key, $val)) };
    ( $other:tt ) => { None };
}