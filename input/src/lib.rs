use bevy::{
    app::{Plugin, Update},
    ecs::{schedule::IntoScheduleConfigs, system::Res},
};

use crate::{
    events::input_intent::InputIntent,
    resources::{input_binding::InputBinding, plugin_config::InputPluginConfig},
    systems::{
        gamepad::{
            gamepad_accept_input, gamepad_cancel_input, gamepad_crouch_input,
            gamepad_interact_input, gamepad_jump_input, gamepad_look_input, gamepad_menu_input,
            gamepad_movement_input, gamepad_pause_input, gamepad_primary_action_input,
            gamepad_secondary_action_input, gamepad_sprint_input,
        },
        keyboard::{
            keyboard_accept_input, keyboard_cancel_input, keyboard_crouch_input,
            keyboard_interact_input, keyboard_jump_input, keyboard_look_input, keyboard_menu_input,
            keyboard_movement_input, keyboard_pause_input, keyboard_primary_action_input,
            keyboard_secondary_action_input, keyboard_sprint_input,
        },
        mouse::{mouse_look_input, mouse_primary_action_input, mouse_secondary_action_input},
    },
};

pub mod events;
pub mod resources;
mod states;
mod systems;
mod traits;
pub struct OKInputPlugin {
    config: InputPluginConfig,
}

impl OKInputPlugin {
    pub fn new(config: InputPluginConfig) -> OKInputPlugin {
        OKInputPlugin { config }
    }
}

impl Plugin for OKInputPlugin {
    fn build(&self, app: &mut bevy::app::App) {
        app.insert_resource(self.config.clone())
            .add_message::<InputIntent>()
            .init_resource::<InputBinding>()
            .add_systems(
                Update,
                (
                    keyboard_movement_input,
                    keyboard_look_input,
                    keyboard_jump_input,
                    keyboard_crouch_input,
                    keyboard_sprint_input,
                    keyboard_interact_input,
                    keyboard_primary_action_input,
                    keyboard_secondary_action_input,
                    keyboard_pause_input,
                    keyboard_menu_input,
                    keyboard_accept_input,
                    keyboard_cancel_input,
                )
                    .run_if(|config: Res<InputPluginConfig>| config.keyboard_enabled),
            )
            .add_systems(
                Update,
                (
                    mouse_look_input,
                    mouse_primary_action_input,
                    mouse_secondary_action_input,
                )
                    .run_if(|config: Res<InputPluginConfig>| config.mouse_enabled),
            )
            .add_systems(
                Update,
                (
                    gamepad_movement_input,
                    gamepad_look_input,
                    gamepad_jump_input,
                    gamepad_crouch_input,
                    gamepad_sprint_input,
                    gamepad_interact_input,
                    gamepad_primary_action_input,
                    gamepad_secondary_action_input,
                    gamepad_pause_input,
                    gamepad_menu_input,
                    gamepad_accept_input,
                    gamepad_cancel_input,
                )
                    .run_if(|config: Res<InputPluginConfig>| config.gamepad_enabled),
            );
    }
}
