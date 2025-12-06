#![doc = include_str!("../README.md")]

use proc_macro::TokenStream;
use quote::{format_ident, quote};
use syn::{
    parse::{Parse, ParseStream}, parse_macro_input, Data, DeriveInput, Ident
};

struct TaggedDelegateArgs {
    name: Ident,
}

impl Parse for TaggedDelegateArgs {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let name = if input.is_empty() {
            format_ident!("delegate")
        } else {
            input.parse()?
        };

        Ok(Self {
            name,
        })
    }
}

#[proc_macro_attribute]
pub fn tagged_delegate(attr: TokenStream, item: TokenStream) -> TokenStream {
    let TaggedDelegateArgs { name } = parse_macro_input!(attr as TaggedDelegateArgs);
    let input = parse_macro_input!(item as DeriveInput);

    let enum_name = &input.ident;
    let variants = match &input.data {
        Data::Enum(data) => &data.variants,
        _ => panic!("tagged_delegates only supports enums")
    };

    let variant_names = variants.iter().map(|v| &v.ident).collect::<Vec<_>>();
    let mut_macro_name = format_ident!("mut_{name}");
    let pinned_macro_name = format_ident!("pinned_{name}");

    let output = quote! {
        #input

        #[allow(unused_macros)]
        macro_rules! #name {
            ($self: expr, |$variant:ident| $body:expr) => {
                match &$self.#name {
                    #(
                        #enum_name::#variant_names($variant) => $body,
                    )*
                }
            };
        }

        #[allow(unused_macros)]
        macro_rules! #mut_macro_name {
            ($self: expr, |$variant:ident| $body:expr) => {
                match &mut $self.#name {
                    #(
                        #enum_name::#variant_names($variant) => $body,
                    )*
                }
            };
        }

        #[allow(unused_macros)]
        macro_rules! #pinned_macro_name {
            ($self: expr, |$variant:ident| $body:expr) => {
                match &$self.#name {
                    #(
                        #enum_name::#variant_names($variant) => {
                            #[cfg(not(target_arch = "wasm32"))]
                            {
                                ::std::boxed::Box::pin($body) as ::std::pin::Pin<::std::boxed::Box<dyn ::std::future::Future<Output = _> + ::std::marker::Send + '_>>
                            }
                            #[cfg(target_arch = "wasm32")]
                            {
                                ::std::boxed::Box::pin($body) as ::std::pin::Pin<::std::boxed::Box<dyn ::std::future::Future<Output = _> + '_>>
                            }
                        }
                    )*
                }
            };
        }
    };

    TokenStream::from(output)
}
