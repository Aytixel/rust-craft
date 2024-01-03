extern crate proc_macro;

mod deserialize;
mod serialize;

use std::str::FromStr;

use proc_macro::{Delimiter, Span, TokenStream, TokenTree};
use proc_macro2::Literal;
use quote::{quote, ToTokens};
use syn::{parse_macro_input, Attribute, Error, Expr, Ident, ItemStruct, Meta, MetaList};

fn get_id(attrs: &Vec<Attribute>) -> Literal {
    attrs
        .iter()
        .find(|attr| attr.path().is_ident("id"))
        .map(|attr| attr.parse_args::<Literal>().ok())
        .flatten()
        .unwrap_or(Literal::from_str("0").unwrap())
}

fn is_variable(attrs: &Vec<Attribute>) -> bool {
    attrs.iter().any(|attr| attr.path().is_ident("variable"))
}

fn get_array(attrs: &Vec<Attribute>) -> Option<Ident> {
    attrs
        .iter()
        .find(|attr| attr.path().is_ident("array"))
        .map(|attr| attr.parse_args::<Ident>().ok())
        .flatten()
}

fn get_option(attrs: &Vec<Attribute>) -> Option<Expr> {
    attrs
        .iter()
        .find(|attr| attr.path().is_ident("option"))
        .map(|attr| attr.parse_args::<Expr>().ok())
        .flatten()
}

#[proc_macro_derive(DeserializePacket, attributes(variable, array, option, id))]
pub fn deserialize_packet(item: TokenStream) -> TokenStream {
    deserialize::deserialize_packet(item)
}

#[proc_macro_derive(SerializePacket, attributes(variable, array, option, id))]
pub fn serialize_packet(item: TokenStream) -> TokenStream {
    serialize::serialize_packet(item)
}

#[proc_macro_derive(DeserializeStruct, attributes(variable, array, option))]
pub fn deserialize_struct(item: TokenStream) -> TokenStream {
    deserialize::deserialize_struct(item)
}

#[proc_macro_derive(SerializeStruct, attributes(variable, array, option))]
pub fn serialize_struct(item: TokenStream) -> TokenStream {
    serialize::serialize_struct(item)
}

#[proc_macro]
pub fn packet_enum(item: TokenStream) -> TokenStream {
    let mut item_iter = item.into_iter();
    let ident = item_iter.next().map(TokenStream::from).unwrap_or_else(|| {
        Error::new(Span::call_site().into(), "No enum name")
            .into_compile_error()
            .into()
    });
    let ident = parse_macro_input!(ident as Ident);
    let mut item_struct = TokenStream::new();
    let mut items_struct = Vec::new();
    let mut items_struct_name = Vec::new();
    let mut deserialize_items_struct_name = Vec::new();
    let mut deserialize_items_struct_id = Vec::new();
    let mut serialize_items_struct_name = Vec::new();
    let mut serialize_items_struct_id = Vec::new();

    for item in item_iter {
        item_struct.extend([item.clone()]);

        if let TokenTree::Group(group) = item {
            if group.delimiter() == Delimiter::Brace {
                let tmp_item_struct = item_struct.clone();

                item_struct = TokenStream::new();

                let tmp_item_struct = parse_macro_input!(tmp_item_struct as ItemStruct);
                let mut id = quote!(0);

                for attr in tmp_item_struct.attrs.iter() {
                    if let Attribute {
                        meta: Meta::List(MetaList { path, tokens, .. }),
                        ..
                    } = attr
                    {
                        if path.to_token_stream().to_string() == "id" {
                            id = tokens.clone();
                            break;
                        }
                    }
                }

                for attr in tmp_item_struct.attrs.iter() {
                    if let Attribute {
                        meta: Meta::List(MetaList { path, tokens, .. }),
                        ..
                    } = attr
                    {
                        if path.to_token_stream().to_string() == "derive"
                            && tokens
                                .clone()
                                .into_iter()
                                .any(|token| token.to_string() == "DeserializePacket")
                        {
                            deserialize_items_struct_id.push(id.clone());
                            deserialize_items_struct_name.push(tmp_item_struct.ident.clone());
                            break;
                        }
                    }
                }

                for attr in tmp_item_struct.attrs.iter() {
                    if let Attribute {
                        meta: Meta::List(MetaList { path, tokens, .. }),
                        ..
                    } = attr
                    {
                        if path.to_token_stream().to_string() == "derive"
                            && tokens
                                .clone()
                                .into_iter()
                                .any(|token| token.to_string() == "SerializePacket")
                        {
                            serialize_items_struct_id.push(id);
                            serialize_items_struct_name.push(tmp_item_struct.ident.clone());
                            break;
                        }
                    }
                }

                items_struct_name.push(tmp_item_struct.ident.clone());
                items_struct.push(tmp_item_struct);
            }
        }
    }

    let quote = quote! {
        #[derive(Debug)]
        pub enum #ident {
            #( #items_struct_name(#items_struct_name), )*
        }

        impl TryFrom<packet::Packet> for #ident {
            type Error = packet::Error;

            fn try_from(packet: packet::Packet) -> Result<Self, Self::Error> {
                match packet.id {
                    #( #deserialize_items_struct_id => Ok(#ident::#deserialize_items_struct_name(#deserialize_items_struct_name::try_from(packet)?)), )*
                    _ => Err(Self::Error::msg(format!("No packet matching id : {}", packet.id)))
                }
            }
        }

        impl TryFrom<#ident> for packet::Packet {
            type Error = packet::Error;

            fn try_from(packet: #ident) -> Result<Self, Self::Error> {
                match packet {
                    #( #ident::#serialize_items_struct_name(packet) => Ok(packet::Packet::from(packet)), )*
                    _ => Err(Self::Error::msg("You can't serialize this packet"))
                }
            }
        }

        #( #items_struct )*
    };

    TokenStream::from(quote)
}
