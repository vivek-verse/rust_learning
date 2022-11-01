extern crate proc_macro;
use chrono::prelude::*;
use proc_macro::TokenStream;
use quote::quote;
use syn;

#[proc_macro_derive(Log)]
pub fn log_derive(input: TokenStream) -> TokenStream {
    let ast: syn::DeriveInput = syn::parse(input).unwrap();
    let name = &ast.ident;

    let trait_impl = quote! {
        impl Log for #name {
            fn info(&self, msg: &str){
                println!("[Info] {}: {}", stringify!(#name), msg);
            }
            fn warn(&self, msg: &str){
                println!("[Warn] {}: {}", stringify!(#name), msg);
            }
            fn error(&self, msg: &str){
                println!("[Err] {}: {}", stringify!(#name), msg);
            }
        }
    };

    trait_impl.into()
}
