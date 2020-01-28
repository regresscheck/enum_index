extern crate syn;
#[macro_use]
extern crate quote;
extern crate proc_macro;
extern crate proc_macro2;
use quote::format_ident;
use syn::{parse_macro_input, DeriveInput};

#[proc_macro_derive(EnumIndex)]
pub fn enum_index(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let ast = parse_macro_input!(input as DeriveInput);

    let tokens = impl_enum_index(&ast);
    proc_macro::TokenStream::from(tokens)
}


fn impl_enum_index(ast: &syn::DeriveInput) -> proc_macro2::TokenStream {
    let name = &ast.ident;
    let (impl_generics, ty_generics, where_clause) = ast.generics.split_for_impl();

    let variants = match ast.data {
        syn::Data::Enum(ref v) => &v.variants,
        _ => panic!("EnumIndex can be only implemented for Enums")
    };

    let mut matches = Vec::new();

    for variant in variants {
        use syn::Fields::*;
        let ident = &variant.ident;

        let params = match variant.fields {
            Unit => format_ident!(""),
            Unnamed(..) => format_ident!("(..)"),
            Named(..) => format_ident!("{{..}}")
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
pub fn index_enum(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let ast = parse_macro_input!(input as DeriveInput);

    let tokens = impl_index_enum(&ast);
    proc_macro::TokenStream::from(tokens)
}


fn impl_index_enum(ast: &syn::DeriveInput) -> proc_macro2::TokenStream {
    let name = &ast.ident;
    let (impl_generics, ty_generics, where_clause) = ast.generics.split_for_impl();

    let variants = match ast.data {
        syn::Data::Enum(ref v) => &v.variants,
        _ => panic!("IndexEnum can be only implemented for Enums")
    };

    let mut index_matches = Vec::new();
    let mut index : usize = 0;

    for variant in variants {
        use syn::Fields::*;
        let ident = &variant.ident;
        match variant.discriminant {
            None => {},
            Some((_, syn::Expr::Lit(syn::ExprLit { lit: syn::Lit::Int(ref i), .. }))) => {
                index = i.base10_parse::<usize>().unwrap();
            },
            Some(_) => panic!("Unhandled enum discriminant!"),
        };

        match variant.fields {
            Unit => {
                index_matches.push(quote! { #index => Some(#name::#ident) });
            },
            Unnamed(ref fields) => {
                let mut initialized_fields = Vec::new();
                for field in &fields.unnamed {
                    let field_type = &field.ty;
                    initialized_fields.push(quote! { #field_type::default()} );
                }
                index_matches.push(quote! {
                    #index => Some(#name::#ident(#(#initialized_fields),*))
                });
            }
            Named(ref fields) => {
                let mut initialized_fields = Vec::new();
                for field in &fields.named {
                    let field_name = &field.ident;
                    let field_type = &field.ty;
                    initialized_fields.push(quote! { #field_name: #field_type::default()});
                }
                index_matches.push(quote! { #index => Some(#name::#ident{#(#initialized_fields),*})});
            }
        }
        index += 1;
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
