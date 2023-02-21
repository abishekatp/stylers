use crate::style::utils::add_spaces;
use proc_macro2::{Delimiter, Group, TokenTree};
use std::cmp::min;
use std::collections::HashMap;

//ref: https://www.w3schools.com/cssref/index.php
static ALL_PROPERTIES: [&str; 328] = [
    "accent-color",
    "align-content",
    "align-items",
    "align-self",
    "all",
    "animation",
    "animation-delay",
    "animation-direction",
    "animation-duration",
    "animation-fill-mode",
    "animation-iteration-count",
    "animation-name",
    "animation-play-state",
    "animation-timing-function",
    "aspect-ratio",
    "backdrop-filter",
    "backface-visibility",
    "background",
    "background-attachment",
    "background-blend-mode",
    "background-clip",
    "background-color",
    "background-image",
    "background-origin",
    "background-position",
    "background-position-x",
    "background-position-y",
    "background-repeat",
    "background-size",
    "block-size",
    "border",
    "border-block",
    "border-block-color",
    "border-block-end-color",
    "border-block-end-style",
    "border-block-end-width",
    "border-block-start-color",
    "border-block-start-style",
    "border-block-start-width",
    "border-block-style",
    "border-block-width",
    "border-bottom",
    "border-bottom-color",
    "border-bottom-left-radius",
    "border-bottom-right-radius",
    "border-bottom-style",
    "border-bottom-width",
    "border-collapse",
    "border-color",
    "border-image",
    "border-image-outset",
    "border-image-repeat",
    "border-image-slice",
    "border-image-source",
    "border-image-width",
    "border-inline",
    "border-inline-color",
    "border-inline-end-color",
    "border-inline-end-style",
    "border-inline-end-width",
    "border-inline-start-color",
    "border-inline-start-style",
    "border-inline-start-width",
    "border-inline-style",
    "border-inline-width",
    "border-left",
    "border-left-color",
    "border-left-style",
    "border-left-width",
    "border-radius",
    "border-right",
    "border-right-color",
    "border-right-style",
    "border-right-width",
    "border-spacing",
    "border-style",
    "border-top",
    "border-top-color",
    "border-top-left-radius",
    "border-top-right-radius",
    "border-top-style",
    "border-top-width",
    "border-width",
    "bottom",
    "box-decoration-break",
    "box-reflect",
    "box-shadow",
    "box-sizing",
    "break-after",
    "break-before",
    "break-inside",
    "caption-side",
    "caret-color",
    "clear",
    "clip",
    "color",
    "column-count",
    "column-fill",
    "column-gap",
    "column-rule",
    "column-rule-color",
    "column-rule-style",
    "column-rule-width",
    "column-span",
    "column-width",
    "columns",
    "content",
    "counter-increment",
    "counter-reset",
    "cursor",
    "direction",
    "display",
    "empty-cells",
    "filter",
    "flex",
    "flex-basis",
    "flex-direction",
    "flex-flow",
    "flex-grow",
    "flex-shrink",
    "flex-wrap",
    "float",
    "font",
    "font-family",
    "font-feature-settings",
    "font-kerning",
    "font-language-override",
    "font-size",
    "font-size-adjust",
    "font-stretch",
    "font-style",
    "font-synthesis",
    "font-variant",
    "font-variant-alternates",
    "font-variant-caps",
    "font-variant-east-asian",
    "font-variant-ligatures",
    "font-variant-numeric",
    "font-variant-position",
    "font-weight",
    "gap",
    "grid",
    "grid-area",
    "grid-auto-columns",
    "grid-auto-flow",
    "grid-auto-rows",
    "grid-column",
    "grid-column-end",
    "grid-column-gap",
    "grid-column-start",
    "grid-gap",
    "grid-row",
    "grid-row-end",
    "grid-row-gap",
    "grid-row-start",
    "grid-template",
    "grid-template-areas",
    "grid-template-columns",
    "grid-template-rows",
    "hanging-punctuation",
    "height",
    "hyphens",
    "image-rendering",
    "inline-size",
    "inset",
    "inset-block",
    "inset-block-end",
    "inset-block-start",
    "inset-inline",
    "inset-inline-end",
    "inset-inline-start",
    "isolation",
    "justify-content",
    "justify-items",
    "justify-self",
    "left",
    "letter-spacing",
    "line-break",
    "line-height",
    "list-style",
    "list-style-image",
    "list-style-position",
    "list-style-type",
    "margin",
    "margin-block",
    "margin-block-end",
    "margin-block-start",
    "margin-bottom",
    "margin-inline",
    "margin-inline-end",
    "margin-inline-start",
    "margin-left",
    "margin-right",
    "margin-top",
    "mask",
    "mask-clip",
    "mask-composite",
    "mask-image",
    "mask-mode",
    "mask-origin",
    "mask-position",
    "mask-repeat",
    "mask-size",
    "mask-type",
    "max-height",
    "max-width",
    "@media",
    "max-block-size",
    "max-inline-size",
    "min-block-size",
    "min-inline-size",
    "min-height",
    "min-width",
    "mix-blend-mode",
    "object-fit",
    "object-position",
    "opacity",
    "order",
    "orphans",
    "outline",
    "outline-color",
    "outline-offset",
    "outline-style",
    "outline-width",
    "overflow",
    "overflow-anchor",
    "overflow-wrap",
    "overflow-x",
    "overflow-y",
    "overscroll-behavior",
    "overscroll-behavior-block",
    "overscroll-behavior-inline",
    "overscroll-behavior-x",
    "overscroll-behavior-y",
    "padding",
    "padding-block",
    "padding-block-end",
    "padding-block-start",
    "padding-bottom",
    "padding-inline",
    "padding-inline-end",
    "padding-inline-start",
    "padding-left",
    "padding-right",
    "padding-top",
    "page-break-after",
    "page-break-before",
    "page-break-inside",
    "paint-order",
    "perspective",
    "perspective-origin",
    "place-content",
    "place-items",
    "place-self",
    "pointer-events",
    "position",
    "quotes",
    "resize",
    "right",
    "rotate",
    "row-gap",
    "scale",
    "scroll-behavior",
    "scroll-margin",
    "scroll-margin-block",
    "scroll-margin-block-end",
    "scroll-margin-block-start",
    "scroll-margin-bottom",
    "scroll-margin-inline",
    "scroll-margin-inline-end",
    "scroll-margin-inline-start",
    "scroll-margin-left",
    "scroll-margin-right",
    "scroll-margin-top",
    "scroll-padding",
    "scroll-padding-block",
    "scroll-padding-block-end",
    "scroll-padding-block-start",
    "scroll-padding-bottom",
    "scroll-padding-inline",
    "scroll-padding-inline-end",
    "scroll-padding-inline-start",
    "scroll-padding-left",
    "scroll-padding-right",
    "scroll-padding-top",
    "scroll-snap-align",
    "scroll-snap-stop",
    "scroll-snap-type",
    "tab-size",
    "table-layout",
    "text-align",
    "text-align-last",
    "text-combine-upright",
    "text-decoration",
    "text-decoration-color",
    "text-decoration-line",
    "text-decoration-style",
    "text-decoration-thickness",
    "text-emphasis",
    "text-indent",
    "text-justify",
    "text-orientation",
    "text-overflow",
    "text-shadow",
    "text-transform",
    "text-underline-position",
    "top",
    "transform",
    "transform-origin",
    "transform-style",
    "transition",
    "transition-delay",
    "transition-duration",
    "transition-property",
    "transition-timing-function",
    "translate",
    "unicode-bidi",
    "user-select",
    "vertical-align",
    "visibility",
    "white-space",
    "widows",
    "width",
    "word-break",
    "word-spacing",
    "word-wrap",
    "writing-mode",
    "z-index",
];

