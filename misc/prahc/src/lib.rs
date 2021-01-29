extern crate proc_macro;
use proc_macro::TokenStream;
use quote::quote;

#[proc_macro_attribute]
pub fn hello(attr: TokenStream, item: TokenStream) -> TokenStream {
let input = syn::parse_macro_input!(item as syn::ItemFn);
    let name = &input.ident;

    // Our input function is always equivalent to returning 42, right?
    let result = quote! {
        fn #name() -> u32 { 42 }
    };
    result.into()
}

