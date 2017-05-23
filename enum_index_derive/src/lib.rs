extern crate syn;
#[macro_use]
extern crate quote;
extern crate proc_macro;
use proc_macro::TokenStream;

#[proc_macro_derive(EnumIndex)]
pub fn enum_index(input: TokenStream) -> TokenStream {
    let s = input.to_string();
    let ast = syn::parse_derive_input(&s).unwrap();

    let tokens = impl_enum_index(&ast);
    tokens.parse().unwrap()
}


fn impl_enum_index(ast: &syn::DeriveInput) -> quote::Tokens {
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
            fn enum_index(&self) -> usize {
                match *self {
                    #(#matches),*
                }
            }
        }
    }
}


#[proc_macro_derive(IndexEnum)]
pub fn index_enum(input: TokenStream) -> TokenStream {
    let s = input.to_string();
    let ast = syn::parse_derive_input(&s).unwrap();

    let tokens = impl_index_enum(&ast);
    tokens.parse().unwrap()
}


fn impl_index_enum(ast: &syn::DeriveInput) -> quote::Tokens {
    let name = &ast.ident;
    let (impl_generics, ty_generics, where_clause) = ast.generics.split_for_impl();

    let variants = match ast.body {
        syn::Body::Enum(ref v) => v,
        _ => panic!("IndexEnum can be only implemented for Enums")
    };

    let mut index_matches = Vec::new();
    let mut index : usize = 0;

    for variant in variants {
        use syn::VariantData::*;

        if let Unit = variant.data {
            let ident = &variant.ident;
            
            index_matches.push(quote! { #index => Some(#name::#ident) });
            index += 1;
        } else {
            panic!("IndexEnum can be only implemented for C-like Enums");
        }
    }


    quote!{
        impl #impl_generics enum_index::IndexEnum for #name #ty_generics #where_clause {
            fn index_enum(index: usize) -> Option<Self> {
                match index {
                    #(#index_matches),*,
                    _ => None,
                }
            }
        }
    }
}