//ref: https://developer.mozilla.org/en-US/docs/Web/API/CSSStyleDeclaration
//CSSStyleDeclaration is actual style declaration for each selectors
#[derive(Debug)]
pub struct CSSStyleDeclaration {
    //e.g {color:red;}
    style_css_text: String,
}

impl CSSStyleDeclaration {
    pub fn empty() -> CSSStyleDeclaration {
        CSSStyleDeclaration {
            style_css_text: "".to_string(),
        }
    }
    pub fn new(group: Group) -> CSSStyleDeclaration {
        let mut css_style_declar = CSSStyleDeclaration {
            style_css_text: "".to_string(),
        };
        css_style_declar.parse(group);
        css_style_declar
    }
    pub fn style_css_text(&self) -> String {
        self.style_css_text.clone()
    }

    pub fn parse(&mut self, group: Group) {
        let mut body = String::new();
        let mut property_map = HashMap::new();
        ALL_PROPERTIES.iter().for_each(|key| {
            property_map.insert(*key, ());
        });
        let mut property = String::new();
        let mut is_property_start = false;
        //if raw_str then we should not remove the double quotes
        let mut raw_str = false;

        let mut pre_col: usize = 0;
        let mut pre_line: usize = 0;

        body.push('{');
        group.stream().into_iter().for_each(|tt| match tt {
            TokenTree::Group(t) => {
                add_spaces(&mut property, t.span(), &mut pre_line, &mut pre_col);
                property.push_str(&parse_property_group(t, raw_str));
                //completed parsing current raw_str group.
                raw_str = false;
            }
            TokenTree::Ident(t) => {
                add_spaces(&mut property, t.span(), &mut pre_line, &mut pre_col);
                let ident = t.to_string();
                if ident == "raw_str" {
                    raw_str = true;
                } else {
                    property.push_str(&ident);
                }
            }
            TokenTree::Literal(t) => {
                add_spaces(&mut property, t.span(), &mut pre_line, &mut pre_col);
                //we are trimming r and # because in some cases user have to use r#"\1g"34"#.
                //note: we will also trim all double quotes by default unless it is wrapped with raw_str()
                property.push_str(
                    t.to_string()
                        .trim_start_matches('r')
                        .trim_matches(|c| c == '#' || c == '"'),
                );
            }
            TokenTree::Punct(t) => {
                let ch = t.as_char();
                //this will check if user added the semicolon or not.
                //since we are validating using colon actual line which is missing semicolon will be pre_line-1.
                if is_property_start && ch == ':' {
                    panic!("Missing simicolon in line {}", pre_line - 1)
                } else if ch == ':' {
                    is_property_start = true;
                    let (is_valid, suggest) = validate_property(&property, &property_map);
                    if !is_valid {
                        panic!(
                            "Did you mean to use {} property at line number {}",
                            suggest.expect("Expected suggestion"),
                            pre_line
                        );
                    }
                }

                add_spaces(&mut property, t.span(), &mut pre_line, &mut pre_col);
                property.push(ch);
                //end of declaration of one property key value pair.
                if ch == ';' {
                    body.push_str(&property);
                    property = String::new();
                    is_property_start = false;
                }
            }
        });
        body.push('}');
        self.style_css_text.push_str(&body);
    }
}

