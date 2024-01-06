mod packet;
mod r#struct;

pub use packet::*;
pub use r#struct::*;

use std::str::FromStr;

use proc_macro2::{Delimiter, TokenStream, TokenTree};
use quote::{quote, ToTokens};

fn option_setter_from_type(
    field_type: TokenStream,
    name: TokenStream,
    variable: bool,
    nbt: bool,
) -> TokenStream {
    let mut field_type_iter = field_type.clone().into_iter();

    match field_type_iter.next() {
        Some(TokenTree::Ident(ident)) if ident.to_string() == "Option" => {
            let field_type = TokenStream::from_iter(field_type_iter).to_string();
            let setter = vec_setter_from_type(
                TokenStream::from_str(&field_type[1..field_type.len() - 1]).unwrap(),
                quote!(option),
                variable,
                nbt,
            );

            quote! { #name.map(|option| #setter).unwrap_or_default() }
        }
        _ => vec_setter_from_type(field_type, name, variable, nbt),
    }
}

fn vec_setter_from_type(
    field_type: TokenStream,
    name: TokenStream,
    variable: bool,
    nbt: bool,
) -> TokenStream {
    let mut field_type_iter = field_type.clone().into_iter();

    match field_type_iter.next() {
        Some(TokenTree::Ident(ident)) if ident.to_string() == "Vec" => {
            let setter = setter_from_type(
                field_type_iter.skip(1).next().to_token_stream(),
                quote!(value),
                variable,
                nbt,
            );

            quote! { #name.into_iter().flat_map(|value| #setter).collect::<Vec<u8>>() }
        }
        Some(TokenTree::Group(group)) if group.delimiter() == Delimiter::Bracket => {
            let mut group_stream_iter = group.stream().into_iter();
            let setter = setter_from_type(
                group_stream_iter.next().to_token_stream(),
                quote!(value),
                variable,
                nbt,
            );

            quote! { #name.into_iter().flat_map(|value| #setter).collect::<Vec<u8>>() }
        }
        _ => setter_from_type(field_type, name, variable, nbt),
    }
}

fn setter_from_type(
    field_type: TokenStream,
    name: TokenStream,
    variable: bool,
    nbt: bool,
) -> TokenStream {
    match field_type.to_string().as_str() {
        "bool" => quote! { #name.to_byte() },
        "i8" => quote! { #name.to_byte() },
        "u8" => quote! { #name.to_byte() },
        "i16" => quote! { #name.to_short() },
        "u16" => quote! { #name.to_short() },
        "i32" => {
            if variable {
                quote! { #name.to_varint() }
            } else {
                quote! { #name.to_int() }
            }
        }
        "i64" => {
            if variable {
                quote! { #name.to_varlong() }
            } else {
                quote! { #name.to_long() }
            }
        }
        "String" => quote! { #name.to_packet_string() },
        "Uuid" => quote! { #name.to_uuid() },
        _ => {
            if nbt {
                quote! { packet::nbt::serde::serialize::<#field_type>(&#name, None, packet::nbt::io::Flavor::Uncompressed).unwrap() }
            } else {
                quote! { Vec::<u8>::from(#name) }
            }
        }
    }
}
