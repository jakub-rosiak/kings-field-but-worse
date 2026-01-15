use bevy::{
    ecs::{
        message::MessageWriter,
        system::{Query, Res},
    },
    input::gamepad::Gamepad,
};

use crate::{
    events::input_intent::InputIntent,
    resources::{input_binding::InputBinding, plugin_config::InputPluginConfig},
    states::actions::Actions,
    traits::input_map::InputMap,
};

pub fn gamepad_movement_input(
    input: Query<&Gamepad>,
    bindings: Res<InputBinding>,
    mut intents: MessageWriter<InputIntent>,
    config: Res<InputPluginConfig>,
) {
    for gamepad in input {
        let x = bindings.get_gamepad_axis(Actions::MoveX, gamepad);
        let z = bindings.get_gamepad_axis(Actions::MoveZ, gamepad);
        if x.abs() > config.gamepad_deadzone || z.abs() > config.gamepad_deadzone {
            intents.write(InputIntent::Move { x, z });
        }
    }
}

pub fn gamepad_look_input(
    input: Query<&Gamepad>,
    bindings: Res<InputBinding>,
    mut intents: MessageWriter<InputIntent>,
    config: Res<InputPluginConfig>,
) {
    for gamepad in input {
        let dx = bindings.get_gamepad_axis(Actions::LookX, gamepad);
        let dy = bindings.get_gamepad_axis(Actions::LookY, gamepad);
        if dx.abs() > config.gamepad_deadzone || dy.abs() > config.gamepad_deadzone {
            intents.write(InputIntent::Look { dx, dy });
        }
    }
}

pub fn gamepad_jump_input(
    input: Query<&Gamepad>,
    bindings: Res<InputBinding>,
    mut intents: MessageWriter<InputIntent>,
) {
    for gamepad in input {
        let pressed = bindings.action_just_pressed(Actions::Jump, gamepad);
        let held = bindings.action_pressed(Actions::Jump, gamepad);
        let released = bindings.action_just_released(Actions::Jump, gamepad);
        if pressed || held || released {
            intents.write(InputIntent::Jump {
                pressed,
                held,
                released,
            });
        }
    }
}

pub fn gamepad_crouch_input(
    input: Query<&Gamepad>,
    bindings: Res<InputBinding>,
    mut intents: MessageWriter<InputIntent>,
) {
    for gamepad in input {
        let pressed = bindings.action_just_pressed(Actions::Crouch, gamepad);
        let held = bindings.action_pressed(Actions::Crouch, gamepad);
        let released = bindings.action_just_released(Actions::Crouch, gamepad);
        if pressed || held || released {
            intents.write(InputIntent::Crouch {
                pressed,
                held,
                released,
            });
        }
    }
}

pub fn gamepad_sprint_input(
    input: Query<&Gamepad>,
    bindings: Res<InputBinding>,
    mut intents: MessageWriter<InputIntent>,
) {
    for gamepad in input {
        let pressed = bindings.action_just_pressed(Actions::Sprint, gamepad);
        let held = bindings.action_pressed(Actions::Sprint, gamepad);
        let released = bindings.action_just_released(Actions::Sprint, gamepad);
        if pressed || held || released {
            intents.write(InputIntent::Sprint {
                pressed,
                held,
                released,
            });
        }
    }
}

pub fn gamepad_interact_input(
    input: Query<&Gamepad>,
    bindings: Res<InputBinding>,
    mut intents: MessageWriter<InputIntent>,
) {
    for gamepad in input {
        let pressed = bindings.action_just_pressed(Actions::Interact, gamepad);
        let held = bindings.action_pressed(Actions::Interact, gamepad);
        let released = bindings.action_just_released(Actions::Interact, gamepad);
        if pressed || held || released {
            intents.write(InputIntent::Interact {
                pressed,
                held,
                released,
            });
        }
    }
}

pub fn gamepad_primary_action_input(
    input: Query<&Gamepad>,
    bindings: Res<InputBinding>,
    mut intents: MessageWriter<InputIntent>,
) {
    for gamepad in input {
        let pressed = bindings.action_just_pressed(Actions::PrimaryAction, gamepad);
        let held = bindings.action_pressed(Actions::PrimaryAction, gamepad);
        let released = bindings.action_just_released(Actions::PrimaryAction, gamepad);
        if pressed || held || released {
            intents.write(InputIntent::PrimaryAction {
                pressed,
                held,
                released,
            });
        }
    }
}

pub fn gamepad_secondary_action_input(
    input: Query<&Gamepad>,
    bindings: Res<InputBinding>,
    mut intents: MessageWriter<InputIntent>,
) {
    for gamepad in input {
        let pressed = bindings.action_just_pressed(Actions::SecondaryAction, gamepad);
        let held = bindings.action_pressed(Actions::SecondaryAction, gamepad);
        let released = bindings.action_just_released(Actions::SecondaryAction, gamepad);
        if pressed || held || released {
            intents.write(InputIntent::SecondaryAction {
                pressed,
                held,
                released,
            });
        }
    }
}

pub fn gamepad_pause_input(
    input: Query<&Gamepad>,
    bindings: Res<InputBinding>,
    mut intents: MessageWriter<InputIntent>,
) {
    for gamepad in input {
        if bindings.action_just_pressed(Actions::Pause, gamepad) {
            intents.write(InputIntent::Pause);
        }
    }
}

pub fn gamepad_menu_input(
    input: Query<&Gamepad>,
    bindings: Res<InputBinding>,
    mut intents: MessageWriter<InputIntent>,
) {
    for gamepad in input {
        if bindings.action_just_pressed(Actions::Menu, gamepad) {
            intents.write(InputIntent::Menu);
        }
    }
}

pub fn gamepad_accept_input(
    input: Query<&Gamepad>,
    bindings: Res<InputBinding>,
    mut intents: MessageWriter<InputIntent>,
) {
    for gamepad in input {
        if bindings.action_just_pressed(Actions::Accept, gamepad) {
            intents.write(InputIntent::Accept);
        }
    }
}

pub fn gamepad_cancel_input(
    input: Query<&Gamepad>,
    bindings: Res<InputBinding>,
    mut intents: MessageWriter<InputIntent>,
) {
    for gamepad in input {
        if bindings.action_just_pressed(Actions::Cancel, gamepad) {
            intents.write(InputIntent::Cancel);
        }
    }
}
