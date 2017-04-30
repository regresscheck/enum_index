extern crate syn;
#[macro_use]
extern crate quote;
extern crate proc_macro;
use proc_macro::TokenStream;

fn impl_get_index(ast: &syn::DeriveInput) -> quote::Tokens {
    let name = &ast.ident;
    let (impl_generics, ty_generics, where_clause) = ast.generics.split_for_impl();

    let variants = match ast.body {
        syn::Body::Enum(ref v) => v,
        _ => panic!("EnumIndex can be only implemented for Enums")
    };

    let mut matches = Vec::new();

    for variant in variants {
        use syn::VariantData::*;
        let ident = &variant.ident;

        let params = match variant.data {
            Unit => quote::Ident::from(""),
            Tuple(..) => quote::Ident::from("(..)"),
            Struct(..) => quote::Ident::from("{..}")
        };

        let index = matches.len();
        matches.push(quote!{ #name::#ident #params => #index});
    }


    quote!{
        impl #impl_generics enum_index::EnumIndex for #name #ty_generics #where_clause {
            fn get_index(&self) -> usize {
                match *self {
                    #(#matches),*
                }
            }
        }
    }
}

#[proc_macro_derive(EnumIndex)]
pub fn get_index(input: TokenStream) -> TokenStream {
    let s = input.to_string();
    let ast = syn::parse_derive_input(&s).unwrap();

    let tokens = impl_get_index(&ast);
    tokens.parse().unwrap()
}
