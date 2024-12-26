extern crate proc_macro;

use proc_macro::TokenStream;
use quote::{quote, ToTokens};
use syn::{parse_macro_input, ItemFn};

/// A procedural macro attribute to convert a function's definition into a static string.
///
/// # Parameters
/// - `_attr`: Attributes passed to the macro (currently unused).
/// - `item`: The function to which the macro is applied.
///
/// # Functionality
/// This macro generates a new function that returns the original function's source code as a static string.
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
