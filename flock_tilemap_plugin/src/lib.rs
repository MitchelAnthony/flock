use bevy::{prelude::*, render::render_resource::TextureUsages};
use bevy_ecs_tilemap::prelude::*;

pub struct FlockTilemapPlugin;

impl Plugin for FlockTilemapPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_startup_system(setup)
            .add_plugin(TilemapPlugin)
            .add_system(set_texture_filters_to_nearest);
    }
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>, mut map_query: MapQuery) {
    let texture_handle = asset_server.load("tiles.png");

    // Create map entity and component:
    let map_entity = commands.spawn().id();
    let mut map = Map::new(0u16, map_entity);

    // Creates a new layer builder with a layer entity.
    let (mut layer_builder, _) = LayerBuilder::new(
        &mut commands,
        LayerSettings::new(
            MapSize(2, 2),
            ChunkSize(8, 8),
            TileSize(16.0, 16.0),
            TextureSize(96.0, 16.0),
        ),
        0u16,
        0u16,
    );

    layer_builder.set_all(TileBundle::default());
    layer_builder.fill(TilePos(0, 0), TilePos(8, 8), TileBundle { tile: Tile { texture_index: 1, ..default() }, ..default() });
    layer_builder.fill(TilePos(8, 8), TilePos(16, 16), TileBundle { tile: Tile { texture_index: 2, ..default() }, ..default() });

    // Builds the layer.
    // Note: Once this is called you can no longer edit the layer until a hard sync in bevy.
    let layer_entity = map_query.build_layer(&mut commands, layer_builder, texture_handle);

    // Required to keep track of layers for a map internally.
    map.add_layer(&mut commands, 0u16, layer_entity);

    // Spawn Map
    // Required in order to use map_query to retrieve layers/tiles.
    commands
        .entity(map_entity)
        .insert(map)
        .insert(Transform::from_xyz(-128.0, -128.0, 0.0))
        .insert(GlobalTransform::default());
}

fn set_texture_filters_to_nearest(
    mut texture_events: EventReader<AssetEvent<Image>>,
    mut textures: ResMut<Assets<Image>>,
) {
    // quick and dirty, run this for all textures anytime a texture is created.
    for event in texture_events.iter() {
        if let AssetEvent::Created { handle } = event {
            if let Some(mut texture) = textures.get_mut(handle) {
                texture.texture_descriptor.usage = TextureUsages::TEXTURE_BINDING
                    | TextureUsages::COPY_SRC
                    | TextureUsages::COPY_DST;
            }
        }
    }
}

#[cfg(test)]
mod tests {
    // use super::*;

    #[test]
    fn it_works() {
        assert!(true);
    }
}
