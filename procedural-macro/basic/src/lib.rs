use proc_macro::TokenStream;

// three kinds of procedural macros:
// 1. Custom #[derive] macros
// 2. Attribute-like macros
// 3. Function-like macros

#[proc_macro]
pub fn sql(input: TokenStream) -> TokenStream {
    println!("{:#?}", input);
    "fn hello() { println!(\"Hello world\") }".parse().unwrap()
}

#[proc_macro]
pub fn generate(input: TokenStream) -> TokenStream {
    println!("{:#?}", input);
    TokenStream::default()
}