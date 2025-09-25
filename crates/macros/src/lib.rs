use proc_macro::TokenStream;
use quote::quote;
use syn::{DeriveInput, parse_macro_input};

use syn::Fields;

#[proc_macro_attribute]
pub fn show_streams(attr: TokenStream, item: TokenStream) -> TokenStream {
    println!("attr: \"{attr}\"");
    println!("item: \"{item}\"");
    item
}

#[proc_macro_derive(TouchUpdateEvent)]
pub fn derive_touch_update_event(input: TokenStream) -> TokenStream {
    eprintln!("Macro invoked! Tokens: {}", input);
    let input = parse_macro_input!(input as DeriveInput);
    let name = &input.ident;
    eprintln!("Macro parsed!");
    eprintln!("Input struct name: {}", name);
    // Expect a tuple struct with a single field like: pub struct Foo(pub Arc<T>);
    let inner_ty = match &input.data {
        syn::Data::Struct(s) => match &s.fields {
            Fields::Unnamed(unnamed) if unnamed.unnamed.len() == 1 => {
                &unnamed.unnamed.first().unwrap().ty
            }
            _ => panic!("#[derive(TouchUpdateEvent)] requires a tuple struct with one field"),
        },
        _ => panic!("#[derive(TouchUpdateEvent)] only supports tuple structs"),
    };

    // generate implementations
    let expanded = quote! {
        impl std::ops::Deref for #name {
            type Target = #inner_ty;
            fn deref(&self) -> &Self::Target {
                &self.0
            }
        }

        impl Clone for #name {
            fn clone(&self) -> Self {
                Self(self.0.clone())
            }
        }

        impl From<&TouchEvent> for #name {
            fn from(value: &TouchEvent) -> Self {
                #name(value.0.clone())
            }
        }

        impl #name {
            pub fn from_unchecked(value: &TouchEvent) -> Self {
                #name(value.0.clone())
            }
        }
    };

    TokenStream::from(expanded)
}
