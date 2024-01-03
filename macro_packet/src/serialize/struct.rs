use proc_macro::TokenStream;
use quote::{quote, ToTokens};
use syn::{parse_macro_input, spanned::Spanned, Error, Fields, ItemStruct};

use crate::{get_array, is_variable, serialize::option_setter_from_type};

pub fn serialize_struct(item: TokenStream) -> TokenStream {
    let item_struct = parse_macro_input!(item as ItemStruct);
    let quote = if let Fields::Named(fields_named) = item_struct.fields {
        let mut fields = Vec::new();

        for field_name in fields_named.named {
            let field_type = field_name.ty.to_token_stream();
            let field_ident = field_name.ident.unwrap();
            let variable = is_variable(&field_name.attrs);
            let array = get_array(&field_name.attrs);

            fields.push(option_setter_from_type(
                field_type,
                quote! { packet.#field_ident },
                variable,
                array.map(|array| quote!(packet.#array)),
            ));
        }

        let item_struct_ident = item_struct.ident;

        quote! {
            impl From<#item_struct_ident> for Vec<u8> {
                fn from(packet: #item_struct_ident) -> Self {
                    use packet::{ToByte, ToShort, ToInt, ToLong, ToString, ToUuid, ToVarInt, ToVarLong};

                    let mut data: Vec<u8> = Vec::new();

                    #( data.extend(#fields); )*

                    data
                }
            }
        }
    } else {
        Error::new(item_struct.span(), "Use only on struct").into_compile_error()
    };

    TokenStream::from(quote)
}
