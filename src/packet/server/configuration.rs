use packet::{packet_enum, SerializePacket};
use uuid::Uuid;

use crate::r#struct::{RegistryCodec, Tags, TextComponent};

packet_enum! { ServerConfiguration
    #[derive(Debug, SerializePacket)]
    #[id(0x00)]
    pub struct PluginMessage {
        pub channel: String,
        pub data: Vec<u8>,
    }

    #[derive(Debug, SerializePacket)]
    #[id(0x01)]
    pub struct Disconnect {
        #[nbt]
        pub reason: TextComponent,
    }

    #[derive(Debug, SerializePacket)]
    #[id(0x02)]
    pub struct FinishConfiguration {}

    #[derive(Debug, SerializePacket)]
    #[id(0x03)]
    pub struct KeepAlive {
        pub keep_alive_id: i64,
    }

    #[derive(Debug, SerializePacket)]
    #[id(0x04)]
    pub struct Ping {
        pub id: i32,
    }

    #[derive(Debug, SerializePacket)]
    #[id(0x05)]
    pub struct RegistryData {
        #[nbt]
        pub registery_codec: RegistryCodec,
    }

    #[derive(Debug, SerializePacket)]
    #[id(0x06)]
    pub struct RemoveResourcePack {
        pub has_uuid: bool,
        #[option(has_uuid)]
        pub uuid: Option<Uuid>,
    }

    #[derive(Debug, SerializePacket)]
    #[id(0x07)]
    pub struct AddResourcePack {
        pub uuid: Uuid,
        pub url: String,
        pub hash: String,
        pub forced: bool,
        pub has_prompt_message: bool,
        #[option(has_prompt_message)]
        #[nbt]
        pub prompt_message: Option<TextComponent>
    }

    #[derive(Debug, SerializePacket)]
    #[id(0x08)]
    pub struct FeatureFlags {
        #[variable]
        pub total_features: i32,
        #[array(total_features)]
        pub feature_flags: Vec<String>,
    }

    #[derive(Debug, SerializePacket)]
    #[id(0x09)]
    pub struct UpdateTags {
        #[variable]
        pub length_of_the_array: i32,
        #[array(length_of_the_array)]
        pub array_of_tags: Vec<Tags>,
    }
}