//this function will check if property key exists or not. if not it will suggest the most relevent property.
fn validate_property(prop_key: &str, prop_map: &HashMap<&str, ()>) -> (bool, Option<String>) {
    let property = prop_key.trim_start_matches("-webkit-");
    if prop_map.contains_key(property) {
        return (true, None);
    } else if property.starts_with("--") {
        //this will check if the property is custom css property.
        return (true, None);
    }
    let mut most_relevent = String::new();
    let mut min_distance = 1000;
    ALL_PROPERTIES.iter().for_each(|key| {
        let dist = levenshtein(prop_key, key);
        if dist < min_distance {
            min_distance = dist;
            most_relevent = key.to_string();
        }
    });
    (false, Some(most_relevent))
}

//levenshtein edit distance will give number inserterion or deletion or substitution needed to get string t from string s.
//ref: https://github.com/mbrlabs/distance/blob/master/src/levenshtein.rs, https://en.wikibooks.org/wiki/Algorithm_Implementation/Strings/Levenshtein_distance#Rust
pub fn levenshtein(s: &str, t: &str) -> usize {
    // get length of unicode chars
    let len_s = s.chars().count();
    let len_t = t.chars().count();

    // initialize the matrix
    let mut mat: Vec<Vec<usize>> = vec![vec![0; len_t + 1]; len_s + 1];
    for i in 1..(len_s + 1) {
        mat[i][0] = i;
    }
    for i in 1..(len_t + 1) {
        mat[0][i] = i;
    }

    //It will compare upper cell, left side cell and left digonal cell for minimum value.
    for (i, s_char) in s.chars().enumerate() {
        for (j, t_char) in t.chars().enumerate() {
            let substitution = if s_char == t_char { 0 } else { 1 };
            mat[i + 1][j + 1] = min3(
                mat[i][j + 1] + 1,        // deletion
                mat[i + 1][j] + 1,        // insertion
                mat[i][j] + substitution, // substitution
            );
        }
    }

    return mat[len_s][len_t];
}

