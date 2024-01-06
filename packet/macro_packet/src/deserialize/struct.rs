use proc_macro::TokenStream;
use quote::{quote, ToTokens};
use syn::{parse_macro_input, spanned::Spanned, Error, Fields, ItemStruct};

use crate::{deserialize::option_getter_from_type, get_array, get_option, is_nbt, is_variable};

pub fn deserialize_struct(item: TokenStream) -> TokenStream {
    let item_struct = parse_macro_input!(item as ItemStruct);
    let quote = if let Fields::Named(fields_named) = item_struct.fields {
        let mut fields_ident = Vec::new();
        let mut fields_value = Vec::new();

        for field_name in fields_named.named {
            let field_type = field_name.ty.to_token_stream();
            let variable = is_variable(&field_name.attrs);
            let array = get_array(&field_name.attrs);
            let option = get_option(&field_name.attrs);
            let nbt = is_nbt(&field_name.attrs);

            fields_ident.push(field_name.ident.unwrap());
            fields_value.push(option_getter_from_type(
                field_type,
                quote!(packet),
                variable,
                array,
                option,
                nbt,
            ));
        }

        let item_struct_ident = item_struct.ident;

        quote! {
            impl TryFrom<&mut Vec<u8>> for #item_struct_ident {
                type Error = packet::Error;

                fn try_from(packet: &mut Vec<u8>) -> Result<Self, Self::Error> {
                    use packet::{FromByte, FromShort, FromInt, FromLong, FromString, FromUuid, FromVarInt, FromVarLong};

                    #( let #fields_ident = #fields_value; )*

                    Ok(Self {
                        #( #fields_ident, )*
                    })
                }
            }
        }
    } else {
        Error::new(item_struct.span(), "Use only on struct").into_compile_error()
    };

    TokenStream::from(quote)
}
