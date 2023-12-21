use quartz_nbt::{io::Flavor, serde::serialize, snbt, NbtCompound};

use crate::{
    data_type::{Packet, ToByte, ToLong, ToString, ToVarInt},
    packet::ClientPlayPacketId,
};

#[derive(Debug)]
pub struct LoginPacket {
    shared_secret: Vec<u8>,
    verify_token: Vec<u8>,
}

impl LoginPacket {
    pub fn new() -> Packet {
        let mut registry_codec: NbtCompound = snbt::parse(
            "
        {
            \"minecraft:chat_type\": {
              \"type\": \"minecraft:chat_type\",
              \"value\": [
                {
                  \"element\": {
                    \"chat\": {
                      \"parameters\": [
                        \"sender\",
                        \"content\"
                      ],
                      \"translation_key\": \"chat.type.text\"
                    },
                    \"narration\": {
                      \"parameters\": [
                        \"sender\",
                        \"content\"
                      ],
                      \"translation_key\": \"chat.type.text.narrate\"
                    }
                  },
                  \"id\": {
                    \"value\": 0
                  },
                  \"name\": \"minecraft:chat\"
                },
                {
                  \"element\": {
                    \"chat\": {
                      \"parameters\": [
                        \"sender\",
                        \"content\"
                      ],
                      \"translation_key\": \"chat.type.announcement\"
                    },
                    \"narration\": {
                      \"parameters\": [
                        \"sender\",
                        \"content\"
                      ],
                      \"translation_key\": \"chat.type.text.narrate\"
                    }
                  },
                  \"id\": {
                    \"value\": 1
                  },
                  \"name\": \"minecraft:say_command\"
                },
                {
                  \"element\": {
                    \"chat\": {
                      \"parameters\": [
                        \"sender\",
                        \"content\"
                      ],
                      \"style\": {
                        \"color\": \"gray\",
                        \"italic\": {
                          \"value\": 1
                        }
                      },
                      \"translation_key\": \"commands.message.display.incoming\"
                    },
                    \"narration\": {
                      \"parameters\": [
                        \"sender\",
                        \"content\"
                      ],
                      \"translation_key\": \"chat.type.text.narrate\"
                    }
                  },
                  \"id\": {
                    \"value\": 2
                  },
                  \"name\": \"minecraft:msg_command_incoming\"
                },
                {
                  \"element\": {
                    \"chat\": {
                      \"parameters\": [
                        \"target\",
                        \"content\"
                      ],
                      \"style\": {
                        \"color\": \"gray\",
                        \"italic\": {
                          \"value\": 1
                        }
                      },
                      \"translation_key\": \"commands.message.display.outgoing\"
                    },
                    \"narration\": {
                      \"parameters\": [
                        \"sender\",
                        \"content\"
                      ],
                      \"translation_key\": \"chat.type.text.narrate\"
                    }
                  },
                  \"id\": {
                    \"value\": 3
                  },
                  \"name\": \"minecraft:msg_command_outgoing\"
                },
                {
                  \"element\": {
                    \"chat\": {
                      \"parameters\": [
                        \"target\",
                        \"sender\",
                        \"content\"
                      ],
                      \"translation_key\": \"chat.type.team.text\"
                    },
                    \"narration\": {
                      \"parameters\": [
                        \"sender\",
                        \"content\"
                      ],
                      \"translation_key\": \"chat.type.text.narrate\"
                    }
                  },
                  \"id\": {
                    \"value\": 4
                  },
                  \"name\": \"minecraft:team_msg_command_incoming\"
                },
                {
                  \"element\": {
                    \"chat\": {
                      \"parameters\": [
                        \"target\",
                        \"sender\",
                        \"content\"
                      ],
                      \"translation_key\": \"chat.type.team.sent\"
                    },
                    \"narration\": {
                      \"parameters\": [
                        \"sender\",
                        \"content\"
                      ],
                      \"translation_key\": \"chat.type.text.narrate\"
                    }
                  },
                  \"id\": {
                    \"value\": 5
                  },
                  \"name\": \"minecraft:team_msg_command_outgoing\"
                },
                {
                  \"element\": {
                    \"chat\": {
                      \"parameters\": [
                        \"sender\",
                        \"content\"
                      ],
                      \"translation_key\": \"chat.type.emote\"
                    },
                    \"narration\": {
                      \"parameters\": [
                        \"sender\",
                        \"content\"
                      ],
                      \"translation_key\": \"chat.type.emote\"
                    }
                  },
                  \"id\": {
                    \"value\": 6
                  },
                  \"name\": \"minecraft:emote_command\"
                }
              ]
            },
            \"minecraft:dimension_type\": {
              \"type\": \"minecraft:dimension_type\",
              \"value\": [
                {
                  \"element\": {
                    \"ambient_light\": {
                      \"value\": 0
                    },
                    \"bed_works\": {
                      \"value\": 1
                    },
                    \"coordinate_scale\": {
                      \"value\": 1
                    },
                    \"effects\": \"minecraft:overworld\",
                    \"has_ceiling\": {
                      \"value\": 0
                    },
                    \"has_raids\": {
                      \"value\": 1
                    },
                    \"has_skylight\": {
                      \"value\": 1
                    },
                    \"height\": {
                      \"value\": 384
                    },
                    \"infiniburn\": \"#minecraft:infiniburn_overworld\",
                    \"logical_height\": {
                      \"value\": 384
                    },
                    \"min_y\": {
                      \"value\": -64
                    },
                    \"monster_spawn_block_light_limit\": {
                      \"value\": 0
                    },
                    \"monster_spawn_light_level\": {
                      \"type\": \"minecraft:uniform\",
                      \"value\": {
                        \"max_inclusive\": {
                          \"value\": 7
                        },
                        \"min_inclusive\": {
                          \"value\": 0
                        }
                      }
                    },
                    \"natural\": {
                      \"value\": 1
                    },
                    \"piglin_safe\": {
                      \"value\": 0
                    },
                    \"respawn_anchor_works\": {
                      \"value\": 0
                    },
                    \"ultrawarm\": {
                      \"value\": 0
                    }
                  },
                  \"id\": {
                    \"value\": 0
                  },
                  \"name\": \"minecraft:overworld\"
                },
                {
                  \"element\": {
                    \"ambient_light\": {
                      \"value\": 0.10000000149011612
                    },
                    \"bed_works\": {
                      \"value\": 0
                    },
                    \"coordinate_scale\": {
                      \"value\": 8
                    },
                    \"effects\": \"minecraft:the_nether\",
                    \"fixed_time\": 18000,
                    \"has_ceiling\": {
                      \"value\": 1
                    },
                    \"has_raids\": {
                      \"value\": 0
                    },
                    \"has_skylight\": {
                      \"value\": 0
                    },
                    \"height\": {
                      \"value\": 256
                    },
                    \"infiniburn\": \"#minecraft:infiniburn_nether\",
                    \"logical_height\": {
                      \"value\": 128
                    },
                    \"min_y\": {
                      \"value\": 0
                    },
                    \"monster_spawn_block_light_limit\": {
                      \"value\": 15
                    },
                    \"monster_spawn_light_level\": {
                      \"value\": 11
                    },
                    \"natural\": {
                      \"value\": 0
                    },
                    \"piglin_safe\": {
                      \"value\": 1
                    },
                    \"respawn_anchor_works\": {
                      \"value\": 1
                    },
                    \"ultrawarm\": {
                      \"value\": 1
                    }
                  },
                  \"id\": {
                    \"value\": 1
                  },
                  \"name\": \"minecraft:the_nether\"
                },
                {
                  \"element\": {
                    \"ambient_light\": {
                      \"value\": 0
                    },
                    \"bed_works\": {
                      \"value\": 0
                    },
                    \"coordinate_scale\": {
                      \"value\": 1
                    },
                    \"effects\": \"minecraft:the_end\",
                    \"fixed_time\": 6000,
                    \"has_ceiling\": {
                      \"value\": 0
                    },
                    \"has_raids\": {
                      \"value\": 1
                    },
                    \"has_skylight\": {
                      \"value\": 0
                    },
                    \"height\": {
                      \"value\": 256
                    },
                    \"infiniburn\": \"#minecraft:infiniburn_end\",
                    \"logical_height\": {
                      \"value\": 256
                    },
                    \"min_y\": {
                      \"value\": 0
                    },
                    \"monster_spawn_block_light_limit\": {
                      \"value\": 0
                    },
                    \"monster_spawn_light_level\": {
                      \"type\": \"minecraft:uniform\",
                      \"value\": {
                        \"max_inclusive\": {
                          \"value\": 7
                        },
                        \"min_inclusive\": {
                          \"value\": 0
                        }
                      }
                    },
                    \"natural\": {
                      \"value\": 0
                    },
                    \"piglin_safe\": {
                      \"value\": 0
                    },
                    \"respawn_anchor_works\": {
                      \"value\": 0
                    },
                    \"ultrawarm\": {
                      \"value\": 0
                    }
                  },
                  \"id\": {
                    \"value\": 2
                  },
                  \"name\": \"minecraft:the_end\"
                },
                {
                  \"element\": {
                    \"ambient_light\": {
                      \"value\": 0
                    },
                    \"bed_works\": {
                      \"value\": 1
                    },
                    \"coordinate_scale\": {
                      \"value\": 1
                    },
                    \"effects\": \"minecraft:overworld\",
                    \"has_ceiling\": {
                      \"value\": 1
                    },
                    \"has_raids\": {
                      \"value\": 1
                    },
                    \"has_skylight\": {
                      \"value\": 1
                    },
                    \"height\": {
                      \"value\": 384
                    },
                    \"infiniburn\": \"#minecraft:infiniburn_overworld\",
                    \"logical_height\": {
                      \"value\": 384
                    },
                    \"min_y\": {
                      \"value\": -64
                    },
                    \"monster_spawn_block_light_limit\": {
                      \"value\": 0
                    },
                    \"monster_spawn_light_level\": {
                      \"type\": \"minecraft:uniform\",
                      \"value\": {
                        \"max_inclusive\": {
                          \"value\": 7
                        },
                        \"min_inclusive\": {
                          \"value\": 0
                        }
                      }
                    },
                    \"natural\": {
                      \"value\": 1
                    },
                    \"piglin_safe\": {
                      \"value\": 0
                    },
                    \"respawn_anchor_works\": {
                      \"value\": 0
                    },
                    \"ultrawarm\": {
                      \"value\": 0
                    }
                  },
                  \"id\": {
                    \"value\": 3
                  },
                  \"name\": \"minecraft:overworld_caves\"
                }
              ]
            },
            \"minecraft:worldgen/biome\": {
              \"type\": \"minecraft:worldgen/biome\",
              \"value\": [
                {
                  \"element\": {
                    \"downfall\": {
                      \"value\": 0.5
                    },
                    \"effects\": {
                      \"fog_color\": {
                        \"value\": 12638463
                      },
                      \"mood_sound\": {
                        \"block_search_extent\": {
                          \"value\": 8
                        },
                        \"offset\": {
                          \"value\": 2
                        },
                        \"sound\": \"minecraft:ambient.cave\",
                        \"tick_delay\": {
                          \"value\": 6000
                        }
                      },
                      \"sky_color\": {
                        \"value\": 8103167
                      },
                      \"water_color\": {
                        \"value\": 4159204
                      },
                      \"water_fog_color\": {
                        \"value\": 329011
                      }
                    },
                    \"precipitation\": \"none\",
                    \"temperature\": {
                      \"value\": 0.5
                    }
                  },
                  \"id\": {
                    \"value\": 0
                  },
                  \"name\": \"minecraft:the_void\"
                },
                {
                  \"element\": {
                    \"downfall\": {
                      \"value\": 0.4000000059604645
                    },
                    \"effects\": {
                      \"fog_color\": {
                        \"value\": 12638463
                      },
                      \"mood_sound\": {
                        \"block_search_extent\": {
                          \"value\": 8
                        },
                        \"offset\": {
                          \"value\": 2
                        },
                        \"sound\": \"minecraft:ambient.cave\",
                        \"tick_delay\": {
                          \"value\": 6000
                        }
                      },
                      \"sky_color\": {
                        \"value\": 7907327
                      },
                      \"water_color\": {
                        \"value\": 4159204
                      },
                      \"water_fog_color\": {
                        \"value\": 329011
                      }
                    },
                    \"precipitation\": \"rain\",
                    \"temperature\": {
                      \"value\": 0.800000011920929
                    }
                  },
                  \"id\": {
                    \"value\": 1
                  },
                  \"name\": \"minecraft:plains\"
                },
                {
                  \"element\": {
                    \"downfall\": {
                      \"value\": 0.4000000059604645
                    },
                    \"effects\": {
                      \"fog_color\": {
                        \"value\": 12638463
                      },
                      \"mood_sound\": {
                        \"block_search_extent\": {
                          \"value\": 8
                        },
                        \"offset\": {
                          \"value\": 2
                        },
                        \"sound\": \"minecraft:ambient.cave\",
                        \"tick_delay\": {
                          \"value\": 6000
                        }
                      },
                      \"sky_color\": {
                        \"value\": 7907327
                      },
                      \"water_color\": {
                        \"value\": 4159204
                      },
                      \"water_fog_color\": {
                        \"value\": 329011
                      }
                    },
                    \"precipitation\": \"rain\",
                    \"temperature\": {
                      \"value\": 0.800000011920929
                    }
                  },
                  \"id\": {
                    \"value\": 2
                  },
                  \"name\": \"minecraft:sunflower_plains\"
                },
                {
                  \"element\": {
                    \"downfall\": {
                      \"value\": 0.5
                    },
                    \"effects\": {
                      \"fog_color\": {
                        \"value\": 12638463
                      },
                      \"mood_sound\": {
                        \"block_search_extent\": {
                          \"value\": 8
                        },
                        \"offset\": {
                          \"value\": 2
                        },
                        \"sound\": \"minecraft:ambient.cave\",
                        \"tick_delay\": {
                          \"value\": 6000
                        }
                      },
                      \"sky_color\": {
                        \"value\": 8364543
                      },
                      \"water_color\": {
                        \"value\": 4159204
                      },
                      \"water_fog_color\": {
                        \"value\": 329011
                      }
                    },
                    \"precipitation\": \"snow\",
                    \"temperature\": {
                      \"value\": 0
                    }
                  },
                  \"id\": {
                    \"value\": 3
                  },
                  \"name\": \"minecraft:snowy_plains\"
                },
                {
                  \"element\": {
                    \"downfall\": {
                      \"value\": 0.5
                    },
                    \"effects\": {
                      \"fog_color\": {
                        \"value\": 12638463
                      },
                      \"mood_sound\": {
                        \"block_search_extent\": {
                          \"value\": 8
                        },
                        \"offset\": {
                          \"value\": 2
                        },
                        \"sound\": \"minecraft:ambient.cave\",
                        \"tick_delay\": {
                          \"value\": 6000
                        }
                      },
                      \"sky_color\": {
                        \"value\": 8364543
                      },
                      \"water_color\": {
                        \"value\": 4159204
                      },
                      \"water_fog_color\": {
                        \"value\": 329011
                      }
                    },
                    \"precipitation\": \"snow\",
                    \"temperature\": {
                      \"value\": 0
                    }
                  },
                  \"id\": {
                    \"value\": 4
                  },
                  \"name\": \"minecraft:ice_spikes\"
                },
                {
                  \"element\": {
                    \"downfall\": {
                      \"value\": 0
                    },
                    \"effects\": {
                      \"fog_color\": {
                        \"value\": 12638463
                      },
                      \"mood_sound\": {
                        \"block_search_extent\": {
                          \"value\": 8
                        },
                        \"offset\": {
                          \"value\": 2
                        },
                        \"sound\": \"minecraft:ambient.cave\",
                        \"tick_delay\": {
                          \"value\": 6000
                        }
                      },
                      \"sky_color\": {
                        \"value\": 7254527
                      },
                      \"water_color\": {
                        \"value\": 4159204
                      },
                      \"water_fog_color\": {
                        \"value\": 329011
                      }
                    },
                    \"precipitation\": \"none\",
                    \"temperature\": {
                      \"value\": 2
                    }
                  },
                  \"id\": {
                    \"value\": 5
                  },
                  \"name\": \"minecraft:desert\"
                },
                {
                  \"element\": {
                    \"downfall\": {
                      \"value\": 0.8999999761581421
                    },
                    \"effects\": {
                      \"fog_color\": {
                        \"value\": 12638463
                      },
                      \"foliage_color\": {
                        \"value\": 6975545
                      },
                      \"grass_color_modifier\": \"swamp\",
                      \"mood_sound\": {
                        \"block_search_extent\": {
                          \"value\": 8
                        },
                        \"offset\": {
                          \"value\": 2
                        },
                        \"sound\": \"minecraft:ambient.cave\",
                        \"tick_delay\": {
                          \"value\": 6000
                        }
                      },
                      \"music\": {
                        \"max_delay\": {
                          \"value\": 24000
                        },
                        \"min_delay\": {
                          \"value\": 12000
                        },
                        \"replace_current_music\": {
                          \"value\": 0
                        },
                        \"sound\": \"minecraft:music.overworld.swamp\"
                      },
                      \"sky_color\": {
                        \"value\": 7907327
                      },
                      \"water_color\": {
                        \"value\": 6388580
                      },
                      \"water_fog_color\": {
                        \"value\": 2302743
                      }
                    },
                    \"precipitation\": \"rain\",
                    \"temperature\": {
                      \"value\": 0.800000011920929
                    }
                  },
                  \"id\": {
                    \"value\": 6
                  },
                  \"name\": \"minecraft:swamp\"
                },
                {
                  \"element\": {
                    \"downfall\": {
                      \"value\": 0.8999999761581421
                    },
                    \"effects\": {
                      \"fog_color\": {
                        \"value\": 12638463
                      },
                      \"foliage_color\": {
                        \"value\": 9285927
                      },
                      \"grass_color_modifier\": \"swamp\",
                      \"mood_sound\": {
                        \"block_search_extent\": {
                          \"value\": 8
                        },
                        \"offset\": {
                          \"value\": 2
                        },
                        \"sound\": \"minecraft:ambient.cave\",
                        \"tick_delay\": {
                          \"value\": 6000
                        }
                      },
                      \"music\": {
                        \"max_delay\": {
                          \"value\": 24000
                        },
                        \"min_delay\": {
                          \"value\": 12000
                        },
                        \"replace_current_music\": {
                          \"value\": 0
                        },
                        \"sound\": \"minecraft:music.overworld.swamp\"
                      },
                      \"sky_color\": {
                        \"value\": 7907327
                      },
                      \"water_color\": {
                        \"value\": 3832426
                      },
                      \"water_fog_color\": {
                        \"value\": 5077600
                      }
                    },
                    \"precipitation\": \"rain\",
                    \"temperature\": {
                      \"value\": 0.800000011920929
                    }
                  },
                  \"id\": {
                    \"value\": 7
                  },
                  \"name\": \"minecraft:mangrove_swamp\"
                },
                {
                  \"element\": {
                    \"downfall\": {
                      \"value\": 0.800000011920929
                    },
                    \"effects\": {
                      \"fog_color\": {
                        \"value\": 12638463
                      },
                      \"mood_sound\": {
                        \"block_search_extent\": {
                          \"value\": 8
                        },
                        \"offset\": {
                          \"value\": 2
                        },
                        \"sound\": \"minecraft:ambient.cave\",
                        \"tick_delay\": {
                          \"value\": 6000
                        }
                      },
                      \"music\": {
                        \"max_delay\": {
                          \"value\": 24000
                        },
                        \"min_delay\": {
                          \"value\": 12000
                        },
                        \"replace_current_music\": {
                          \"value\": 0
                        },
                        \"sound\": \"minecraft:music.overworld.jungle_and_forest\"
                      },
                      \"sky_color\": {
                        \"value\": 7972607
                      },
                      \"water_color\": {
                        \"value\": 4159204
                      },
                      \"water_fog_color\": {
                        \"value\": 329011
                      }
                    },
                    \"precipitation\": \"rain\",
                    \"temperature\": {
                      \"value\": 0.699999988079071
                    }
                  },
                  \"id\": {
                    \"value\": 8
                  },
                  \"name\": \"minecraft:forest\"
                },
                {
                  \"element\": {
                    \"downfall\": {
                      \"value\": 0.800000011920929
                    },
                    \"effects\": {
                      \"fog_color\": {
                        \"value\": 12638463
                      },
                      \"mood_sound\": {
                        \"block_search_extent\": {
                          \"value\": 8
                        },
                        \"offset\": {
                          \"value\": 2
                        },
                        \"sound\": \"minecraft:ambient.cave\",
                        \"tick_delay\": {
                          \"value\": 6000
                        }
                      },
                      \"music\": {
                        \"max_delay\": {
                          \"value\": 24000
                        },
                        \"min_delay\": {
                          \"value\": 12000
                        },
                        \"replace_current_music\": {
                          \"value\": 0
                        },
                        \"sound\": \"minecraft:music.overworld.jungle_and_forest\"
                      },
                      \"sky_color\": {
                        \"value\": 7972607
                      },
                      \"water_color\": {
                        \"value\": 4159204
                      },
                      \"water_fog_color\": {
                        \"value\": 329011
                      }
                    },
                    \"precipitation\": \"rain\",
                    \"temperature\": {
                      \"value\": 0.699999988079071
                    }
                  },
                  \"id\": {
                    \"value\": 9
                  },
                  \"name\": \"minecraft:flower_forest\"
                },
                {
                  \"element\": {
                    \"downfall\": {
                      \"value\": 0.6000000238418579
                    },
                    \"effects\": {
                      \"fog_color\": {
                        \"value\": 12638463
                      },
                      \"mood_sound\": {
                        \"block_search_extent\": {
                          \"value\": 8
                        },
                        \"offset\": {
                          \"value\": 2
                        },
                        \"sound\": \"minecraft:ambient.cave\",
                        \"tick_delay\": {
                          \"value\": 6000
                        }
                      },
                      \"music\": {
                        \"max_delay\": {
                          \"value\": 24000
                        },
                        \"min_delay\": {
                          \"value\": 12000
                        },
                        \"replace_current_music\": {
                          \"value\": 0
                        },
                        \"sound\": \"minecraft:music.overworld.jungle_and_forest\"
                      },
                      \"sky_color\": {
                        \"value\": 8037887
                      },
                      \"water_color\": {
                        \"value\": 4159204
                      },
                      \"water_fog_color\": {
                        \"value\": 329011
                      }
                    },
                    \"precipitation\": \"rain\",
                    \"temperature\": {
                      \"value\": 0.6000000238418579
                    }
                  },
                  \"id\": {
                    \"value\": 10
                  },
                  \"name\": \"minecraft:birch_forest\"
                },
                {
                  \"element\": {
                    \"downfall\": {
                      \"value\": 0.800000011920929
                    },
                    \"effects\": {
                      \"fog_color\": {
                        \"value\": 12638463
                      },
                      \"grass_color_modifier\": \"dark_forest\",
                      \"mood_sound\": {
                        \"block_search_extent\": {
                          \"value\": 8
                        },
                        \"offset\": {
                          \"value\": 2
                        },
                        \"sound\": \"minecraft:ambient.cave\",
                        \"tick_delay\": {
                          \"value\": 6000
                        }
                      },
                      \"music\": {
                        \"max_delay\": {
                          \"value\": 24000
                        },
                        \"min_delay\": {
                          \"value\": 12000
                        },
                        \"replace_current_music\": {
                          \"value\": 0
                        },
                        \"sound\": \"minecraft:music.overworld.jungle_and_forest\"
                      },
                      \"sky_color\": {
                        \"value\": 7972607
                      },
                      \"water_color\": {
                        \"value\": 4159204
                      },
                      \"water_fog_color\": {
                        \"value\": 329011
                      }
                    },
                    \"precipitation\": \"rain\",
                    \"temperature\": {
                      \"value\": 0.699999988079071
                    }
                  },
                  \"id\": {
                    \"value\": 11
                  },
                  \"name\": \"minecraft:dark_forest\"
                },
                {
                  \"element\": {
                    \"downfall\": {
                      \"value\": 0.6000000238418579
                    },
                    \"effects\": {
                      \"fog_color\": {
                        \"value\": 12638463
                      },
                      \"mood_sound\": {
                        \"block_search_extent\": {
                          \"value\": 8
                        },
                        \"offset\": {
                          \"value\": 2
                        },
                        \"sound\": \"minecraft:ambient.cave\",
                        \"tick_delay\": {
                          \"value\": 6000
                        }
                      },
                      \"music\": {
                        \"max_delay\": {
                          \"value\": 24000
                        },
                        \"min_delay\": {
                          \"value\": 12000
                        },
                        \"replace_current_music\": {
                          \"value\": 0
                        },
                        \"sound\": \"minecraft:music.overworld.jungle_and_forest\"
                      },
                      \"sky_color\": {
                        \"value\": 8037887
                      },
                      \"water_color\": {
                        \"value\": 4159204
                      },
                      \"water_fog_color\": {
                        \"value\": 329011
                      }
                    },
                    \"precipitation\": \"rain\",
                    \"temperature\": {
                      \"value\": 0.6000000238418579
                    }
                  },
                  \"id\": {
                    \"value\": 12
                  },
                  \"name\": \"minecraft:old_growth_birch_forest\"
                },
                {
                  \"element\": {
                    \"downfall\": {
                      \"value\": 0.800000011920929
                    },
                    \"effects\": {
                      \"fog_color\": {
                        \"value\": 12638463
                      },
                      \"mood_sound\": {
                        \"block_search_extent\": {
                          \"value\": 8
                        },
                        \"offset\": {
                          \"value\": 2
                        },
                        \"sound\": \"minecraft:ambient.cave\",
                        \"tick_delay\": {
                          \"value\": 6000
                        }
                      },
                      \"music\": {
                        \"max_delay\": {
                          \"value\": 24000
                        },
                        \"min_delay\": {
                          \"value\": 12000
                        },
                        \"replace_current_music\": {
                          \"value\": 0
                        },
                        \"sound\": \"minecraft:music.overworld.old_growth_taiga\"
                      },
                      \"sky_color\": {
                        \"value\": 8168447
                      },
                      \"water_color\": {
                        \"value\": 4159204
                      },
                      \"water_fog_color\": {
                        \"value\": 329011
                      }
                    },
                    \"precipitation\": \"rain\",
                    \"temperature\": {
                      \"value\": 0.30000001192092896
                    }
                  },
                  \"id\": {
                    \"value\": 13
                  },
                  \"name\": \"minecraft:old_growth_pine_taiga\"
                },
                {
                  \"element\": {
                    \"downfall\": {
                      \"value\": 0.800000011920929
                    },
                    \"effects\": {
                      \"fog_color\": {
                        \"value\": 12638463
                      },
                      \"mood_sound\": {
                        \"block_search_extent\": {
                          \"value\": 8
                        },
                        \"offset\": {
                          \"value\": 2
                        },
                        \"sound\": \"minecraft:ambient.cave\",
                        \"tick_delay\": {
                          \"value\": 6000
                        }
                      },
                      \"music\": {
                        \"max_delay\": {
                          \"value\": 24000
                        },
                        \"min_delay\": {
                          \"value\": 12000
                        },
                        \"replace_current_music\": {
                          \"value\": 0
                        },
                        \"sound\": \"minecraft:music.overworld.old_growth_taiga\"
                      },
                      \"sky_color\": {
                        \"value\": 8233983
                      },
                      \"water_color\": {
                        \"value\": 4159204
                      },
                      \"water_fog_color\": {
                        \"value\": 329011
                      }
                    },
                    \"precipitation\": \"rain\",
                    \"temperature\": {
                      \"value\": 0.25
                    }
                  },
                  \"id\": {
                    \"value\": 14
                  },
                  \"name\": \"minecraft:old_growth_spruce_taiga\"
                },
                {
                  \"element\": {
                    \"downfall\": {
                      \"value\": 0.800000011920929
                    },
                    \"effects\": {
                      \"fog_color\": {
                        \"value\": 12638463
                      },
                      \"mood_sound\": {
                        \"block_search_extent\": {
                          \"value\": 8
                        },
                        \"offset\": {
                          \"value\": 2
                        },
                        \"sound\": \"minecraft:ambient.cave\",
                        \"tick_delay\": {
                          \"value\": 6000
                        }
                      },
                      \"sky_color\": {
                        \"value\": 8233983
                      },
                      \"water_color\": {
                        \"value\": 4159204
                      },
                      \"water_fog_color\": {
                        \"value\": 329011
                      }
                    },
                    \"precipitation\": \"rain\",
                    \"temperature\": {
                      \"value\": 0.25
                    }
                  },
                  \"id\": {
                    \"value\": 15
                  },
                  \"name\": \"minecraft:taiga\"
                },
                {
                  \"element\": {
                    \"downfall\": {
                      \"value\": 0.4000000059604645
                    },
                    \"effects\": {
                      \"fog_color\": {
                        \"value\": 12638463
                      },
                      \"mood_sound\": {
                        \"block_search_extent\": {
                          \"value\": 8
                        },
                        \"offset\": {
                          \"value\": 2
                        },
                        \"sound\": \"minecraft:ambient.cave\",
                        \"tick_delay\": {
                          \"value\": 6000
                        }
                      },
                      \"sky_color\": {
                        \"value\": 8625919
                      },
                      \"water_color\": {
                        \"value\": 4020182
                      },
                      \"water_fog_color\": {
                        \"value\": 329011
                      }
                    },
                    \"precipitation\": \"snow\",
                    \"temperature\": {
                      \"value\": -0.5
                    }
                  },
                  \"id\": {
                    \"value\": 16
                  },
                  \"name\": \"minecraft:snowy_taiga\"
                },
                {
                  \"element\": {
                    \"downfall\": {
                      \"value\": 0
                    },
                    \"effects\": {
                      \"fog_color\": {
                        \"value\": 12638463
                      },
                      \"mood_sound\": {
                        \"block_search_extent\": {
                          \"value\": 8
                        },
                        \"offset\": {
                          \"value\": 2
                        },
                        \"sound\": \"minecraft:ambient.cave\",
                        \"tick_delay\": {
                          \"value\": 6000
                        }
                      },
                      \"sky_color\": {
                        \"value\": 7254527
                      },
                      \"water_color\": {
                        \"value\": 4159204
                      },
                      \"water_fog_color\": {
                        \"value\": 329011
                      }
                    },
                    \"precipitation\": \"none\",
                    \"temperature\": {
                      \"value\": 2
                    }
                  },
                  \"id\": {
                    \"value\": 17
                  },
                  \"name\": \"minecraft:savanna\"
                },
                {
                  \"element\": {
                    \"downfall\": {
                      \"value\": 0
                    },
                    \"effects\": {
                      \"fog_color\": {
                        \"value\": 12638463
                      },
                      \"mood_sound\": {
                        \"block_search_extent\": {
                          \"value\": 8
                        },
                        \"offset\": {
                          \"value\": 2
                        },
                        \"sound\": \"minecraft:ambient.cave\",
                        \"tick_delay\": {
                          \"value\": 6000
                        }
                      },
                      \"sky_color\": {
                        \"value\": 7254527
                      },
                      \"water_color\": {
                        \"value\": 4159204
                      },
                      \"water_fog_color\": {
                        \"value\": 329011
                      }
                    },
                    \"precipitation\": \"none\",
                    \"temperature\": {
                      \"value\": 2
                    }
                  },
                  \"id\": {
                    \"value\": 18
                  },
                  \"name\": \"minecraft:savanna_plateau\"
                },
                {
                  \"element\": {
                    \"downfall\": {
                      \"value\": 0.30000001192092896
                    },
                    \"effects\": {
                      \"fog_color\": {
                        \"value\": 12638463
                      },
                      \"mood_sound\": {
                        \"block_search_extent\": {
                          \"value\": 8
                        },
                        \"offset\": {
                          \"value\": 2
                        },
                        \"sound\": \"minecraft:ambient.cave\",
                        \"tick_delay\": {
                          \"value\": 6000
                        }
                      },
                      \"sky_color\": {
                        \"value\": 8233727
                      },
                      \"water_color\": {
                        \"value\": 4159204
                      },
                      \"water_fog_color\": {
                        \"value\": 329011
                      }
                    },
                    \"precipitation\": \"rain\",
                    \"temperature\": {
                      \"value\": 0.20000000298023224
                    }
                  },
                  \"id\": {
                    \"value\": 19
                  },
                  \"name\": \"minecraft:windswept_hills\"
                },
                {
                  \"element\": {
                    \"downfall\": {
                      \"value\": 0.30000001192092896
                    },
                    \"effects\": {
                      \"fog_color\": {
                        \"value\": 12638463
                      },
                      \"mood_sound\": {
                        \"block_search_extent\": {
                          \"value\": 8
                        },
                        \"offset\": {
                          \"value\": 2
                        },
                        \"sound\": \"minecraft:ambient.cave\",
                        \"tick_delay\": {
                          \"value\": 6000
                        }
                      },
                      \"sky_color\": {
                        \"value\": 8233727
                      },
                      \"water_color\": {
                        \"value\": 4159204
                      },
                      \"water_fog_color\": {
                        \"value\": 329011
                      }
                    },
                    \"precipitation\": \"rain\",
                    \"temperature\": {
                      \"value\": 0.20000000298023224
                    }
                  },
                  \"id\": {
                    \"value\": 20
                  },
                  \"name\": \"minecraft:windswept_gravelly_hills\"
                },
                {
                  \"element\": {
                    \"downfall\": {
                      \"value\": 0.30000001192092896
                    },
                    \"effects\": {
                      \"fog_color\": {
                        \"value\": 12638463
                      },
                      \"mood_sound\": {
                        \"block_search_extent\": {
                          \"value\": 8
                        },
                        \"offset\": {
                          \"value\": 2
                        },
                        \"sound\": \"minecraft:ambient.cave\",
                        \"tick_delay\": {
                          \"value\": 6000
                        }
                      },
                      \"sky_color\": {
                        \"value\": 8233727
                      },
                      \"water_color\": {
                        \"value\": 4159204
                      },
                      \"water_fog_color\": {
                        \"value\": 329011
                      }
                    },
                    \"precipitation\": \"rain\",
                    \"temperature\": {
                      \"value\": 0.20000000298023224
                    }
                  },
                  \"id\": {
                    \"value\": 21
                  },
                  \"name\": \"minecraft:windswept_forest\"
                },
                {
                  \"element\": {
                    \"downfall\": {
                      \"value\": 0
                    },
                    \"effects\": {
                      \"fog_color\": {
                        \"value\": 12638463
                      },
                      \"mood_sound\": {
                        \"block_search_extent\": {
                          \"value\": 8
                        },
                        \"offset\": {
                          \"value\": 2
                        },
                        \"sound\": \"minecraft:ambient.cave\",
                        \"tick_delay\": {
                          \"value\": 6000
                        }
                      },
                      \"sky_color\": {
                        \"value\": 7254527
                      },
                      \"water_color\": {
                        \"value\": 4159204
                      },
                      \"water_fog_color\": {
                        \"value\": 329011
                      }
                    },
                    \"precipitation\": \"none\",
                    \"temperature\": {
                      \"value\": 2
                    }
                  },
                  \"id\": {
                    \"value\": 22
                  },
                  \"name\": \"minecraft:windswept_savanna\"
                },
                {
                  \"element\": {
                    \"downfall\": {
                      \"value\": 0.8999999761581421
                    },
                    \"effects\": {
                      \"fog_color\": {
                        \"value\": 12638463
                      },
                      \"mood_sound\": {
                        \"block_search_extent\": {
                          \"value\": 8
                        },
                        \"offset\": {
                          \"value\": 2
                        },
                        \"sound\": \"minecraft:ambient.cave\",
                        \"tick_delay\": {
                          \"value\": 6000
                        }
                      },
                      \"music\": {
                        \"max_delay\": {
                          \"value\": 24000
                        },
                        \"min_delay\": {
                          \"value\": 12000
                        },
                        \"replace_current_music\": {
                          \"value\": 0
                        },
                        \"sound\": \"minecraft:music.overworld.jungle_and_forest\"
                      },
                      \"sky_color\": {
                        \"value\": 7842047
                      },
                      \"water_color\": {
                        \"value\": 4159204
                      },
                      \"water_fog_color\": {
                        \"value\": 329011
                      }
                    },
                    \"precipitation\": \"rain\",
                    \"temperature\": {
                      \"value\": 0.949999988079071
                    }
                  },
                  \"id\": {
                    \"value\": 23
                  },
                  \"name\": \"minecraft:jungle\"
                },
                {
                  \"element\": {
                    \"downfall\": {
                      \"value\": 0.800000011920929
                    },
                    \"effects\": {
                      \"fog_color\": {
                        \"value\": 12638463
                      },
                      \"mood_sound\": {
                        \"block_search_extent\": {
                          \"value\": 8
                        },
                        \"offset\": {
                          \"value\": 2
                        },
                        \"sound\": \"minecraft:ambient.cave\",
                        \"tick_delay\": {
                          \"value\": 6000
                        }
                      },
                      \"music\": {
                        \"max_delay\": {
                          \"value\": 24000
                        },
                        \"min_delay\": {
                          \"value\": 12000
                        },
                        \"replace_current_music\": {
                          \"value\": 0
                        },
                        \"sound\": \"minecraft:music.overworld.jungle_and_forest\"
                      },
                      \"sky_color\": {
                        \"value\": 7842047
                      },
                      \"water_color\": {
                        \"value\": 4159204
                      },
                      \"water_fog_color\": {
                        \"value\": 329011
                      }
                    },
                    \"precipitation\": \"rain\",
                    \"temperature\": {
                      \"value\": 0.949999988079071
                    }
                  },
                  \"id\": {
                    \"value\": 24
                  },
                  \"name\": \"minecraft:sparse_jungle\"
                },
                {
                  \"element\": {
                    \"downfall\": {
                      \"value\": 0.8999999761581421
                    },
                    \"effects\": {
                      \"fog_color\": {
                        \"value\": 12638463
                      },
                      \"mood_sound\": {
                        \"block_search_extent\": {
                          \"value\": 8
                        },
                        \"offset\": {
                          \"value\": 2
                        },
                        \"sound\": \"minecraft:ambient.cave\",
                        \"tick_delay\": {
                          \"value\": 6000
                        }
                      },
                      \"music\": {
                        \"max_delay\": {
                          \"value\": 24000
                        },
                        \"min_delay\": {
                          \"value\": 12000
                        },
                        \"replace_current_music\": {
                          \"value\": 0
                        },
                        \"sound\": \"minecraft:music.overworld.jungle_and_forest\"
                      },
                      \"sky_color\": {
                        \"value\": 7842047
                      },
                      \"water_color\": {
                        \"value\": 4159204
                      },
                      \"water_fog_color\": {
                        \"value\": 329011
                      }
                    },
                    \"precipitation\": \"rain\",
                    \"temperature\": {
                      \"value\": 0.949999988079071
                    }
                  },
                  \"id\": {
                    \"value\": 25
                  },
                  \"name\": \"minecraft:bamboo_jungle\"
                },
                {
                  \"element\": {
                    \"downfall\": {
                      \"value\": 0
                    },
                    \"effects\": {
                      \"fog_color\": {
                        \"value\": 12638463
                      },
                      \"foliage_color\": {
                        \"value\": 10387789
                      },
                      \"grass_color\": {
                        \"value\": 9470285
                      },
                      \"mood_sound\": {
                        \"block_search_extent\": {
                          \"value\": 8
                        },
                        \"offset\": {
                          \"value\": 2
                        },
                        \"sound\": \"minecraft:ambient.cave\",
                        \"tick_delay\": {
                          \"value\": 6000
                        }
                      },
                      \"sky_color\": {
                        \"value\": 7254527
                      },
                      \"water_color\": {
                        \"value\": 4159204
                      },
                      \"water_fog_color\": {
                        \"value\": 329011
                      }
                    },
                    \"precipitation\": \"none\",
                    \"temperature\": {
                      \"value\": 2
                    }
                  },
                  \"id\": {
                    \"value\": 26
                  },
                  \"name\": \"minecraft:badlands\"
                },
                {
                  \"element\": {
                    \"downfall\": {
                      \"value\": 0
                    },
                    \"effects\": {
                      \"fog_color\": {
                        \"value\": 12638463
                      },
                      \"foliage_color\": {
                        \"value\": 10387789
                      },
                      \"grass_color\": {
                        \"value\": 9470285
                      },
                      \"mood_sound\": {
                        \"block_search_extent\": {
                          \"value\": 8
                        },
                        \"offset\": {
                          \"value\": 2
                        },
                        \"sound\": \"minecraft:ambient.cave\",
                        \"tick_delay\": {
                          \"value\": 6000
                        }
                      },
                      \"sky_color\": {
                        \"value\": 7254527
                      },
                      \"water_color\": {
                        \"value\": 4159204
                      },
                      \"water_fog_color\": {
                        \"value\": 329011
                      }
                    },
                    \"precipitation\": \"none\",
                    \"temperature\": {
                      \"value\": 2
                    }
                  },
                  \"id\": {
                    \"value\": 27
                  },
                  \"name\": \"minecraft:eroded_badlands\"
                },
                {
                  \"element\": {
                    \"downfall\": {
                      \"value\": 0
                    },
                    \"effects\": {
                      \"fog_color\": {
                        \"value\": 12638463
                      },
                      \"foliage_color\": {
                        \"value\": 10387789
                      },
                      \"grass_color\": {
                        \"value\": 9470285
                      },
                      \"mood_sound\": {
                        \"block_search_extent\": {
                          \"value\": 8
                        },
                        \"offset\": {
                          \"value\": 2
                        },
                        \"sound\": \"minecraft:ambient.cave\",
                        \"tick_delay\": {
                          \"value\": 6000
                        }
                      },
                      \"sky_color\": {
                        \"value\": 7254527
                      },
                      \"water_color\": {
                        \"value\": 4159204
                      },
                      \"water_fog_color\": {
                        \"value\": 329011
                      }
                    },
                    \"precipitation\": \"none\",
                    \"temperature\": {
                      \"value\": 2
                    }
                  },
                  \"id\": {
                    \"value\": 28
                  },
                  \"name\": \"minecraft:wooded_badlands\"
                },
                {
                  \"element\": {
                    \"downfall\": {
                      \"value\": 0.800000011920929
                    },
                    \"effects\": {
                      \"fog_color\": {
                        \"value\": 12638463
                      },
                      \"mood_sound\": {
                        \"block_search_extent\": {
                          \"value\": 8
                        },
                        \"offset\": {
                          \"value\": 2
                        },
                        \"sound\": \"minecraft:ambient.cave\",
                        \"tick_delay\": {
                          \"value\": 6000
                        }
                      },
                      \"music\": {
                        \"max_delay\": {
                          \"value\": 24000
                        },
                        \"min_delay\": {
                          \"value\": 12000
                        },
                        \"replace_current_music\": {
                          \"value\": 0
                        },
                        \"sound\": \"minecraft:music.overworld.meadow\"
                      },
                      \"sky_color\": {
                        \"value\": 8103167
                      },
                      \"water_color\": {
                        \"value\": 937679
                      },
                      \"water_fog_color\": {
                        \"value\": 329011
                      }
                    },
                    \"precipitation\": \"rain\",
                    \"temperature\": {
                      \"value\": 0.5
                    }
                  },
                  \"id\": {
                    \"value\": 29
                  },
                  \"name\": \"minecraft:meadow\"
                },
                {
                  \"element\": {
                    \"downfall\": {
                      \"value\": 0.800000011920929
                    },
                    \"effects\": {
                      \"fog_color\": {
                        \"value\": 12638463
                      },
                      \"mood_sound\": {
                        \"block_search_extent\": {
                          \"value\": 8
                        },
                        \"offset\": {
                          \"value\": 2
                        },
                        \"sound\": \"minecraft:ambient.cave\",
                        \"tick_delay\": {
                          \"value\": 6000
                        }
                      },
                      \"music\": {
                        \"max_delay\": {
                          \"value\": 24000
                        },
                        \"min_delay\": {
                          \"value\": 12000
                        },
                        \"replace_current_music\": {
                          \"value\": 0
                        },
                        \"sound\": \"minecraft:music.overworld.grove\"
                      },
                      \"sky_color\": {
                        \"value\": 8495359
                      },
                      \"water_color\": {
                        \"value\": 4159204
                      },
                      \"water_fog_color\": {
                        \"value\": 329011
                      }
                    },
                    \"precipitation\": \"snow\",
                    \"temperature\": {
                      \"value\": -0.20000000298023224
                    }
                  },
                  \"id\": {
                    \"value\": 30
                  },
                  \"name\": \"minecraft:grove\"
                },
                {
                  \"element\": {
                    \"downfall\": {
                      \"value\": 0.8999999761581421
                    },
                    \"effects\": {
                      \"fog_color\": {
                        \"value\": 12638463
                      },
                      \"mood_sound\": {
                        \"block_search_extent\": {
                          \"value\": 8
                        },
                        \"offset\": {
                          \"value\": 2
                        },
                        \"sound\": \"minecraft:ambient.cave\",
                        \"tick_delay\": {
                          \"value\": 6000
                        }
                      },
                      \"music\": {
                        \"max_delay\": {
                          \"value\": 24000
                        },
                        \"min_delay\": {
                          \"value\": 12000
                        },
                        \"replace_current_music\": {
                          \"value\": 0
                        },
                        \"sound\": \"minecraft:music.overworld.snowy_slopes\"
                      },
                      \"sky_color\": {
                        \"value\": 8560639
                      },
                      \"water_color\": {
                        \"value\": 4159204
                      },
                      \"water_fog_color\": {
                        \"value\": 329011
                      }
                    },
                    \"precipitation\": \"snow\",
                    \"temperature\": {
                      \"value\": -0.30000001192092896
                    }
                  },
                  \"id\": {
                    \"value\": 31
                  },
                  \"name\": \"minecraft:snowy_slopes\"
                },
                {
                  \"element\": {
                    \"downfall\": {
                      \"value\": 0.8999999761581421
                    },
                    \"effects\": {
                      \"fog_color\": {
                        \"value\": 12638463
                      },
                      \"mood_sound\": {
                        \"block_search_extent\": {
                          \"value\": 8
                        },
                        \"offset\": {
                          \"value\": 2
                        },
                        \"sound\": \"minecraft:ambient.cave\",
                        \"tick_delay\": {
                          \"value\": 6000
                        }
                      },
                      \"music\": {
                        \"max_delay\": {
                          \"value\": 24000
                        },
                        \"min_delay\": {
                          \"value\": 12000
                        },
                        \"replace_current_music\": {
                          \"value\": 0
                        },
                        \"sound\": \"minecraft:music.overworld.frozen_peaks\"
                      },
                      \"sky_color\": {
                        \"value\": 8756735
                      },
                      \"water_color\": {
                        \"value\": 4159204
                      },
                      \"water_fog_color\": {
                        \"value\": 329011
                      }
                    },
                    \"precipitation\": \"snow\",
                    \"temperature\": {
                      \"value\": -0.699999988079071
                    }
                  },
                  \"id\": {
                    \"value\": 32
                  },
                  \"name\": \"minecraft:frozen_peaks\"
                },
                {
                  \"element\": {
                    \"downfall\": {
                      \"value\": 0.8999999761581421
                    },
                    \"effects\": {
                      \"fog_color\": {
                        \"value\": 12638463
                      },
                      \"mood_sound\": {
                        \"block_search_extent\": {
                          \"value\": 8
                        },
                        \"offset\": {
                          \"value\": 2
                        },
                        \"sound\": \"minecraft:ambient.cave\",
                        \"tick_delay\": {
                          \"value\": 6000
                        }
                      },
                      \"music\": {
                        \"max_delay\": {
                          \"value\": 24000
                        },
                        \"min_delay\": {
                          \"value\": 12000
                        },
                        \"replace_current_music\": {
                          \"value\": 0
                        },
                        \"sound\": \"minecraft:music.overworld.jagged_peaks\"
                      },
                      \"sky_color\": {
                        \"value\": 8756735
                      },
                      \"water_color\": {
                        \"value\": 4159204
                      },
                      \"water_fog_color\": {
                        \"value\": 329011
                      }
                    },
                    \"precipitation\": \"snow\",
                    \"temperature\": {
                      \"value\": -0.699999988079071
                    }
                  },
                  \"id\": {
                    \"value\": 33
                  },
                  \"name\": \"minecraft:jagged_peaks\"
                },
                {
                  \"element\": {
                    \"downfall\": {
                      \"value\": 0.30000001192092896
                    },
                    \"effects\": {
                      \"fog_color\": {
                        \"value\": 12638463
                      },
                      \"mood_sound\": {
                        \"block_search_extent\": {
                          \"value\": 8
                        },
                        \"offset\": {
                          \"value\": 2
                        },
                        \"sound\": \"minecraft:ambient.cave\",
                        \"tick_delay\": {
                          \"value\": 6000
                        }
                      },
                      \"music\": {
                        \"max_delay\": {
                          \"value\": 24000
                        },
                        \"min_delay\": {
                          \"value\": 12000
                        },
                        \"replace_current_music\": {
                          \"value\": 0
                        },
                        \"sound\": \"minecraft:music.overworld.stony_peaks\"
                      },
                      \"sky_color\": {
                        \"value\": 7776511
                      },
                      \"water_color\": {
                        \"value\": 4159204
                      },
                      \"water_fog_color\": {
                        \"value\": 329011
                      }
                    },
                    \"precipitation\": \"rain\",
                    \"temperature\": {
                      \"value\": 1
                    }
                  },
                  \"id\": {
                    \"value\": 34
                  },
                  \"name\": \"minecraft:stony_peaks\"
                },
                {
                  \"element\": {
                    \"downfall\": {
                      \"value\": 0.5
                    },
                    \"effects\": {
                      \"fog_color\": {
                        \"value\": 12638463
                      },
                      \"mood_sound\": {
                        \"block_search_extent\": {
                          \"value\": 8
                        },
                        \"offset\": {
                          \"value\": 2
                        },
                        \"sound\": \"minecraft:ambient.cave\",
                        \"tick_delay\": {
                          \"value\": 6000
                        }
                      },
                      \"sky_color\": {
                        \"value\": 8103167
                      },
                      \"water_color\": {
                        \"value\": 4159204
                      },
                      \"water_fog_color\": {
                        \"value\": 329011
                      }
                    },
                    \"precipitation\": \"rain\",
                    \"temperature\": {
                      \"value\": 0.5
                    }
                  },
                  \"id\": {
                    \"value\": 35
                  },
                  \"name\": \"minecraft:river\"
                },
                {
                  \"element\": {
                    \"downfall\": {
                      \"value\": 0.5
                    },
                    \"effects\": {
                      \"fog_color\": {
                        \"value\": 12638463
                      },
                      \"mood_sound\": {
                        \"block_search_extent\": {
                          \"value\": 8
                        },
                        \"offset\": {
                          \"value\": 2
                        },
                        \"sound\": \"minecraft:ambient.cave\",
                        \"tick_delay\": {
                          \"value\": 6000
                        }
                      },
                      \"sky_color\": {
                        \"value\": 8364543
                      },
                      \"water_color\": {
                        \"value\": 3750089
                      },
                      \"water_fog_color\": {
                        \"value\": 329011
                      }
                    },
                    \"precipitation\": \"snow\",
                    \"temperature\": {
                      \"value\": 0
                    }
                  },
                  \"id\": {
                    \"value\": 36
                  },
                  \"name\": \"minecraft:frozen_river\"
                },
                {
                  \"element\": {
                    \"downfall\": {
                      \"value\": 0.4000000059604645
                    },
                    \"effects\": {
                      \"fog_color\": {
                        \"value\": 12638463
                      },
                      \"mood_sound\": {
                        \"block_search_extent\": {
                          \"value\": 8
                        },
                        \"offset\": {
                          \"value\": 2
                        },
                        \"sound\": \"minecraft:ambient.cave\",
                        \"tick_delay\": {
                          \"value\": 6000
                        }
                      },
                      \"sky_color\": {
                        \"value\": 7907327
                      },
                      \"water_color\": {
                        \"value\": 4159204
                      },
                      \"water_fog_color\": {
                        \"value\": 329011
                      }
                    },
                    \"precipitation\": \"rain\",
                    \"temperature\": {
                      \"value\": 0.800000011920929
                    }
                  },
                  \"id\": {
                    \"value\": 37
                  },
                  \"name\": \"minecraft:beach\"
                },
                {
                  \"element\": {
                    \"downfall\": {
                      \"value\": 0.30000001192092896
                    },
                    \"effects\": {
                      \"fog_color\": {
                        \"value\": 12638463
                      },
                      \"mood_sound\": {
                        \"block_search_extent\": {
                          \"value\": 8
                        },
                        \"offset\": {
                          \"value\": 2
                        },
                        \"sound\": \"minecraft:ambient.cave\",
                        \"tick_delay\": {
                          \"value\": 6000
                        }
                      },
                      \"sky_color\": {
                        \"value\": 8364543
                      },
                      \"water_color\": {
                        \"value\": 4020182
                      },
                      \"water_fog_color\": {
                        \"value\": 329011
                      }
                    },
                    \"precipitation\": \"snow\",
                    \"temperature\": {
                      \"value\": 0.05000000074505806
                    }
                  },
                  \"id\": {
                    \"value\": 38
                  },
                  \"name\": \"minecraft:snowy_beach\"
                },
                {
                  \"element\": {
                    \"downfall\": {
                      \"value\": 0.30000001192092896
                    },
                    \"effects\": {
                      \"fog_color\": {
                        \"value\": 12638463
                      },
                      \"mood_sound\": {
                        \"block_search_extent\": {
                          \"value\": 8
                        },
                        \"offset\": {
                          \"value\": 2
                        },
                        \"sound\": \"minecraft:ambient.cave\",
                        \"tick_delay\": {
                          \"value\": 6000
                        }
                      },
                      \"sky_color\": {
                        \"value\": 8233727
                      },
                      \"water_color\": {
                        \"value\": 4159204
                      },
                      \"water_fog_color\": {
                        \"value\": 329011
                      }
                    },
                    \"precipitation\": \"rain\",
                    \"temperature\": {
                      \"value\": 0.20000000298023224
                    }
                  },
                  \"id\": {
                    \"value\": 39
                  },
                  \"name\": \"minecraft:stony_shore\"
                },
                {
                  \"element\": {
                    \"downfall\": {
                      \"value\": 0.5
                    },
                    \"effects\": {
                      \"fog_color\": {
                        \"value\": 12638463
                      },
                      \"mood_sound\": {
                        \"block_search_extent\": {
                          \"value\": 8
                        },
                        \"offset\": {
                          \"value\": 2
                        },
                        \"sound\": \"minecraft:ambient.cave\",
                        \"tick_delay\": {
                          \"value\": 6000
                        }
                      },
                      \"sky_color\": {
                        \"value\": 8103167
                      },
                      \"water_color\": {
                        \"value\": 4445678
                      },
                      \"water_fog_color\": {
                        \"value\": 270131
                      }
                    },
                    \"precipitation\": \"rain\",
                    \"temperature\": {
                      \"value\": 0.5
                    }
                  },
                  \"id\": {
                    \"value\": 40
                  },
                  \"name\": \"minecraft:warm_ocean\"
                },
                {
                  \"element\": {
                    \"downfall\": {
                      \"value\": 0.5
                    },
                    \"effects\": {
                      \"fog_color\": {
                        \"value\": 12638463
                      },
                      \"mood_sound\": {
                        \"block_search_extent\": {
                          \"value\": 8
                        },
                        \"offset\": {
                          \"value\": 2
                        },
                        \"sound\": \"minecraft:ambient.cave\",
                        \"tick_delay\": {
                          \"value\": 6000
                        }
                      },
                      \"sky_color\": {
                        \"value\": 8103167
                      },
                      \"water_color\": {
                        \"value\": 4566514
                      },
                      \"water_fog_color\": {
                        \"value\": 267827
                      }
                    },
                    \"precipitation\": \"rain\",
                    \"temperature\": {
                      \"value\": 0.5
                    }
                  },
                  \"id\": {
                    \"value\": 41
                  },
                  \"name\": \"minecraft:lukewarm_ocean\"
                },
                {
                  \"element\": {
                    \"downfall\": {
                      \"value\": 0.5
                    },
                    \"effects\": {
                      \"fog_color\": {
                        \"value\": 12638463
                      },
                      \"mood_sound\": {
                        \"block_search_extent\": {
                          \"value\": 8
                        },
                        \"offset\": {
                          \"value\": 2
                        },
                        \"sound\": \"minecraft:ambient.cave\",
                        \"tick_delay\": {
                          \"value\": 6000
                        }
                      },
                      \"sky_color\": {
                        \"value\": 8103167
                      },
                      \"water_color\": {
                        \"value\": 4566514
                      },
                      \"water_fog_color\": {
                        \"value\": 267827
                      }
                    },
                    \"precipitation\": \"rain\",
                    \"temperature\": {
                      \"value\": 0.5
                    }
                  },
                  \"id\": {
                    \"value\": 42
                  },
                  \"name\": \"minecraft:deep_lukewarm_ocean\"
                },
                {
                  \"element\": {
                    \"downfall\": {
                      \"value\": 0.5
                    },
                    \"effects\": {
                      \"fog_color\": {
                        \"value\": 12638463
                      },
                      \"mood_sound\": {
                        \"block_search_extent\": {
                          \"value\": 8
                        },
                        \"offset\": {
                          \"value\": 2
                        },
                        \"sound\": \"minecraft:ambient.cave\",
                        \"tick_delay\": {
                          \"value\": 6000
                        }
                      },
                      \"sky_color\": {
                        \"value\": 8103167
                      },
                      \"water_color\": {
                        \"value\": 4159204
                      },
                      \"water_fog_color\": {
                        \"value\": 329011
                      }
                    },
                    \"precipitation\": \"rain\",
                    \"temperature\": {
                      \"value\": 0.5
                    }
                  },
                  \"id\": {
                    \"value\": 43
                  },
                  \"name\": \"minecraft:ocean\"
                },
                {
                  \"element\": {
                    \"downfall\": {
                      \"value\": 0.5
                    },
                    \"effects\": {
                      \"fog_color\": {
                        \"value\": 12638463
                      },
                      \"mood_sound\": {
                        \"block_search_extent\": {
                          \"value\": 8
                        },
                        \"offset\": {
                          \"value\": 2
                        },
                        \"sound\": \"minecraft:ambient.cave\",
                        \"tick_delay\": {
                          \"value\": 6000
                        }
                      },
                      \"sky_color\": {
                        \"value\": 8103167
                      },
                      \"water_color\": {
                        \"value\": 4159204
                      },
                      \"water_fog_color\": {
                        \"value\": 329011
                      }
                    },
                    \"precipitation\": \"rain\",
                    \"temperature\": {
                      \"value\": 0.5
                    }
                  },
                  \"id\": {
                    \"value\": 44
                  },
                  \"name\": \"minecraft:deep_ocean\"
                },
                {
                  \"element\": {
                    \"downfall\": {
                      \"value\": 0.5
                    },
                    \"effects\": {
                      \"fog_color\": {
                        \"value\": 12638463
                      },
                      \"mood_sound\": {
                        \"block_search_extent\": {
                          \"value\": 8
                        },
                        \"offset\": {
                          \"value\": 2
                        },
                        \"sound\": \"minecraft:ambient.cave\",
                        \"tick_delay\": {
                          \"value\": 6000
                        }
                      },
                      \"sky_color\": {
                        \"value\": 8103167
                      },
                      \"water_color\": {
                        \"value\": 4020182
                      },
                      \"water_fog_color\": {
                        \"value\": 329011
                      }
                    },
                    \"precipitation\": \"rain\",
                    \"temperature\": {
                      \"value\": 0.5
                    }
                  },
                  \"id\": {
                    \"value\": 45
                  },
                  \"name\": \"minecraft:cold_ocean\"
                },
                {
                  \"element\": {
                    \"downfall\": {
                      \"value\": 0.5
                    },
                    \"effects\": {
                      \"fog_color\": {
                        \"value\": 12638463
                      },
                      \"mood_sound\": {
                        \"block_search_extent\": {
                          \"value\": 8
                        },
                        \"offset\": {
                          \"value\": 2
                        },
                        \"sound\": \"minecraft:ambient.cave\",
                        \"tick_delay\": {
                          \"value\": 6000
                        }
                      },
                      \"sky_color\": {
                        \"value\": 8103167
                      },
                      \"water_color\": {
                        \"value\": 4020182
                      },
                      \"water_fog_color\": {
                        \"value\": 329011
                      }
                    },
                    \"precipitation\": \"rain\",
                    \"temperature\": {
                      \"value\": 0.5
                    }
                  },
                  \"id\": {
                    \"value\": 46
                  },
                  \"name\": \"minecraft:deep_cold_ocean\"
                },
                {
                  \"element\": {
                    \"downfall\": {
                      \"value\": 0.5
                    },
                    \"effects\": {
                      \"fog_color\": {
                        \"value\": 12638463
                      },
                      \"mood_sound\": {
                        \"block_search_extent\": {
                          \"value\": 8
                        },
                        \"offset\": {
                          \"value\": 2
                        },
                        \"sound\": \"minecraft:ambient.cave\",
                        \"tick_delay\": {
                          \"value\": 6000
                        }
                      },
                      \"sky_color\": {
                        \"value\": 8364543
                      },
                      \"water_color\": {
                        \"value\": 3750089
                      },
                      \"water_fog_color\": {
                        \"value\": 329011
                      }
                    },
                    \"precipitation\": \"snow\",
                    \"temperature\": {
                      \"value\": 0
                    },
                    \"temperature_modifier\": \"frozen\"
                  },
                  \"id\": {
                    \"value\": 47
                  },
                  \"name\": \"minecraft:frozen_ocean\"
                },
                {
                  \"element\": {
                    \"downfall\": {
                      \"value\": 0.5
                    },
                    \"effects\": {
                      \"fog_color\": {
                        \"value\": 12638463
                      },
                      \"mood_sound\": {
                        \"block_search_extent\": {
                          \"value\": 8
                        },
                        \"offset\": {
                          \"value\": 2
                        },
                        \"sound\": \"minecraft:ambient.cave\",
                        \"tick_delay\": {
                          \"value\": 6000
                        }
                      },
                      \"sky_color\": {
                        \"value\": 8103167
                      },
                      \"water_color\": {
                        \"value\": 3750089
                      },
                      \"water_fog_color\": {
                        \"value\": 329011
                      }
                    },
                    \"precipitation\": \"rain\",
                    \"temperature\": {
                      \"value\": 0.5
                    },
                    \"temperature_modifier\": \"frozen\"
                  },
                  \"id\": {
                    \"value\": 48
                  },
                  \"name\": \"minecraft:deep_frozen_ocean\"
                },
                {
                  \"element\": {
                    \"downfall\": {
                      \"value\": 1
                    },
                    \"effects\": {
                      \"fog_color\": {
                        \"value\": 12638463
                      },
                      \"mood_sound\": {
                        \"block_search_extent\": {
                          \"value\": 8
                        },
                        \"offset\": {
                          \"value\": 2
                        },
                        \"sound\": \"minecraft:ambient.cave\",
                        \"tick_delay\": {
                          \"value\": 6000
                        }
                      },
                      \"sky_color\": {
                        \"value\": 7842047
                      },
                      \"water_color\": {
                        \"value\": 4159204
                      },
                      \"water_fog_color\": {
                        \"value\": 329011
                      }
                    },
                    \"precipitation\": \"rain\",
                    \"temperature\": {
                      \"value\": 0.8999999761581421
                    }
                  },
                  \"id\": {
                    \"value\": 49
                  },
                  \"name\": \"minecraft:mushroom_fields\"
                },
                {
                  \"element\": {
                    \"downfall\": {
                      \"value\": 0.4000000059604645
                    },
                    \"effects\": {
                      \"fog_color\": {
                        \"value\": 12638463
                      },
                      \"mood_sound\": {
                        \"block_search_extent\": {
                          \"value\": 8
                        },
                        \"offset\": {
                          \"value\": 2
                        },
                        \"sound\": \"minecraft:ambient.cave\",
                        \"tick_delay\": {
                          \"value\": 6000
                        }
                      },
                      \"music\": {
                        \"max_delay\": {
                          \"value\": 24000
                        },
                        \"min_delay\": {
                          \"value\": 12000
                        },
                        \"replace_current_music\": {
                          \"value\": 0
                        },
                        \"sound\": \"minecraft:music.overworld.dripstone_caves\"
                      },
                      \"sky_color\": {
                        \"value\": 7907327
                      },
                      \"water_color\": {
                        \"value\": 4159204
                      },
                      \"water_fog_color\": {
                        \"value\": 329011
                      }
                    },
                    \"precipitation\": \"rain\",
                    \"temperature\": {
                      \"value\": 0.800000011920929
                    }
                  },
                  \"id\": {
                    \"value\": 50
                  },
                  \"name\": \"minecraft:dripstone_caves\"
                },
                {
                  \"element\": {
                    \"downfall\": {
                      \"value\": 0.5
                    },
                    \"effects\": {
                      \"fog_color\": {
                        \"value\": 12638463
                      },
                      \"mood_sound\": {
                        \"block_search_extent\": {
                          \"value\": 8
                        },
                        \"offset\": {
                          \"value\": 2
                        },
                        \"sound\": \"minecraft:ambient.cave\",
                        \"tick_delay\": {
                          \"value\": 6000
                        }
                      },
                      \"music\": {
                        \"max_delay\": {
                          \"value\": 24000
                        },
                        \"min_delay\": {
                          \"value\": 12000
                        },
                        \"replace_current_music\": {
                          \"value\": 0
                        },
                        \"sound\": \"minecraft:music.overworld.lush_caves\"
                      },
                      \"sky_color\": {
                        \"value\": 8103167
                      },
                      \"water_color\": {
                        \"value\": 4159204
                      },
                      \"water_fog_color\": {
                        \"value\": 329011
                      }
                    },
                    \"precipitation\": \"rain\",
                    \"temperature\": {
                      \"value\": 0.5
                    }
                  },
                  \"id\": {
                    \"value\": 51
                  },
                  \"name\": \"minecraft:lush_caves\"
                },
                {
                  \"element\": {
                    \"downfall\": {
                      \"value\": 0.4000000059604645
                    },
                    \"effects\": {
                      \"fog_color\": {
                        \"value\": 12638463
                      },
                      \"mood_sound\": {
                        \"block_search_extent\": {
                          \"value\": 8
                        },
                        \"offset\": {
                          \"value\": 2
                        },
                        \"sound\": \"minecraft:ambient.cave\",
                        \"tick_delay\": {
                          \"value\": 6000
                        }
                      },
                      \"music\": {
                        \"max_delay\": {
                          \"value\": 24000
                        },
                        \"min_delay\": {
                          \"value\": 12000
                        },
                        \"replace_current_music\": {
                          \"value\": 0
                        },
                        \"sound\": \"minecraft:music.overworld.deep_dark\"
                      },
                      \"sky_color\": {
                        \"value\": 7907327
                      },
                      \"water_color\": {
                        \"value\": 4159204
                      },
                      \"water_fog_color\": {
                        \"value\": 329011
                      }
                    },
                    \"precipitation\": \"rain\",
                    \"temperature\": {
                      \"value\": 0.800000011920929
                    }
                  },
                  \"id\": {
                    \"value\": 52
                  },
                  \"name\": \"minecraft:deep_dark\"
                },
                {
                  \"element\": {
                    \"downfall\": {
                      \"value\": 0
                    },
                    \"effects\": {
                      \"additions_sound\": {
                        \"sound\": \"minecraft:ambient.nether_wastes.additions\",
                        \"tick_chance\": {
                          \"value\": 0.0111
                        }
                      },
                      \"ambient_sound\": \"minecraft:ambient.nether_wastes.loop\",
                      \"fog_color\": {
                        \"value\": 3344392
                      },
                      \"mood_sound\": {
                        \"block_search_extent\": {
                          \"value\": 8
                        },
                        \"offset\": {
                          \"value\": 2
                        },
                        \"sound\": \"minecraft:ambient.nether_wastes.mood\",
                        \"tick_delay\": {
                          \"value\": 6000
                        }
                      },
                      \"music\": {
                        \"max_delay\": {
                          \"value\": 24000
                        },
                        \"min_delay\": {
                          \"value\": 12000
                        },
                        \"replace_current_music\": {
                          \"value\": 0
                        },
                        \"sound\": \"minecraft:music.nether.nether_wastes\"
                      },
                      \"sky_color\": {
                        \"value\": 7254527
                      },
                      \"water_color\": {
                        \"value\": 4159204
                      },
                      \"water_fog_color\": {
                        \"value\": 329011
                      }
                    },
                    \"precipitation\": \"none\",
                    \"temperature\": {
                      \"value\": 2
                    }
                  },
                  \"id\": {
                    \"value\": 53
                  },
                  \"name\": \"minecraft:nether_wastes\"
                },
                {
                  \"element\": {
                    \"downfall\": {
                      \"value\": 0
                    },
                    \"effects\": {
                      \"additions_sound\": {
                        \"sound\": \"minecraft:ambient.warped_forest.additions\",
                        \"tick_chance\": {
                          \"value\": 0.0111
                        }
                      },
                      \"ambient_sound\": \"minecraft:ambient.warped_forest.loop\",
                      \"fog_color\": {
                        \"value\": 1705242
                      },
                      \"mood_sound\": {
                        \"block_search_extent\": {
                          \"value\": 8
                        },
                        \"offset\": {
                          \"value\": 2
                        },
                        \"sound\": \"minecraft:ambient.warped_forest.mood\",
                        \"tick_delay\": {
                          \"value\": 6000
                        }
                      },
                      \"music\": {
                        \"max_delay\": {
                          \"value\": 24000
                        },
                        \"min_delay\": {
                          \"value\": 12000
                        },
                        \"replace_current_music\": {
                          \"value\": 0
                        },
                        \"sound\": \"minecraft:music.nether.warped_forest\"
                      },
                      \"particle\": {
                        \"options\": {
                          \"type\": \"minecraft:warped_spore\"
                        },
                        \"probability\": {
                          \"value\": 0.014279999770224094
                        }
                      },
                      \"sky_color\": {
                        \"value\": 7254527
                      },
                      \"water_color\": {
                        \"value\": 4159204
                      },
                      \"water_fog_color\": {
                        \"value\": 329011
                      }
                    },
                    \"precipitation\": \"none\",
                    \"temperature\": {
                      \"value\": 2
                    }
                  },
                  \"id\": {
                    \"value\": 54
                  },
                  \"name\": \"minecraft:warped_forest\"
                },
                {
                  \"element\": {
                    \"downfall\": {
                      \"value\": 0
                    },
                    \"effects\": {
                      \"additions_sound\": {
                        \"sound\": \"minecraft:ambient.crimson_forest.additions\",
                        \"tick_chance\": {
                          \"value\": 0.0111
                        }
                      },
                      \"ambient_sound\": \"minecraft:ambient.crimson_forest.loop\",
                      \"fog_color\": {
                        \"value\": 3343107
                      },
                      \"mood_sound\": {
                        \"block_search_extent\": {
                          \"value\": 8
                        },
                        \"offset\": {
                          \"value\": 2
                        },
                        \"sound\": \"minecraft:ambient.crimson_forest.mood\",
                        \"tick_delay\": {
                          \"value\": 6000
                        }
                      },
                      \"music\": {
                        \"max_delay\": {
                          \"value\": 24000
                        },
                        \"min_delay\": {
                          \"value\": 12000
                        },
                        \"replace_current_music\": {
                          \"value\": 0
                        },
                        \"sound\": \"minecraft:music.nether.crimson_forest\"
                      },
                      \"particle\": {
                        \"options\": {
                          \"type\": \"minecraft:crimson_spore\"
                        },
                        \"probability\": {
                          \"value\": 0.02500000037252903
                        }
                      },
                      \"sky_color\": {
                        \"value\": 7254527
                      },
                      \"water_color\": {
                        \"value\": 4159204
                      },
                      \"water_fog_color\": {
                        \"value\": 329011
                      }
                    },
                    \"precipitation\": \"none\",
                    \"temperature\": {
                      \"value\": 2
                    }
                  },
                  \"id\": {
                    \"value\": 55
                  },
                  \"name\": \"minecraft:crimson_forest\"
                },
                {
                  \"element\": {
                    \"downfall\": {
                      \"value\": 0
                    },
                    \"effects\": {
                      \"additions_sound\": {
                        \"sound\": \"minecraft:ambient.soul_sand_valley.additions\",
                        \"tick_chance\": {
                          \"value\": 0.0111
                        }
                      },
                      \"ambient_sound\": \"minecraft:ambient.soul_sand_valley.loop\",
                      \"fog_color\": {
                        \"value\": 1787717
                      },
                      \"mood_sound\": {
                        \"block_search_extent\": {
                          \"value\": 8
                        },
                        \"offset\": {
                          \"value\": 2
                        },
                        \"sound\": \"minecraft:ambient.soul_sand_valley.mood\",
                        \"tick_delay\": {
                          \"value\": 6000
                        }
                      },
                      \"music\": {
                        \"max_delay\": {
                          \"value\": 24000
                        },
                        \"min_delay\": {
                          \"value\": 12000
                        },
                        \"replace_current_music\": {
                          \"value\": 0
                        },
                        \"sound\": \"minecraft:music.nether.soul_sand_valley\"
                      },
                      \"particle\": {
                        \"options\": {
                          \"type\": \"minecraft:ash\"
                        },
                        \"probability\": {
                          \"value\": 0.0062500000931322575
                        }
                      },
                      \"sky_color\": {
                        \"value\": 7254527
                      },
                      \"water_color\": {
                        \"value\": 4159204
                      },
                      \"water_fog_color\": {
                        \"value\": 329011
                      }
                    },
                    \"precipitation\": \"none\",
                    \"temperature\": {
                      \"value\": 2
                    }
                  },
                  \"id\": {
                    \"value\": 56
                  },
                  \"name\": \"minecraft:soul_sand_valley\"
                },
                {
                  \"element\": {
                    \"downfall\": {
                      \"value\": 0
                    },
                    \"effects\": {
                      \"additions_sound\": {
                        \"sound\": \"minecraft:ambient.basalt_deltas.additions\",
                        \"tick_chance\": {
                          \"value\": 0.0111
                        }
                      },
                      \"ambient_sound\": \"minecraft:ambient.basalt_deltas.loop\",
                      \"fog_color\": {
                        \"value\": 6840176
                      },
                      \"mood_sound\": {
                        \"block_search_extent\": {
                          \"value\": 8
                        },
                        \"offset\": {
                          \"value\": 2
                        },
                        \"sound\": \"minecraft:ambient.basalt_deltas.mood\",
                        \"tick_delay\": {
                          \"value\": 6000
                        }
                      },
                      \"music\": {
                        \"max_delay\": {
                          \"value\": 24000
                        },
                        \"min_delay\": {
                          \"value\": 12000
                        },
                        \"replace_current_music\": {
                          \"value\": 0
                        },
                        \"sound\": \"minecraft:music.nether.basalt_deltas\"
                      },
                      \"particle\": {
                        \"options\": {
                          \"type\": \"minecraft:white_ash\"
                        },
                        \"probability\": {
                          \"value\": 0.1180933341383934
                        }
                      },
                      \"sky_color\": {
                        \"value\": 7254527
                      },
                      \"water_color\": {
                        \"value\": 4159204
                      },
                      \"water_fog_color\": {
                        \"value\": 329011
                      }
                    },
                    \"precipitation\": \"none\",
                    \"temperature\": {
                      \"value\": 2
                    }
                  },
                  \"id\": {
                    \"value\": 57
                  },
                  \"name\": \"minecraft:basalt_deltas\"
                },
                {
                  \"element\": {
                    \"downfall\": {
                      \"value\": 0.5
                    },
                    \"effects\": {
                      \"fog_color\": {
                        \"value\": 10518688
                      },
                      \"mood_sound\": {
                        \"block_search_extent\": {
                          \"value\": 8
                        },
                        \"offset\": {
                          \"value\": 2
                        },
                        \"sound\": \"minecraft:ambient.cave\",
                        \"tick_delay\": {
                          \"value\": 6000
                        }
                      },
                      \"sky_color\": {
                        \"value\": 0
                      },
                      \"water_color\": {
                        \"value\": 4159204
                      },
                      \"water_fog_color\": {
                        \"value\": 329011
                      }
                    },
                    \"precipitation\": \"none\",
                    \"temperature\": {
                      \"value\": 0.5
                    }
                  },
                  \"id\": {
                    \"value\": 58
                  },
                  \"name\": \"minecraft:the_end\"
                },
                {
                  \"element\": {
                    \"downfall\": {
                      \"value\": 0.5
                    },
                    \"effects\": {
                      \"fog_color\": {
                        \"value\": 10518688
                      },
                      \"mood_sound\": {
                        \"block_search_extent\": {
                          \"value\": 8
                        },
                        \"offset\": {
                          \"value\": 2
                        },
                        \"sound\": \"minecraft:ambient.cave\",
                        \"tick_delay\": {
                          \"value\": 6000
                        }
                      },
                      \"sky_color\": {
                        \"value\": 0
                      },
                      \"water_color\": {
                        \"value\": 4159204
                      },
                      \"water_fog_color\": {
                        \"value\": 329011
                      }
                    },
                    \"precipitation\": \"none\",
                    \"temperature\": {
                      \"value\": 0.5
                    }
                  },
                  \"id\": {
                    \"value\": 59
                  },
                  \"name\": \"minecraft:end_highlands\"
                },
                {
                  \"element\": {
                    \"downfall\": {
                      \"value\": 0.5
                    },
                    \"effects\": {
                      \"fog_color\": {
                        \"value\": 10518688
                      },
                      \"mood_sound\": {
                        \"block_search_extent\": {
                          \"value\": 8
                        },
                        \"offset\": {
                          \"value\": 2
                        },
                        \"sound\": \"minecraft:ambient.cave\",
                        \"tick_delay\": {
                          \"value\": 6000
                        }
                      },
                      \"sky_color\": {
                        \"value\": 0
                      },
                      \"water_color\": {
                        \"value\": 4159204
                      },
                      \"water_fog_color\": {
                        \"value\": 329011
                      }
                    },
                    \"precipitation\": \"none\",
                    \"temperature\": {
                      \"value\": 0.5
                    }
                  },
                  \"id\": {
                    \"value\": 60
                  },
                  \"name\": \"minecraft:end_midlands\"
                },
                {
                  \"element\": {
                    \"downfall\": {
                      \"value\": 0.5
                    },
                    \"effects\": {
                      \"fog_color\": {
                        \"value\": 10518688
                      },
                      \"mood_sound\": {
                        \"block_search_extent\": {
                          \"value\": 8
                        },
                        \"offset\": {
                          \"value\": 2
                        },
                        \"sound\": \"minecraft:ambient.cave\",
                        \"tick_delay\": {
                          \"value\": 6000
                        }
                      },
                      \"sky_color\": {
                        \"value\": 0
                      },
                      \"water_color\": {
                        \"value\": 4159204
                      },
                      \"water_fog_color\": {
                        \"value\": 329011
                      }
                    },
                    \"precipitation\": \"none\",
                    \"temperature\": {
                      \"value\": 0.5
                    }
                  },
                  \"id\": {
                    \"value\": 61
                  },
                  \"name\": \"minecraft:small_end_islands\"
                },
                {
                  \"element\": {
                    \"downfall\": {
                      \"value\": 0.5
                    },
                    \"effects\": {
                      \"fog_color\": {
                        \"value\": 10518688
                      },
                      \"mood_sound\": {
                        \"block_search_extent\": {
                          \"value\": 8
                        },
                        \"offset\": {
                          \"value\": 2
                        },
                        \"sound\": \"minecraft:ambient.cave\",
                        \"tick_delay\": {
                          \"value\": 6000
                        }
                      },
                      \"sky_color\": {
                        \"value\": 0
                      },
                      \"water_color\": {
                        \"value\": 4159204
                      },
                      \"water_fog_color\": {
                        \"value\": 329011
                      }
                    },
                    \"precipitation\": \"none\",
                    \"temperature\": {
                      \"value\": 0.5
                    }
                  },
                  \"id\": {
                    \"value\": 62
                  },
                  \"name\": \"minecraft:end_barrens\"
                }
              ]
            }
          }
        ",
        )
        .unwrap();

        let mut data = (false as i8).to_byte(); // Is hardcore

        data.append(&mut 1i8.to_byte()); // Gamemode
        data.append(&mut (-1i8).to_byte()); // Previous Gamemode
        data.append(&mut 1.to_varint()); // Dimension Count
        data.append(&mut "minecraft:overworld".to_packet_string()); // Dimension Names

        data.append(&mut serialize(&registry_codec, None, Flavor::Uncompressed).unwrap()); // Registry Codec

        data.append(&mut "minecraft:overworld".to_packet_string()); // Dimension Type
        data.append(&mut "minecraft:overworld".to_packet_string()); // Dimension Name
        data.append(&mut (0i64).to_long()); // Hashed seed
        data.append(&mut 0.to_varint()); // Max Players
        data.append(&mut 10.to_varint()); // View Distance
        data.append(&mut 10.to_varint()); // Simulation Distance
        data.append(&mut (false as i8).to_byte()); // Reduced Debug Info
        data.append(&mut (true as i8).to_byte()); // Enable respawn screen
        data.append(&mut (false as i8).to_byte()); // Is Debug
        data.append(&mut (false as i8).to_byte()); // Is Flat
        data.append(&mut (false as i8).to_byte()); // Has death location

        Packet {
            id: ClientPlayPacketId::Login as i32,
            data,
        }
    }
}
