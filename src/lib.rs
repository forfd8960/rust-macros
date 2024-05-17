// for enum, generate From implements for each variant
mod auto;
mod enum_from;
mod enum_from_darling;

use auto::{process_auto_debug, process_auto_deref};
use enum_from::procee_enum_from_input;
use enum_from_darling::process_from_darling;
use proc_macro::TokenStream;

#[proc_macro_derive(EnumFrom)]
pub fn derive_enum_from(input: TokenStream) -> TokenStream {
    let input = syn::parse_macro_input!(input as syn::DeriveInput);
    procee_enum_from_input(input).into()
}

#[proc_macro_derive(EnumFromDarling)]
pub fn derive_enum_from_darling(input: TokenStream) -> TokenStream {
    let input = syn::parse_macro_input!(input as syn::DeriveInput);
    process_from_darling(input).into()
}

#[proc_macro_derive(AutoDeref, attributes(deref))]
pub fn derive_auto_deref(input: TokenStream) -> TokenStream {
    let input = syn::parse_macro_input!(input as syn::DeriveInput);
    process_auto_deref(input).into()
}

#[warn(unused_variables)]
#[proc_macro_derive(AutoDebug, attributes(debug))]
pub fn derive_auto_debug(_input: TokenStream) -> TokenStream {
    let input = syn::parse_macro_input!(_input as syn::DeriveInput);
    process_auto_debug(input).into()
}
