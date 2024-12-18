extern crate proc_macro;

use proc_macro::TokenStream;
use quote::{quote, ToTokens};
use syn::{parse_macro_input, ItemFn};


// Его задача — взять функцию, помеченную атрибутом #[function_to_string],
// и сгенерировать новую функцию, которая возвращает текстовое представление исходной функции.
#[proc_macro_attribute]
pub fn function_to_string(_attr: TokenStream, item: TokenStream) -> TokenStream {
    let input_fn: ItemFn = parse_macro_input!(item as ItemFn);
    let function_str: String = format!("{}", input_fn.to_token_stream());
    let fn_ident: proc_macro2::Ident = input_fn.sig.ident;
    let fn_inputs: syn::punctuated::Punctuated<syn::FnArg, syn::token::Comma> = input_fn.sig.inputs;
    let fn_generics: syn::Generics = input_fn.sig.generics;
    let output: proc_macro2::TokenStream = quote! {
        pub fn #fn_ident #fn_generics(#fn_inputs) -> &'static str {
            #function_str
        }
    };
    output.into()
}