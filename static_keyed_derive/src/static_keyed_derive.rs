extern crate proc_macro;
extern crate syn;
extern crate quote;

use proc_macro::{quote, TokenStream};
use syn::parse_macro_input;

#[proc_macro_derive(StaticKeyed)]
pub fn static_keyed(input: TokenStream) -> TokenStream {
    // Parses the TokenStream into a syntax tree that we can manipulate.
    let ast = syn::parse(input).unwrap();

    // Generate the implementation.
    let generated_code = implement_static_keyed(&ast);

    // Return the generated implementation.
    return TokenStream::from(generated_code);
}

fn implement_static_keyed(ast: &syn::DeriveInput) -> quote::Tokens {
    let name = &ast.ident;
    let generics = &ast.generics;

    quote! {
        impl #generics StaticKeyed for #name #generics {
            type StaticKey = #name <'static>;
        }
    }
}