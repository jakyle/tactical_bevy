use std::ops::{Deref, DerefMut};

use crate::loading::TextureAssets;
use crate::GameState;
use crate::{map::TileSize, movement_actions::MoveActions};
use bevy::prelude::*;
use bevy_ecs_tilemap::MapQuery;

pub struct PlayerPlugin;
impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_event::<PlacePlayerOnTileEvent>()
            .add_system_set(
                SystemSet::on_enter(GameState::Playing).with_system(spawn_player.system()),
            )
            .add_system_set(
                SystemSet::on_update(GameState::Playing)
                    .with_system(start_move.system())
                    .with_system(place_sprite_on_tile.system())
                    .with_system(move_player.system()),
            );
    }
}

#[derive(Debug)]
pub struct Player;

#[derive(Debug)]
pub struct GridPosition(UVec2);

impl GridPosition {
    /// finds tile location by multiplying size of the tile (x, y) by the current
    /// location.  then add half the x and y of tile size to the result to center tile
    pub fn tile_center_translation(&self, tile_size: Vec2, z_index: f32) -> Vec3 {
        Vec3::new(
            (tile_size.x as f32 * self.x as f32) + (tile_size.x / 2.),
            (tile_size.y as f32 * self.y as f32) + (tile_size.y / 2.),
            z_index,
        )
    }
}

impl Deref for GridPosition {
    type Target = UVec2;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

struct PlacePlayerOnTileEvent {
    pub map_id: u16,
    pub layer_id: u16,
}

impl PlacePlayerOnTileEvent {
    fn new(map_id: u16, layer_id: u16) -> Self {
        Self { map_id, layer_id }
    }
}

#[derive(Debug)]
pub struct IsMoving(pub Option<Vec3>);

impl Deref for IsMoving {
    type Target = Option<Vec3>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for IsMoving {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

fn spawn_player(
    mut commands: Commands,
    mut place_player_writer: EventWriter<PlacePlayerOnTileEvent>,
    textures: Res<TextureAssets>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    map_query: MapQuery,
) {
    let grid_position = GridPosition(UVec2::new(2, 2));
    let (layer_entity, _) = map_query.get_layer(0u16, 0u16).unwrap();

    commands.entity(layer_entity).with_children(|parent| {
        parent
            .spawn_bundle(SpriteBundle {
                material: materials.add(textures.texture_bevy.clone().into()),
                transform: Transform::from_translation(Vec3::new(0., 0., 1.)),
                sprite: Sprite {
                    resize_mode: SpriteResizeMode::Manual,
                    ..Default::default()
                },
                ..Default::default()
            })
            .insert(Player)
            .insert(IsMoving(None))
            .insert(grid_position);
    });

    place_player_writer.send(PlacePlayerOnTileEvent::new(0u16, 0u16));
}

fn place_sprite_on_tile(
    tile_size: Res<TileSize>,
    mut place_player_reader: EventReader<PlacePlayerOnTileEvent>,
    mut player_query: Query<(&GridPosition, &mut Sprite, &mut Transform), With<Player>>,
) {
    for _ in place_player_reader.iter() {
        for (grid_position, mut sprite, mut transform) in player_query.iter_mut() {
            let border_offset = Vec2::new(5., 5.);
            sprite.size = tile_size.0 - border_offset;
            transform.translation = grid_position.tile_center_translation(tile_size.0, 1.);
        }
    }
}

fn start_move(
    tile_size: Res<TileSize>,
    actions: Res<MoveActions>,
    map_query: MapQuery,
    mut player_query: Query<(&mut IsMoving, &GridPosition), With<Player>>,
) {
    if actions.player_movement.is_none() {
        return;
    }

    for (mut is_moving, grid_position) in player_query.iter_mut() {
        if is_moving.is_some() {
            continue;
        }

        let direction = actions.player_movement.unwrap();
        let neighbors = map_query.get_tile_neighbors(**grid_position, 0u16, 0u16); // TODO: figure out a way to constantly pass in the correct map id and layer id

        let difference_dest = direction + Vec2::new(grid_position.x as f32, grid_position.y as f32);
        let neighbor_direction = IVec2::new(difference_dest.x as i32, difference_dest.y as i32);

        if let Some((neighbor, Some(_))) = neighbors
            .iter()
            .find(|&(neighbor_loc, _)| neighbor_loc == &neighbor_direction)
        {
            let destination = Vec3::new(
                (neighbor.x as f32 * tile_size.0.x) + (tile_size.0.x / 2.),
                (neighbor.y as f32 * tile_size.0.y) + (tile_size.0.y / 2.),
                1.,
            );

            **is_moving = Some(destination);
        }
    }
}

fn move_player(
    tile_size: Res<TileSize>,
    time: Res<Time>,
    mut player_query: Query<(&mut IsMoving, &mut Transform, &mut GridPosition), With<Player>>,
) {
    for (mut is_moving, mut transform, mut grid_position) in player_query.iter_mut() {
        if let Some(destination) = **is_moving {
            let current_position = transform.translation;

            if current_position.distance(destination) > 0.4 {
                let final_destination = (destination - current_position).normalize();

                let speed = 150.;
                transform.translation += Vec3::new(
                    final_destination.x * speed * time.delta_seconds(),
                    final_destination.y * speed * time.delta_seconds(),
                    0.,
                )
            } else {
                **is_moving = None;
                grid_position.0 = UVec2::new(
                    ((destination.x - (tile_size.0.x / 2.)) / tile_size.0.x as f32) as u32,
                    ((destination.y - (tile_size.0.y / 2.)) / tile_size.0.y as f32) as u32,
                )
            }
        }
    }
}
