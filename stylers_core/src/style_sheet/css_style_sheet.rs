use crate::style::CSSAtRule;
use crate::style::CSSStyleRule;
use crate::style::{CSSRule, CSSStyleSheet};

impl CSSStyleSheet {
    //This function will take the whole stylesheet content as string and return CSSStyleSheet structure
    pub(crate) fn from_str(style_str: &str, random_class: &str) -> CSSStyleSheet {
        //removing all the comments in the css content.
        let mut style_str = style_str.to_string();
        while let Some((first, last)) = style_str.split_once("/*") {
            let mut temp = String::new();
            temp.push_str(first);
            let (_, end) = last
                .split_once("*/")
                .expect("Expecting to split the comment");
            temp.push_str(end);
            style_str = temp;
        }

        let mut css_style_sheet = CSSStyleSheet { css_rules: vec![] };
        let mut is_at_rule = false;
        let mut style = String::new();
        let mut no_of_openings = 0;
        let mut no_of_closings = 0;
        for ch in style_str.chars() {
            //trimming the style because empty spaces at the beginning are not significant.
            if style.trim_start().len() <= 0 && ch == '@' {
                is_at_rule = true;
            }
            if ch == '{' {
                no_of_openings += 1;
            }
            if ch == '}' {
                no_of_closings += 1;
            }
            style.push(ch);

            // ending with semicolon means at rule without style declaration
            if ch == ';' && is_at_rule && no_of_openings == 0 {
                //to omit empty whitespaces.
                style = style.trim().to_string();
                let at_rule = CSSAtRule::from_str(&style, random_class);
                css_style_sheet.css_rules.push(CSSRule::AtRule(at_rule));
                style = String::new();
                is_at_rule = false
            } else if ch == '}' && no_of_openings != 0 && no_of_openings == no_of_closings {
                //this else condition handle one block of at_rule or style rule from the whole style sheet content.
                //to omit empty whitespaces.
                style = style.trim().to_string();
                if is_at_rule {
                    let at_rule = CSSAtRule::from_str(&style, random_class);
                    css_style_sheet.css_rules.push(CSSRule::AtRule(at_rule));
                } else {
                    let style_rule = CSSStyleRule::from_str(&style, random_class);
                    css_style_sheet
                        .css_rules
                        .push(CSSRule::StyleRule(style_rule));
                }
                no_of_openings = 0;
                no_of_closings = 0;
                style = String::new();
                is_at_rule = false;
            }
        }

        css_style_sheet
    }
}
