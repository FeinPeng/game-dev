use bevy::prelude::*;
use bevy_asset_loader::prelude::*;
use bevy_kira_audio::AudioSource;

#[derive(AssetCollection, Resource)]
pub struct GameAssets {
    #[asset(path = "arenas/menu.png")]
    pub menu_texture: Handle<Image>,

    #[asset(path = "bricks/brick-sheet.png")]
    pub brick_texture: Handle<Image>,
    #[asset(texture_atlas(tile_size_x = 115, tile_size_y = 40, columns = 11, rows = 1))]
    pub brick_layout: Handle<TextureAtlasLayout>,

    #[asset(path = "arenas/arena-combat-01-sheet.png")]
    pub arena01_texture: Handle<Image>,
    #[asset(texture_atlas(tile_size_x = 1280, tile_size_y = 720, columns = 4, rows = 1))]
    pub arena_combat_01_layout: Handle<TextureAtlasLayout>,

    #[asset(path = "fonts/FiraSans-Bold.ttf")]
    pub font: Handle<Font>,

    #[asset(path = "fonts/ywgh.ttf")]
    pub font_ywgh: Handle<Font>,

    #[asset(path = "balls/tennis.png")]
    pub tennis_texture: Handle<Image>,

    #[asset(path = "balls/none.png")]
    pub none_texture: Handle<Image>,

    #[asset(path = "ui/cursor.png")]
    pub cursor_texture: Handle<Image>,

    #[asset(path = "ui/cursor_invalid.png")]
    pub cursor_invalid_texture: Handle<Image>,

    #[asset(path = "ui/index.png")]
    pub cursor_index: Handle<Image>,

    // enemy start
    #[asset(path = "enemys/enemy_sloth.png")]
    pub enemy_sloth_texture: Handle<Image>,

    #[asset(path = "enemys/enemy_envy.png")]
    pub enemy_envy_texture: Handle<Image>,

    #[asset(path = "enemys/enemy_gluttony.png")]
    pub enemy_gluttony_texture: Handle<Image>,

    #[asset(path = "enemys/enemy_greed.png")]
    pub enemy_greed_texture: Handle<Image>,

    #[asset(path = "enemys/enemy_lust.png")]
    pub enemy_lust_texture: Handle<Image>,

    #[asset(path = "enemys/enemy_pride.png")]
    pub enemy_pride_texture: Handle<Image>,

    #[asset(path = "enemys/enemy_wrath.png")]
    pub enemy_wrath_texture: Handle<Image>,
    // enemy end

    // icon start
    #[asset(path = "icons/boss.png")]
    pub icon_boss: Handle<Image>,

    #[asset(path = "icons/combat.png")]
    pub icon_combat: Handle<Image>,

    #[asset(path = "icons/store.png")]
    pub icon_store: Handle<Image>,

    #[asset(path = "icons/treasure.png")]
    pub icon_tressure: Handle<Image>,

    #[asset(path = "icons/right.png")]
    pub icon_right: Handle<Image>,

    #[asset(path = "icons/wrench.png")]
    pub icon_wrench: Handle<Image>,

    #[asset(path = "icons/exitRight.png")]
    pub icon_exit: Handle<Image>,

    #[asset(path = "icons/branding.png")]
    pub icon_branding: Handle<Image>,
    // icon end

    // item start
    #[asset(path = "items/glue.png")]
    pub item_glue: Handle<Image>,

    #[asset(path = "items/placebo.png")]
    pub item_placebo: Handle<Image>,

    #[asset(path = "items/schoolbag.png")]
    pub item_schoolbag: Handle<Image>,

    #[asset(path = "items/wheel.png")]
    pub item_wheel: Handle<Image>,
    // item end

    // sound start
    #[asset(path = "sounds/tennis.ogg")]
    pub tennis_bounce_sound: Handle<AudioSource>,

    #[asset(path = "sounds/brick_cracked.ogg")]
    pub brick_cracked_sound: Handle<AudioSource>,

    #[asset(path = "sounds/door/qubodup-DoorClose01.ogg")]
    pub close_door_sound: Handle<AudioSource>,
    // sound end

    // music start
    #[asset(path = "music/leap.ogg")]
    pub menu_bgm: Handle<AudioSource>,

    #[asset(path = "music/Long Away Home.ogg")]
    pub gaming_bgm: Handle<AudioSource>,
    // music end
}
