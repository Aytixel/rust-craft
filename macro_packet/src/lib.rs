extern crate proc_macro;

use proc_macro::{Delimiter, Span, TokenStream, TokenTree};
use quote::{quote, ToTokens};
use syn::{
    parse_macro_input, spanned::Spanned, Attribute, Error, Fields, Ident, ItemStruct, Meta,
    MetaList,
};

#[proc_macro_derive(DeserializePacket, attributes(variable, id))]
pub fn deserialize_packet(item: TokenStream) -> TokenStream {
    let item_struct = parse_macro_input!(item as ItemStruct);
    let item_struct_span = item_struct.span();
    let item_struct_ident = item_struct.ident;
    let quote = if let Fields::Named(item_struct_fields) = item_struct.fields {
        let mut fields_name = Vec::new();
        let mut fields_value = Vec::new();

        for item_struct_field in item_struct_fields.named {
            let field_type = item_struct_field.ty.to_token_stream().to_string();
            let mut variable = false;

            for attr in item_struct_field.attrs {
                if "#[variable]".to_string() == attr.to_token_stream().to_string() {
                    variable = true;
                    break;
                }
            }

            fields_name.push(item_struct_field.ident);

            match field_type.as_str() {
                "bool" => fields_value.push(quote! { packet.data.from_bytes()? != 0 }),
                "i8" => fields_value.push(quote! { packet.data.from_bytes()? }),
                "u8" => fields_value.push(quote! { packet.data.from_bytes()? as u8 }),
                "i16" => fields_value.push(quote! { packet.data.from_short()? }),
                "u16" => fields_value.push(quote! { packet.data.from_short()? as u16 }),
                "i32" => fields_value.push(if variable {
                    quote! { packet.data.from_varint()? }
                } else {
                    quote! { packet.data.from_int()? }
                }),
                "u32" => fields_value.push(quote! { packet.data.from_int()? as u32 }),
                "i64" => fields_value.push(if variable {
                    quote! { packet.data.from_varlong()? }
                } else {
                    quote! { packet.data.from_long()? }
                }),
                "u64" => fields_value.push(quote! { packet.data.from_long()? as u64 }),
                "String" => fields_value.push(quote! { packet.data.from_packet_string()? }),
                "Uuid" => fields_value.push(quote! { packet.data.from_uuid()? }),
                _ => fields_value.push(quote! { Default::Default() }),
            }
        }

        let quote = quote! {
            impl TryFrom<packet::Packet> for #item_struct_ident {
                type Error = packet::Error;

                fn try_from(mut packet: packet::Packet) -> Result<Self, Self::Error> {
                    use packet::{FromByte, FromShort, FromInt, FromLong, FromString, FromUuid, FromVarInt, FromVarLong};

                    Ok(Self {
                        #( #fields_name: #fields_value, )*
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

#[proc_macro_derive(SerializePacket, attributes(variable, id))]
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

            for attr in item_struct_field.attrs {
                if "#[variable]".to_string() == attr.to_token_stream().to_string() {
                    variable = true;
                    break;
                }
            }

            let field_name = item_struct_field.ident;

            match field_type.as_str() {
                "bool" => fields.push(quote! { packet.#field_name.to_bytes() != 0 }),
                "i8" => fields.push(quote! { packet.#field_name.to_bytes() }),
                "u8" => fields.push(quote! { (packet.#field_name as i8).to_bytes() }),
                "i16" => fields.push(quote! { packet.#field_name.to_short() }),
                "u16" => fields.push(quote! { (packet.#field_name as i16).to_short() }),
                "i32" => fields.push(if variable {
                    quote! { packet.#field_name.to_varint() }
                } else {
                    quote! { packet.#field_name.to_int() }
                }),
                "u32" => fields.push(quote! { (packet.#field_name as i32).to_int() }),
                "i64" => fields.push(if variable {
                    quote! { packet.#field_name.to_varlong() }
                } else {
                    quote! { packet.#field_name.to_long() }
                }),
                "u64" => fields.push(quote! { (packet.#field_name as i64).to_long() }),
                "String" => fields.push(quote! { packet.#field_name.to_packet_string() }),
                "Uuid" => fields.push(quote! { packet.#field_name.to_uuid() }),
                _ => fields.push(quote! { Default::Default() }),
            }
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

    println!("{}", quote);

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
    let mut items_struct_id = Vec::new();

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

                items_struct_id.push(id);
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
                    #( #items_struct_id => Ok(#ident::#items_struct_name(#items_struct_name::try_from(packet)?)), )*
                    _ => Err(Self::Error::msg(format!("No packet matching id : {}", packet.id)))
                }
            }
        }

        #( #items_struct )*
    };

    println!("{quote}");

    TokenStream::from(quote)
}
