use std::collections::hash_map::RandomState;
use std::hash::{BuildHasher, Hasher};
use proc_macro::{TokenStream,TokenTree};

//todo: in future try to conver this to proc_macro2 types to use outside proc_macro crate
pub fn get_style_string(ts:TokenStream)->String{
    let mut pre_col:usize=0;
    let mut pre_line:usize=0;
    let mut style = String::new();
    let mut joint_punct =false;
    let mut random_class= rand_class();
    ts.into_iter().for_each(|t|{
        match t {
            TokenTree::Group(t) =>{
                //when we start partcular group that means we have completed parsing
                //one full css selector so we will change the random class now.
                random_class = rand_class();
                //after . or # charactor if there is some group
                if joint_punct{
                    panic!("something is wrong on line:{},col:{}",pre_line,pre_col);
                }
                style.push_str(&t.to_string());
                //todo:remove this
                style.push_str("\n");
                let end = t.span().end();
                pre_col = end.column;
                pre_line = end.line;
            },
            TokenTree::Ident(t)=>{
                if !joint_punct{
                    style.push(' ');
                }
                style.push_str(&t.to_string());
                //attaching random class to each selector
                style.push_str(&random_class);
                let end = t.span().end();
                pre_col = end.column;
                pre_line = end.line;
                joint_punct=false;
            },
            TokenTree::Literal(t)=>{
                let start = t.span().start();
                panic!("Literal strings are not allowed in selectors line:{},col:{}",start.line,start.column);
                // style.push_str(&t.to_string());
            },
            TokenTree::Punct(t)=>{
                let start = t.span().start();
                let end = t.span().end();
                let cur_col=start.column;
                let cur_line=start.line;

                if pre_line==cur_line && cur_col>pre_col{
                    style.push(' ');
                }else if pre_line==cur_line {
                    //remove the class we have added in previous step.
                    //we might have added generated class for joint selector(ex. div.class1) 
                    //only if the current selector and previous selector are in same line
                    let len = style.len();
                    if len>9{
                        style = String::from(&style[0..len-9]);
                    }
                }
                let cha = t.as_char();
                if cha == '.' || cha == '#'{
                    joint_punct = true;
                }
                style.push(cha);
                pre_col =end.column;
                pre_line = end.line;
            }
        }
    });
    style
}

fn rand_class() -> String{
    let hash = RandomState::new().build_hasher().finish().to_string();
    let k = &hash[0..6];
    format!(".l-{}", k.to_string())
}



