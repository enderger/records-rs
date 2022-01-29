//! Procedral macro for data classes (records)

use proc_macro::TokenStream;
use syn::{parse_macro_input, Visibility, Token};

fn make_pub(fields: &mut syn::Fields) -> syn::FieldsNamed {
    if let syn::Fields::Named(ref mut fields) = fields {
        for field in fields.named.iter_mut() {
            field.vis = Visibility::from(syn::VisPublic { pub_token: <Token![pub]>::default() });
        }

        fields.clone()
    } else {
        panic!("Records must have named fields!")
    }
}

#[proc_macro_attribute]
/// Converts a struct into a record (must use named fields)
/// This does the following:
/// 1. Make all fields public
/// 2. Add constructor based on order of fields
/// 3. Adds tuple `From` implementations
pub fn record(_metadata: TokenStream, input: TokenStream) -> TokenStream {
    let mut strct = parse_macro_input!(input as syn::ItemStruct);

    // Struct components
    let struct_name = &strct.ident;
    let struct_generics = &strct.generics;
    let struct_fields = make_pub(&mut strct.fields);

    let struct_fields_vec = struct_fields.named
        .iter()
        .map(|it| it.ident.as_ref().unwrap())
        .collect::<Vec<&syn::Ident>>();
    let struct_types_vec = struct_fields.named
        .iter()
        .map(|it| it.ty.clone())
        .collect::<Vec<syn::Type>>();

    let (generics_impl, generics_type, where_clause) = struct_generics.split_for_impl();
    let type_name = quote::quote! { #struct_name #generics_type };

    // Constructor
    let constructor = quote::quote! {
        impl #generics_impl #type_name #where_clause {
            pub fn new(#(#struct_fields_vec: #struct_types_vec),*) -> Self {
                Self { #(#struct_fields_vec),* }
            }
        }
    };

    // Tuple type conversion
    let tuple_repr = quote::quote! { (#(#struct_fields_vec,)*) };
    let tuple_type = quote::quote! { (#(#struct_types_vec,)*) };

    let tuple_from = quote::quote! {
        impl #generics_impl From<#tuple_type> for #type_name #where_clause {
            fn from(#tuple_repr: #tuple_type) -> Self {
                Self::new(#(#struct_fields_vec),*)
            }
        }
    };

    let tuple_into = quote::quote! {
        impl #generics_impl From<#type_name> for #tuple_type #where_clause {
            fn from(it: #type_name) -> Self {
                let #struct_name { #(#struct_fields_vec),* } = it;
                #tuple_repr
            }
        }
    };


    quote::quote! {
        #strct
        #constructor
        #tuple_from
        #tuple_into
    }
    .into()
}
