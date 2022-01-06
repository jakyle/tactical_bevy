use bevy::prelude::*;
use bevy_ecs_tilemap::prelude::*;

use crate::GameState;

pub struct MapPlugin;
impl Plugin for MapPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_startup_system(spawn_map.system()).add_system_set(
            SystemSet::on_enter(GameState::Playing).with_system(spawn_camera.system()),
        );
    }
}

fn spawn_camera(mut commands: Commands) {
    commands.spawn_bundle(OrthographicCameraBundle::new_2d());
}

pub struct MapTransform(pub Transform);

fn spawn_map(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut map_query: MapQuery,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    println!("spawn map");
    let texture_handle = asset_server.load("textures/tiles.png");
    let material_handle = materials.add(ColorMaterial::texture(texture_handle));

    let tile_area = 50.0;

    // Create map entity and component
    let map_entity = commands.spawn().id();
    let mut map = Map::new(0u16, map_entity);
    let layer_settings = LayerSettings::new(
        UVec2::new(2, 2),
        UVec2::new(10, 10),
        Vec2::new(tile_area, tile_area),
        Vec2::new(tile_area * 6., tile_area),
    );

    let center = layer_settings.get_pixel_center();

    let (mut layer_builder, _) = LayerBuilder::new(&mut commands, layer_settings, 0u16, 0u16);

    layer_builder.set_all(TileBundle::default());

    // Builds the layer.
    // Note: Once this is called you can no longer edit the layer until a hard sync in bevy.
    let layer_entity = map_query.build_layer(&mut commands, layer_builder, material_handle);

    // Required to keep track of layers for a map internally.
    map.add_layer(&mut commands, 0u16, layer_entity);

    let grid_transform = Transform::from_xyz(-center.x, -center.y, 0.0);

    // Spawn Map
    // Required in order to use map_query to retrieve layers/tiles
    commands
        .entity(map_entity)
        .insert(map)
        .insert(grid_transform.clone())
        .insert(GlobalTransform::default());

    commands.insert_resource(MapTransform(grid_transform));
}
