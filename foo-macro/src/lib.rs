use proc_macro::TokenStream;

#[proc_macro]
pub fn foo(_: TokenStream) -> TokenStream {
    TokenStream::new()
}
