use crate::GameState;
use bevy::prelude::*;
use bevy_asset_loader::prelude::*;

#[derive(Copy, Clone, Debug)]
pub struct LoadingPlugin;

impl Plugin for LoadingPlugin {
    fn build(&self, app: &mut App) {
        app.add_loading_state(
            LoadingState::new(GameState::Loading)
                .continue_to_state(GameState::Playing)
                .load_collection::<TextureAssets>(),
        );
    }
}

#[derive(AssetCollection, Resource)]
pub struct TextureAssets {
    #[asset(texture_atlas_layout(tile_size_x = 10, tile_size_y = 10, columns = 16, rows = 16,))]
    pub ascii_layout: Handle<TextureAtlasLayout>,

    #[asset(path = "textures/ascii.png")]
    pub ascii: Handle<Image>,
}
