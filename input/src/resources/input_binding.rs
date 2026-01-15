use bevy::{
    ecs::resource::Resource,
    input::{
        ButtonInput,
        gamepad::{Gamepad, GamepadAxis, GamepadButton},
        keyboard::KeyCode,
        mouse::MouseButton,
    },
    platform::collections::HashMap,
};

use crate::{states::actions::Actions, traits::input_map::InputMap};

macro_rules! input_map_impl {
    ($t:ty, $s:ty, $map:ident) => {
        impl InputMap<$t> for $s {
            fn action_pressed(&self, actions: Actions, input: &$t) -> bool {
                self.$map
                    .get(&actions)
                    .map_or(false, |key| input.pressed(*key))
            }
            fn action_just_pressed(&self, actions: Actions, input: &$t) -> bool {
                self.$map
                    .get(&actions)
                    .map_or(false, |key| input.just_pressed(*key))
            }

            fn action_just_released(&self, actions: Actions, input: &$t) -> bool {
                self.$map
                    .get(&actions)
                    .map_or(false, |key| input.just_released(*key))
            }
        }
    };
}

#[derive(Debug, Resource)]
pub struct InputBinding {
    pub key_map: HashMap<Actions, KeyCode>,
    pub mouse_map: HashMap<Actions, MouseButton>,
    pub gamepad_button_map: HashMap<Actions, GamepadButton>,
    pub gamepad_axis_map: HashMap<Actions, GamepadAxis>,
}

impl Default for InputBinding {
    fn default() -> Self {
        let mut key_map = HashMap::new();
        key_map.insert(Actions::MoveForward, KeyCode::KeyW);
        key_map.insert(Actions::MoveBackward, KeyCode::KeyS);
        key_map.insert(Actions::MoveLeft, KeyCode::KeyA);
        key_map.insert(Actions::MoveRight, KeyCode::KeyD);
        key_map.insert(Actions::Jump, KeyCode::Space);
        key_map.insert(Actions::Crouch, KeyCode::ControlLeft);
        key_map.insert(Actions::Sprint, KeyCode::ShiftLeft);
        key_map.insert(Actions::Interact, KeyCode::KeyE);
        key_map.insert(Actions::Pause, KeyCode::Escape);
        key_map.insert(Actions::Menu, KeyCode::KeyI);
        key_map.insert(Actions::Accept, KeyCode::Enter);
        key_map.insert(Actions::Cancel, KeyCode::Backspace);

        let mut mouse_map = HashMap::new();

        mouse_map.insert(Actions::PrimaryAction, MouseButton::Left);
        mouse_map.insert(Actions::SecondaryAction, MouseButton::Right);

        let mut gamepad_button_map = HashMap::new();

        gamepad_button_map.insert(Actions::Jump, GamepadButton::South);
        gamepad_button_map.insert(Actions::Crouch, GamepadButton::East);
        gamepad_button_map.insert(Actions::Sprint, GamepadButton::LeftThumb);
        gamepad_button_map.insert(Actions::Interact, GamepadButton::West);
        gamepad_button_map.insert(Actions::PrimaryAction, GamepadButton::RightTrigger);
        gamepad_button_map.insert(Actions::SecondaryAction, GamepadButton::LeftTrigger);

        gamepad_button_map.insert(Actions::Pause, GamepadButton::Start);
        gamepad_button_map.insert(Actions::Menu, GamepadButton::Select);
        gamepad_button_map.insert(Actions::Accept, GamepadButton::South);
        gamepad_button_map.insert(Actions::Cancel, GamepadButton::East);

        let mut gamepad_axis_map = HashMap::new();

        gamepad_axis_map.insert(Actions::MoveX, GamepadAxis::LeftStickX);
        gamepad_axis_map.insert(Actions::MoveZ, GamepadAxis::LeftStickY);
        gamepad_axis_map.insert(Actions::LookX, GamepadAxis::RightStickX);
        gamepad_axis_map.insert(Actions::LookY, GamepadAxis::RightStickY);

        Self {
            key_map,
            mouse_map,
            gamepad_button_map,
            gamepad_axis_map,
        }
    }
}

input_map_impl!(ButtonInput<KeyCode>, InputBinding, key_map);

input_map_impl!(ButtonInput<MouseButton>, InputBinding, mouse_map);

input_map_impl!(Gamepad, InputBinding, gamepad_button_map);

impl InputBinding {
    pub fn get_gamepad_axis(&self, actions: Actions, input: &Gamepad) -> f32 {
        if let Some(axis) = self.gamepad_axis_map.get(&actions) {
            input.get(*axis).unwrap_or(0.0)
        } else {
            0.0
        }
    }
}
