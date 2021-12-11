//! Procedral macro for data classes (records)

use proc_macro::TokenStream;
use syn::{parse_macro_input, Fields, Ident, Type, Visibility};

#[proc_macro_attribute]
/// Converts a struct into a record (must use named fields)
/// This does the following:
/// 1. Make all fields public
/// 2. Add constructor based on order of fields
/// 3. Adds tuple `From` implementations
pub fn record(_metadata: TokenStream, input: TokenStream) -> TokenStream {
    let mut strct = parse_macro_input!(input as syn::ItemStruct);
    let ident = &strct.ident;

    // fields
    let fields = if let Fields::Named(ref mut fields) = strct.fields {
        let pub_vis = syn::parse_str::<Visibility>("pub").unwrap();
        for field in fields.named.iter_mut() {
            field.vis = pub_vis.clone();
        }

        fields.clone()
    } else {
        panic!("Tried to make a struct with non-named fields a record!")
    };

    // constructor
    let new_fields = fields
        .named
        .iter()
        .map(|it| it.ident.as_ref().unwrap())
        .collect::<Vec<&Ident>>();
    let new_types = fields.named.iter().map(|it| &it.ty).collect::<Vec<&Type>>();

    // implement traits
    let impl_constructor = quote::quote! {
        impl #ident {
            pub fn new(#(#new_fields: #new_types),*) -> Self {
                Self {
                    #(#new_fields),*
                }
            }
        }
    };

    let impl_tuple_conversion = quote::quote! {
        impl From<(#(#new_types),*)> for #ident {
            fn from((#(#new_fields),*): (#(#new_types),*)) -> Self {
                Self::new(#(#new_fields),*)
            }
        }

        impl From<#ident> for (#(#new_types),*) {
            fn from(it: #ident) -> Self {
                (#(it.#new_fields),*)
            }
        }
    };

    quote::quote! {
        #strct
        #impl_constructor
        #impl_tuple_conversion
    }
    .into()
}
