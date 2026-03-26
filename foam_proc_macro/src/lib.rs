use std::str::FromStr;

use proc_macro::Span;
use proc_macro2::TokenStream;
use quote::quote;
use syn::{File, Ident, ItemFn, visit_mut::VisitMut};

fn modern_tag() -> TokenStream {
    TokenStream::from_str("#[cfg(any(windows, unix))]").unwrap()
}
fn retro_tag() -> TokenStream {
    TokenStream::from_str("#[cfg(any(target_os = \"psp\"))]").unwrap()
}

struct ChangeFnName;

impl VisitMut for ChangeFnName {
    fn visit_item_fn_mut(&mut self, node: &mut ItemFn) {
        node.sig.ident = Ident::new("psp_main", Span::call_site().into());
    }
}

#[proc_macro_attribute]
pub fn foam_main(
    _attr: proc_macro::TokenStream,
    item: proc_macro::TokenStream,
) -> proc_macro::TokenStream {
    let original: File = syn::parse2(item.into()).unwrap();
    let mut mutated = original.clone();
    ChangeFnName.visit_file_mut(&mut mutated);
    let modern_tag = modern_tag();
    let output = quote! {
        #[cfg(target_os = "psp")]
        psp::module!("sample_module", 1, 1);
        #[cfg(target_os = "psp")]
        #mutated
        #[cfg(target_os = "psp")]
        extern crate alloc;
        #modern_tag
        #original
    };
    output.into()
}

#[proc_macro_attribute]
pub fn cfg_modern(
    _attr: proc_macro::TokenStream,
    item: proc_macro::TokenStream,
) -> proc_macro::TokenStream {
    let item = proc_macro2::TokenStream::from(item);
    let tag = modern_tag();
    quote! {
        #tag
        #item
    }
    .into()
}

#[proc_macro_attribute]
pub fn cfg_retro(
    _attr: proc_macro::TokenStream,
    item: proc_macro::TokenStream,
) -> proc_macro::TokenStream {
    let item = proc_macro2::TokenStream::from(item);
    let tag = retro_tag();
    quote! {
        #tag
        #item
    }
    .into()
}
