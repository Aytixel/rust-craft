use num_derive::FromPrimitive;

pub mod handshake;
pub mod login;
pub mod play;
pub mod status;

#[derive(FromPrimitive)]
pub enum ServerHandshakePacketId {
    Handshake,
}

#[derive(FromPrimitive)]
pub enum ServerStatusPacketId {
    Status,
    Ping,
}

pub enum ClientStatusPacketId {
    Status,
}

#[derive(FromPrimitive)]
pub enum ServerLoginPacketId {
    LoginStart,
    Encryption,
    LoginPlugin,
}

pub enum ClientLoginPacketId {
    Disconnect,
    Encryption,
    LoginSuccess,
    SetCompression,
    LoginPlugin,
}

#[derive(FromPrimitive)]
pub enum ServerPlayPacketId {
    ConfirmTeleportation,
    QueryBlockEntityTag,
    ChangeDifficulty,
    MessageAcknowledgment,
    ChatCommand,
    ChatMessage,
    ClientCommand,
    ClientInformation,
    CommandSuggestions,
    ClickContainerButton,
    ClickContainer,
    CloseContainer,
    PluginMessage,
    EditBook,
    QueryEntityTag,
    Interact,
    JigsawGenerate,
    KeepAlive,
    LockDifficulty,
    SetPlayerPosition,
    SetPlayerPositionAndRotation,
    SetPlayerRotation,
    SetPlayerOnGround,
    MoveVehicule,
    PaddleBoat,
    PickItem,
    PlaceRecipe,
    PlayerAbilities,
    PlayerAction,
    PlayerCommand,
    PlayerInput,
    Pong,
    PlayerSession,
    ChangeRecipeBookSettings,
    SetSeenRecipe,
    RenameItem,
    ResourcePack,
    SeenAdvancements,
    SelectTrade,
    SetBeaconEffect,
    SetHeldItem,
    ProgramCommandBlock,
    ProgramCommandBlockMinecraft,
    SetCreativeModeSlot,
    ProgramJigsawBlock,
    ProgramStructureBlock,
    UpdateSign,
    SwingArm,
    TeleportToEntity,
    UseItemOn,
    UseItem,
}

pub enum ClientPlayPacketId {
    SpawnEntity,
    SpawnExperienceOrb,
    SpawnPlayer,
    EntityAnimation,
    AwardStatistics,
    AcknowledgeBlockChange,
    SetBlockDestroyStage,
    BlockEntityData,
    BlockAction,
    BlockUpdate,
    BossBar,
    ChangeDifficulty,
    ClearTitles,
    CommandSuggestions,
    Commands,
    CloseContainer,
    SetContainerContent,
    SetContainerProperty,
    SetContainerSlot,
    SetCoolDown,
    ChatSuggestions,
    PluginMessage,
    DeleteMessage,
    Disconnect,
    DisguiseChatMessage,
    EntityEvent,
    Explosion,
    UnloadChunk,
    GameEvent,
    OpenHorseScreen,
    InitializeWorldBorder,
    KeepAlive,
    ChunkDataAndUpdateLight,
    WorldEvent,
    Particle,
    UpdateLight,
    Login,
    MapData,
    MerchantOffers,
    UpdateEntityPosition,
    UpdateEntityPositionAndRotation,
    UpdateEntityRotation,
    MoveVehicule,
    OpenBook,
    OpenScreen,
    OpenSignEditor,
    Ping,
    PlaceGhostRecipe,
    PlayerAbilities,
    PlayerChatMessage,
    EndCombat,
    EnterCombat,
    CombatDeath,
    PlayerInfoRemove,
    PlayerInfoUpdate,
    LookAt,
    SynchronizePlayerPosition,
    UpdateRecipeBook,
    RemoveEntities,
    RemoveEntityEffect,
    ResourcePack,
    Respawn,
    SetHeadRotation,
    UpdateSectionBlocks,
    SelectAdvancementsTab,
    ServerData,
    SetActionBarText,
    SetBorderCenter,
    SetBorderLerpSize,
    SetBorderSize,
    SetBorderWarningDelay,
    SetBorderWarningDistance,
    SetCamera,
    SetHeldItem,
    SetCenterChunk,
    SetRenderDistance,
    SetDefaultSpawnPosition,
    DisplayObjective,
    SetEntityMetadata,
    LinkEntities,
    SetEntityVelocity,
    SetEquipement,
    SetExperience,
    SetHealth,
    UpdateObjectives,
    SetPassengers,
    UpdateTeams,
    UpdateScore,
    SetSimulationDistance,
    SetSubtitleText,
    UpdateTime,
    SetTitleText,
    SetTitleAnimationTimes,
    EntitySoundEffect,
    SoundEffect,
    StopSound,
    SystemChatMessage,
    SetTabListHeaderAndFooter,
    TagQuery,
    PickupItem,
    TeleportEntity,
    UpdateAdvancements,
    UpdateAttributes,
    FeaturesFlags,
    EntityEffect,
    UpdateRecipes,
    UpdateTags,
}
