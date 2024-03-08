#![allow(dead_code)]
#![allow(unused)]


extern crate proc_macro;

use proc_macro::TokenStream;
use quote::quote;
use syn::DeriveInput;
use syn;
use syn::token::Token;


#[proc_macro_derive(HelloMacro)]
pub fn hello_macro_dervie(input: TokenStream) ->  TokenStream {
    let ast: DeriveInput = syn::parse(input).unwrap();
    impl_hello_macro(&ast)
}

fn impl_hello_macro(ast: &DeriveInput) -> TokenStream {
    let name = &ast.ident;

    let gen = quote! {
        impl HelloMacro for #name {
            fn hello_macro() {
                println!("Hello, Macro! My name is {}!", stringify!(#name))
            }
        }
    };

    gen.into()
}

trait HelloMacro {
    fn hello_macro();
}