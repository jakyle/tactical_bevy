use bevy::prelude::*;
use bevy_ecs_tilemap::prelude::*;

use crate::{
    loading::TextureAssets,
    mouse_actions::ClickOnScreenEvent,
    player::{IsMoving, Player},
    GameState,
};

pub struct MapPlugin;
impl Plugin for MapPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_system_set(SystemSet::on_exit(GameState::Menu).with_system(spawn_map.system()))
            .add_system_set(
                SystemSet::new()
                    .after("click_screen")
                    .with_system(in_map_bounds.system()),
            );
    }
}

pub struct GridMap;
pub struct TileSize(pub Vec2);

fn spawn_map(
    mut commands: Commands,
    texture_assets: Res<TextureAssets>,
    mut map_query: MapQuery,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    let material_handle =
        materials.add(ColorMaterial::texture(texture_assets.grid_texture.clone()));

    let tile_area = 50.0;

    // Create map entity and component
    let map_entity = commands.spawn().id();
    let mut map = Map::new(0u16, map_entity);
    let layer_settings = LayerSettings::new(
        UVec2::new(4, 4),
        UVec2::new(2, 2),
        Vec2::new(tile_area, tile_area),
        Vec2::new(tile_area * 6., tile_area),
    );

    let center = layer_settings.get_pixel_center();

    let (mut layer_builder, _) = LayerBuilder::new(&mut commands, layer_settings, 0u16, 0u16);

    layer_builder.set_all(TileBundle::default());

    let tile_size = layer_builder.settings.tile_size;

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
        .insert(grid_transform)
        .insert(GlobalTransform::default())
        .insert(GridMap);

    commands.insert_resource(TileSize(tile_size));
}

fn in_map_bounds(
    tile_size: Option<Res<TileSize>>,
    mut click_screen_reader: EventReader<ClickOnScreenEvent>,
    grid_query: Query<&GlobalTransform, With<GridMap>>,
    mut moving_query: Query<&mut IsMoving, With<Player>>,
) {
    for event in click_screen_reader.iter() {
        for mut moving in moving_query.iter_mut() {
            if moving.is_some() {
                return;
            }

            let (x, y) = (event.0.x, event.0.y);
            let g_transform = grid_query.iter().next().unwrap();
            let (gx, gy) = (g_transform.translation.x, g_transform.translation.y);

            if x > gx && x < gx.abs() && y > gy && y < gy.abs() {
                if let Some(tile_size) = &tile_size {

                    let tile_x = (((x + gx.abs()) as u32 / tile_size.0.x as u32) + 1) as f32
                        * tile_size.0.x
                        - (tile_size.0.x / 2.);

                    let tile_y = (((y + gy.abs()) as u32 / tile_size.0.y as u32) + 1) as f32
                        * tile_size.0.y
                        - (tile_size.0.y / 2.);

                    **moving = Some(Vec3::new(tile_x, tile_y, 1.))
                }
            }
        }
    }
}
