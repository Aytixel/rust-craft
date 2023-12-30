extern crate proc_macro;

use std::str::FromStr;

use proc_macro::{Delimiter, Span, TokenStream, TokenTree};
use quote::{quote, ToTokens};
use syn::{
    parse_macro_input, spanned::Spanned, Attribute, Error, Fields, Ident, ItemStruct, Meta,
    MetaList,
};

#[proc_macro_derive(DeserializePacket, attributes(variable, array, id))]
pub fn deserialize_packet(item: TokenStream) -> TokenStream {
    let item_struct = parse_macro_input!(item as ItemStruct);
    let item_struct_span = item_struct.span();
    let item_struct_ident = item_struct.ident;
    let quote = if let Fields::Named(item_struct_fields) = item_struct.fields {
        let mut fields_name = Vec::new();
        let mut fields_value = Vec::new();

        for item_struct_field in item_struct_fields.named {
            let field_type_tokenstream = item_struct_field.ty.to_token_stream();
            let field_type = field_type_tokenstream.to_string();
            let mut variable = false;

            for attr in item_struct_field.attrs.iter() {
                if attr
                    .to_token_stream()
                    .to_string()
                    .starts_with("#[variable]")
                {
                    variable = true;
                    break;
                }
            }

            let mut array_option = None;

            for attr in item_struct_field.attrs.iter() {
                let attr_string = attr.to_token_stream().to_string();

                if attr_string.starts_with("#[array") {
                    array_option = Some(
                        proc_macro2::TokenStream::from_str(&attr_string[8..attr_string.len() - 2])
                            .unwrap(),
                    );
                    break;
                }
            }

            fn getter_from_type(field_type: &str, variable: bool) -> proc_macro2::TokenStream {
                match field_type {
                    "bool" => quote! { packet.data.from_byte()? != 0 },
                    "i8" => quote! { packet.data.from_byte()? },
                    "u8" => quote! { packet.data.from_byte()? as u8 },
                    "i16" => quote! { packet.data.from_short()? },
                    "u16" => quote! { packet.data.from_short()? as u16 },
                    "i32" => {
                        if variable {
                            quote! { packet.data.from_varint()? }
                        } else {
                            quote! { packet.data.from_int()? }
                        }
                    }
                    "i64" => {
                        if variable {
                            quote! { packet.data.from_varlong()? }
                        } else {
                            quote! { packet.data.from_long()? }
                        }
                    }
                    "String" => quote! { packet.data.from_packet_string()? },
                    "Uuid" => quote! { packet.data.from_uuid()? },
                    _ => quote! {},
                }
            }

            fields_name.push(item_struct_field.ident);
            fields_value.push(if field_type.starts_with("Vec") {
                let getter = getter_from_type(&field_type[6..field_type.len() - 2], variable);

                if let Some(array) = array_option {
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

                            while !packet.data.is_empty() {
                                array.push(#getter);
                            }

                            array
                        }
                    }
                }
            } else if field_type.starts_with("[") {
                let field_type_split: Vec<&str> =
                    field_type[1..field_type.len() - 1].split(" ; ").collect();
                let getter = getter_from_type(field_type_split[0], variable);
                let length = proc_macro2::TokenStream::from_str(field_type_split[1]).unwrap();

                quote! {
                    {
                        let mut array: #field_type_tokenstream = Default::default();

                        for i in 0..#length {
                            array[i] = #getter;
                        }

                        array
                    }
                }
            } else {
                getter_from_type(&field_type, variable)
            })
        }

        let quote = quote! {
            impl TryFrom<packet::Packet> for #item_struct_ident {
                type Error = packet::Error;

                fn try_from(mut packet: packet::Packet) -> Result<Self, Self::Error> {
                    use packet::{FromByte, FromShort, FromInt, FromLong, FromString, FromUuid, FromVarInt, FromVarLong};

                    #( let #fields_name = #fields_value; )*

                    Ok(Self {
                        #( #fields_name, )*
                    })
                }
            }
        };

        quote
    } else {
        Error::new(item_struct_span, "Use only on structs").into_compile_error()
    };

    TokenStream::from(quote)
}

#[proc_macro_derive(SerializePacket, attributes(variable, array, id))]
pub fn serialize_packet(item: TokenStream) -> TokenStream {
    let item_struct = parse_macro_input!(item as ItemStruct);
    let mut id = quote!(0);

    for attr in item_struct.attrs.iter() {
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

    let item_struct_span = item_struct.span();
    let item_struct_ident = item_struct.ident;
    let quote = if let Fields::Named(item_struct_fields) = item_struct.fields {
        let mut fields = Vec::new();

        for item_struct_field in item_struct_fields.named {
            let field_type = item_struct_field.ty.to_token_stream().to_string();
            let mut variable = false;

            for attr in item_struct_field.attrs.iter() {
                if "#[variable]".to_string() == attr.to_token_stream().to_string() {
                    variable = true;
                    break;
                }
            }

            let mut array_option = None;

            for attr in item_struct_field.attrs.iter() {
                let attr_string = attr.to_token_stream().to_string();

                if attr_string.starts_with("#[array") {
                    array_option = Some(
                        proc_macro2::TokenStream::from_str(&attr_string[8..attr_string.len() - 2])
                            .unwrap(),
                    );
                    break;
                }
            }

            let field_name = item_struct_field.ident;

            fn setter_from_type(field_type: &str, variable: bool) -> proc_macro2::TokenStream {
                match field_type {
                    "bool" => quote! { .to_byte() != 0 },
                    "i8" => quote! { .to_byte() },
                    "u8" => quote! { .to_byte() },
                    "i16" => quote! { .to_short() },
                    "u16" => quote! { .to_short() },
                    "i32" => {
                        if variable {
                            quote! { .to_varint() }
                        } else {
                            quote! { .to_int() }
                        }
                    }
                    "i64" => {
                        if variable {
                            quote! { .to_varlong() }
                        } else {
                            quote! { .to_long() }
                        }
                    }
                    "String" => quote! { .to_packet_string() },
                    "Uuid" => quote! { .to_uuid() },
                    _ => quote! {},
                }
            }

            fields.push(if field_type.starts_with("Vec") {
                let setter =
                    setter_from_type(&field_type[6..field_type.len() - 2], variable);


                if let Some(array) = array_option {
                    quote! { packet.#field_name[0..packet.#array as usize].iter().map(|value| value #setter).flatten().collect::<Vec<u8>>() }
                } else {
                    quote! { packet.#field_name.iter().map(|value| value #setter).flatten().collect::<Vec<u8>>() }
                }
            } else if field_type.starts_with("[") {
                let field_type_split: Vec<&str> =
                    field_type[1..field_type.len() - 1].split(" ; ").collect();
                let setter = setter_from_type(field_type_split[0], variable);

                quote! { packet.#field_name.iter().map(|value| value #setter).flatten().collect::<Vec<u8>>() }
            } else {
                let setter = setter_from_type(&field_type, variable);

                quote! { packet.#field_name #setter }
            });
        }

        let quote = quote! {
            impl From<#item_struct_ident> for packet::Packet {
                fn from(mut packet: #item_struct_ident) -> Self {
                    use packet::{ToByte, ToShort, ToInt, ToLong, ToString, ToUuid, ToVarInt, ToVarLong};

                    let mut data: Vec<u8> = Vec::new();

                    #( data.append(&mut #fields); )*

                    Self {
                        id: #id,
                        data,
                    }
                }
            }
        };

        quote
    } else {
        Error::new(item_struct_span, "Use only on structs").into_compile_error()
    };

    TokenStream::from(quote)
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
