extern crate proc_macro;
extern crate proc_macro2;

use proc_macro::TokenStream;
use quote::{quote};
use syn;

fn make_comp(ast: &syn::ItemFn) -> TokenStream {
    let gen = quote! {
        #ast
    };
    gen.into()
}

#[proc_macro_attribute]
pub fn comp(_attrs: TokenStream, item: TokenStream) -> TokenStream {
    // Construct a representation of Rust code as a syntax tree
    // that we can manipulate
    let ast = syn::parse(item).unwrap();

    make_comp(&ast)
}
