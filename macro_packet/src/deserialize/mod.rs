mod packet;
mod r#struct;

pub use packet::*;
pub use r#struct::*;

use std::str::FromStr;

use proc_macro2::{Delimiter, TokenStream, TokenTree};
use quote::{quote, ToTokens};
use syn::{Expr, Ident};

fn option_getter_from_type(
    field_type: TokenStream,
    name: TokenStream,
    variable: bool,
    array: Option<Ident>,
    option: Option<Expr>,
) -> TokenStream {
    let mut field_type_iter = field_type.clone().into_iter();

    match field_type_iter.next() {
        Some(TokenTree::Ident(ident)) if ident.to_string() == "Option" => {
            let field_type = TokenStream::from_iter(field_type_iter).to_string();
            let getter = vec_getter_from_type(
                TokenStream::from_str(&field_type[1..field_type.len() - 1]).unwrap(),
                name,
                variable,
                array,
            );

            quote! { if #option { Some(#getter) } else { None } }
        }
        _ => vec_getter_from_type(field_type, name, variable, array),
    }
}

fn vec_getter_from_type(
    field_type: TokenStream,
    name: TokenStream,
    variable: bool,
    array: Option<Ident>,
) -> TokenStream {
    let mut field_type_iter = field_type.clone().into_iter();

    match field_type_iter.next() {
        Some(TokenTree::Ident(ident)) if ident.to_string() == "Vec" => {
            let getter = getter_from_type(
                field_type_iter.skip(1).next().to_token_stream(),
                name.clone(),
                variable,
            );

            if let Some(array) = array {
                quote! {
                    {
                        let mut array = Vec::new();

                        for _ in 0..(#array as usize) {
                            array.push(#getter);
                        }

                        array
                    }
                }
            } else {
                quote! {
                    {
                        let mut array = Vec::new();

                        while !#name.is_empty() {
                            array.push(#getter);
                        }

                        array
                    }
                }
            }
        }
        Some(TokenTree::Group(group)) if group.delimiter() == Delimiter::Bracket => {
            let mut group_stream_iter = group.stream().into_iter();
            let getter =
                getter_from_type(group_stream_iter.next().to_token_stream(), name, variable);
            let length = group_stream_iter.skip(1).next().to_token_stream();

            quote! {
                {
                    let mut array: #field_type = Default::default();

                    for i in 0..#length {
                        array[i] = #getter;
                    }

                    array
                }
            }
        }
        _ => getter_from_type(field_type, name, variable),
    }
}

fn getter_from_type(field_type: TokenStream, name: TokenStream, variable: bool) -> TokenStream {
    match field_type.to_string().as_str() {
        "bool" => quote! { #name.from_byte()? != 0 },
        "i8" => quote! { #name.from_byte()? },
        "u8" => quote! { #name.from_byte()? as u8 },
        "i16" => quote! { #name.from_short()? },
        "u16" => quote! { #name.from_short()? as u16 },
        "i32" => {
            if variable {
                quote! { #name.from_varint()? }
            } else {
                quote! { #name.from_int()? }
            }
        }
        "i64" => {
            if variable {
                quote! { #name.from_varlong()? }
            } else {
                quote! { #name.from_long()? }
            }
        }
        "String" => quote! { #name.from_packet_string()? },
        "Uuid" => quote! { #name.from_uuid()? },
        _ => quote! { #field_type::try_from(#name.as_mut())? },
    }
}
