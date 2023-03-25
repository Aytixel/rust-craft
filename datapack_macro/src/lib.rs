extern crate proc_macro;

use proc_macro::TokenStream;
use quote::quote;
use syn::DeriveInput;

#[proc_macro_derive(DeserializeFolder)]
pub fn deserialize_folder_macro(input: TokenStream) -> TokenStream {
    let ast = &syn::parse(input).unwrap();

    impl_deserialize_folder_macro(ast)
}

fn impl_deserialize_folder_macro(ast: &DeriveInput) -> TokenStream {
    let name = &ast.ident;
    let gen = quote! {
        impl #name {
            pub fn deserialize_folder(path: &str) -> Result<std::collections::HashMap<String, #name>, String>
            {
                let mut hashmap = std::collections::HashMap::new();

                for file in std::fs::read_dir(path).unwrap() {
                    if let Ok(file) = file {
                        let file_name = file.file_name();
                        let file_name = file_name.to_str().unwrap();

                        if file_name.ends_with(".json") {
                            let file = std::fs::File::open(file.path()).map_err(|e| e.to_string())?;
                            let reader = std::io::BufReader::new(file);

                            hashmap.insert(
                                file_name[..file_name.len() - 5].to_string(),
                                serde_json::from_reader(reader).map_err(|e| e.to_string())?,
                            );
                        }
                    }
                }

                Ok(hashmap)
            }
        }
    };

    gen.into()
}