pub fn min3(a: usize, b: usize, c: usize) -> usize {
    return min(min(a, b), c);
}

// This parse_property_group function will parse the TokenTree::Group and return a string.
// This parse group will handle some property specific conitions.
// when parseing group itself raw_str("hell0"), we will pass raw_str argument as true.
fn parse_property_group(group: Group, raw_str: bool) -> String {
    let mut body = String::new();
    let mut pre_col: usize = 0;
    let mut pre_line: usize = 0;
    let mut closing = ' ';
    //if raw_str then we should not remove the double quotes
    let mut raw_str = raw_str;
    match group.delimiter() {
        Delimiter::Brace => {
            body.push('{');
            closing = '}';
        }
        Delimiter::Parenthesis => {
            //there will be round bracket followed by raw_str() ident.
            if !raw_str {
                body.push('(');
                closing = ')';
            }
        }
        Delimiter::Bracket => {
            body.push('[');
            closing = ']';
        }
        _ => (),
    }
    group.stream().into_iter().for_each(|tt| match tt {
        TokenTree::Group(t) => {
            add_spaces(&mut body, t.span(), &mut pre_line, &mut pre_col);
            let mut group_str: &str = &parse_property_group(t, raw_str);
            //there will be group token followed by raw_str! ident.
            if raw_str {
                group_str = group_str.trim_matches(|c| c == '(' || c == ')');
                //completed parsing current raw_str so set to false.
                raw_str = false;
            }
            body.push_str(group_str);
        }
        TokenTree::Ident(t) => {
            add_spaces(&mut body, t.span(), &mut pre_line, &mut pre_col);
            let ident = t.to_string();
            if ident == "raw_str" {
                raw_str = true;
            }
            body.push_str(&ident);
        }
        TokenTree::Literal(t) => {
            add_spaces(&mut body, t.span(), &mut pre_line, &mut pre_col);
            //in case of properties will trim r,# around the string literals
            let mut literal: &str = &t.to_string();
            literal = literal.trim_start_matches('r').trim_matches(|c| c == '#');
            if !raw_str {
                literal = literal.trim_matches('"');
            }
            body.push_str(literal);
            //completed parsing current raw_str so set to false.
            raw_str = false;
        }
        TokenTree::Punct(t) => {
            add_spaces(&mut body, t.span(), &mut pre_line, &mut pre_col);
            body.push(t.as_char());
        }
    });
    body.push(closing);
    body.trim().to_string()
}
