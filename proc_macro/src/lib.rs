#![feature(proc_macro)]

extern crate proc_macro;
extern crate proc_macro2;
#[macro_use]
extern crate quote;
#[macro_use]
extern crate syn;

use proc_macro2::Span;
use syn::ItemMod;
use quote::ToTokens;

#[proc_macro_attribute]
pub fn value_from_type(
    _args: proc_macro::TokenStream,
    input: proc_macro::TokenStream,
) -> proc_macro::TokenStream {
    println!("[BUILD] Running proc macro: value_from_type");
    println!("DBG INPUT\n{:?}", input.to_string());

    // Parse module code
    // IMPORTANT: Our own macro attribute is automatically stripped!
    let module_def: ItemMod = syn::parse2(input.into()).map_err(|e| {
        // This is the fail-hard message
        let msg = format!("Parse error: {:}", e);
        Span::call_site().unstable().error(msg).emit();
    }).unwrap();

    let out_tokens: proc_macro::TokenStream = module_def.into_tokens().into();
    println!("DBG OUT\n{:}", out_tokens.clone().to_string());
    println!("");
    return out_tokens;
}
