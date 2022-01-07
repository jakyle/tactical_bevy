use crate::{camera::MouseScreenCoords, GameState};
use bevy::{input::mouse::MouseButtonInput, prelude::*};

pub struct MouseActionsPlugin;

// This plugin listens for keyboard input and converts the input into Actions
// Actions can then be used as a resource in other systems to act on the player input.
impl Plugin for MouseActionsPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_event::<ClickOnScreenEvent>().add_system_set(
            SystemSet::on_update(GameState::Playing)
                .with_system(set_click_actions.system().label("click_screen")),
        );
    }
}

pub struct ClickOnScreenEvent(pub Vec2);

fn set_click_actions(
    mut click_screen_writer: EventWriter<ClickOnScreenEvent>,
    mouse_coords: Res<MouseScreenCoords>,
    mouse_input: Res<Input<MouseButton>>,
) {
    if MouseControl::LeftClick.just_pressed(&mouse_input)
        || MouseControl::LeftClick.pressed(&mouse_input)
    {
        click_screen_writer.send(ClickOnScreenEvent(**mouse_coords))
    }
}

enum MouseControl {
    LeftClick,
}

impl MouseControl {
    fn just_released(&self, mouse_input: &Res<Input<MouseButton>>) -> bool {
        match self {
            MouseControl::LeftClick => mouse_input.just_released(MouseButton::Left),
        }
    }

    fn pressed(&self, mouse_input: &Res<Input<MouseButton>>) -> bool {
        match self {
            MouseControl::LeftClick => mouse_input.pressed(MouseButton::Left),
        }
    }

    fn just_pressed(&self, mouse_input: &Res<Input<MouseButton>>) -> bool {
        match self {
            MouseControl::LeftClick => mouse_input.just_pressed(MouseButton::Left),
        }
    }
}
