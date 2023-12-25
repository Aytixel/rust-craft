extern crate proc_macro;

use proc_macro::TokenStream;
use quote::{quote, ToTokens};
use syn::{parse_macro_input, spanned::Spanned, Error, Fields, ItemStruct};

#[proc_macro_derive(DeserializePacket, attributes(variable))]
pub fn deserialize_packet(input: TokenStream) -> TokenStream {
    let input_struct = parse_macro_input!(input as ItemStruct);
    let input_struct_span = input_struct.span();
    let input_struct_ident = input_struct.ident;
    let quote = if let Fields::Named(input_struct_fields) = input_struct.fields {
        let mut fields_name = Vec::new();
        let mut fields_value = Vec::new();

        for input_struct_field in input_struct_fields.named {
            let field_type = input_struct_field.ty.to_token_stream().to_string();
            let variable = Some("#[variable]".to_string())
                == input_struct_field
                    .attrs
                    .get(0)
                    .map(|attr| attr.to_token_stream().to_string());

            fields_name.push(input_struct_field.ident);

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
            impl TryFrom<packet::Packet> for #input_struct_ident {
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
        Error::new(input_struct_span, "Use #[packet] only on structs").into_compile_error()
    };

    TokenStream::from(quote)
}
