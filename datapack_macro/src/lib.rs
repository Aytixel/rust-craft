extern crate proc_macro;

use proc_macro::TokenStream;
use quote::quote;
use syn::DeriveInput;

#[proc_macro_derive(DeserializeJsonFolder)]
pub fn deserialize_json_folder_macro(input: TokenStream) -> TokenStream {
    let ast = &syn::parse(input).unwrap();

    impl_deserialize_json_folder_macro(ast)
}

fn impl_deserialize_json_folder_macro(ast: &DeriveInput) -> TokenStream {
    let name = &ast.ident;
    let gen = quote! {
        impl #name {
            pub fn deserialize_json_folder(path: &str) -> Result<hashbrown::hash_map::HashMap<String, #name>, String>
            {
                #name::deserialize_json_folder_(path, "".to_string())
            }

            fn deserialize_json_folder_(path: &str, parent: String) -> Result<hashbrown::hash_map::HashMap<String, #name>, String> {
                let mut hashmap = hashbrown::hash_map::HashMap::new();
                let parent = parent + "/";

                for file in std::fs::read_dir(path).unwrap() {
                    if let Ok(file) = file {
                        if file.file_type().unwrap().is_dir() {
                            for (key, value) in #name::deserialize_json_folder_(
                                file.path().into_os_string().as_os_str().to_str().unwrap(),
                                parent.clone() + file.path().file_name().unwrap().to_str().unwrap(),
                            )?
                            .drain()
                            {
                                hashmap.insert(key, value);
                            }
                        } else {
                            let file_name = file.file_name();
                            let file_name = file_name.to_str().unwrap();

                            if file_name.ends_with(".json") {
                                let file = std::fs::File::open(file.path()).map_err(|e| e.to_string())?;
                                let reader = std::io::BufReader::new(file);

                                hashmap.insert(
                                    parent.clone() + &file_name[..file_name.len() - 5],
                                    serde_json::from_reader(reader).map_err(|e| e.to_string())?,
                                );
                            }
                        }
                    }
                }

                Ok(hashmap)
            }
        }
    };

    gen.into()
}

#[proc_macro_derive(DeserializeNbtFolder)]
pub fn deserialize_nbt_folder_macro(input: TokenStream) -> TokenStream {
    let ast = &syn::parse(input).unwrap();

    impl_deserialize_nbt_folder_macro(ast)
}

fn impl_deserialize_nbt_folder_macro(ast: &DeriveInput) -> TokenStream {
    let name = &ast.ident;
    let gen = quote! {
        impl #name {
            pub fn deserialize_nbt_folder(path: &str) -> Result<hashbrown::hash_map::HashMap<String, #name>, String>
            {
                #name::deserialize_nbt_folder_(path, "".to_string())
            }

            fn deserialize_nbt_folder_(path: &str, parent: String) -> Result<hashbrown::hash_map::HashMap<String, #name>, String> {
                let mut hashmap = hashbrown::hash_map::HashMap::new();
                let parent = parent + "/";

                for file in std::fs::read_dir(path).unwrap() {
                    if let Ok(file) = file {
                        if file.file_type().unwrap().is_dir() {
                            for (key, value) in #name::deserialize_nbt_folder_(
                                file.path().into_os_string().as_os_str().to_str().unwrap(),
                                parent.clone() + file.path().file_name().unwrap().to_str().unwrap(),
                            )?
                            .drain()
                            {
                                hashmap.insert(key, value);
                            }
                        } else {
                            let file_name = file.file_name();
                            let file_name = file_name.to_str().unwrap();

                            if file_name.ends_with(".nbt") {
                                let file = std::fs::File::open(file.path()).map_err(|e| e.to_string())?;
                                let mut reader = std::io::BufReader::new(file);

                                hashmap.insert(
                                    parent.clone() + &file_name[..file_name.len() - 4],
                                    quartz_nbt::serde::deserialize_from(
                                        &mut reader,
                                        quartz_nbt::io::Flavor::GzCompressed,
                                    )
                                    .map_err(|e| e.to_string())?
                                    .0,
                                );
                            }
                        }
                    }
                }

                Ok(hashmap)
            }
        }
    };

    gen.into()
}
