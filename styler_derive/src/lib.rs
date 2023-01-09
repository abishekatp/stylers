use proc_macro::TokenStream;

#[proc_macro]
pub fn style(_item: TokenStream) -> TokenStream {
    println!("{}",_item);
    "".parse().unwrap()
}