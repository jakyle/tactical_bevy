use std::ops::Deref;

use bevy::prelude::*;

use crate::GameState;

pub struct MainCameraPlugin;
impl Plugin for MainCameraPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_startup_system(spawn_camera.system())
            .add_system_set(
                SystemSet::on_update(GameState::Playing)
                    .with_system(cursor_to_world_camera.system()),
            )
            .insert_resource::<MouseScreenCoords>(MouseScreenCoords::default());
    }
}

pub struct MainCamera;

#[derive(Default)]
pub struct MouseScreenCoords(Vec2);

impl Deref for MouseScreenCoords {
    type Target = Vec2;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

fn spawn_camera(mut commands: Commands) {
    commands
        .spawn_bundle(OrthographicCameraBundle::new_2d())
        .insert(MainCamera);
}

fn cursor_to_world_camera(
    mut mouse_screen_coords: ResMut<MouseScreenCoords>,
    wnds: Res<Windows>,
    camera_query: Query<&Transform, With<MainCamera>>,
) {
    let wnd = wnds.get_primary().unwrap();

    if let Some(pos) = wnd.cursor_position() {
        let size = Vec2::new(wnd.width() as f32, wnd.height() as f32);

        let p = pos - size / 2.0;

        let camera_transform = camera_query.single().unwrap();

        let pos_wld = camera_transform.compute_matrix() * p.extend(0.0).extend(1.0);

        mouse_screen_coords.0 = Vec2::new(pos_wld.x, pos_wld.y);
    }
}
