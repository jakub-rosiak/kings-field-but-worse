use bevy::{
    ecs::{message::MessageWriter, system::Res},
    input::{ButtonInput, keyboard::KeyCode},
};

use crate::{
    events::input_intent::InputIntent, resources::input_binding::InputBinding,
    states::actions::Actions, traits::input_map::InputMap,
};

pub fn keyboard_movement_input(
    input: Res<ButtonInput<KeyCode>>,
    bindings: Res<InputBinding>,
    mut intents: MessageWriter<InputIntent>,
) {
    let (mut x, mut z) = (0.0, 0.0);

    if bindings.action_pressed(Actions::MoveForward, input.as_ref()) {
        z -= 1.0;
    }
    if bindings.action_pressed(Actions::MoveBackward, input.as_ref()) {
        z += 1.0;
    }
    if bindings.action_pressed(Actions::MoveLeft, input.as_ref()) {
        x -= 1.0;
    }
    if bindings.action_pressed(Actions::MoveRight, input.as_ref()) {
        x += 1.0;
    }

    if x != 0.0 || z != 0.0 {
        intents.write(InputIntent::Move { x, z });
    }
}

pub fn keyboard_look_input(
    input: Res<ButtonInput<KeyCode>>,
    bindings: Res<InputBinding>,
    mut intents: MessageWriter<InputIntent>,
) {
    let (mut dx, mut dy) = (0.0, 0.0);

    if bindings.action_pressed(Actions::LookUp, input.as_ref()) {
        dy -= 1.0;
    }
    if bindings.action_pressed(Actions::LookDown, input.as_ref()) {
        dy += 1.0;
    }
    if bindings.action_pressed(Actions::LookLeft, input.as_ref()) {
        dx -= 1.0;
    }
    if bindings.action_pressed(Actions::LookRight, input.as_ref()) {
        dx += 1.0;
    }

    if dx != 0.0 || dy != 0.0 {
        intents.write(InputIntent::Look { dx, dy });
    }
}

pub fn keyboard_jump_input(
    input: Res<ButtonInput<KeyCode>>,
    bindings: Res<InputBinding>,
    mut intents: MessageWriter<InputIntent>,
) {
    let pressed = bindings.action_just_pressed(Actions::Jump, input.as_ref());
    let held = bindings.action_pressed(Actions::Jump, input.as_ref());
    let released = bindings.action_just_released(Actions::Jump, input.as_ref());

    if pressed || held || released {
        intents.write(InputIntent::Jump {
            pressed,
            held,
            released,
        });
    }
}

pub fn keyboard_crouch_input(
    input: Res<ButtonInput<KeyCode>>,
    bindings: Res<InputBinding>,
    mut intents: MessageWriter<InputIntent>,
) {
    let pressed = bindings.action_just_pressed(Actions::Crouch, input.as_ref());
    let held = bindings.action_pressed(Actions::Crouch, input.as_ref());
    let released = bindings.action_just_released(Actions::Crouch, input.as_ref());

    if pressed || held || released {
        intents.write(InputIntent::Crouch {
            pressed,
            held,
            released,
        });
    }
}

pub fn keyboard_sprint_input(
    input: Res<ButtonInput<KeyCode>>,
    bindings: Res<InputBinding>,
    mut intents: MessageWriter<InputIntent>,
) {
    let pressed = bindings.action_just_pressed(Actions::Sprint, input.as_ref());
    let held = bindings.action_pressed(Actions::Sprint, input.as_ref());
    let released = bindings.action_just_released(Actions::Sprint, input.as_ref());

    if pressed || held || released {
        intents.write(InputIntent::Sprint {
            pressed,
            held,
            released,
        });
    }
}

pub fn keyboard_interact_input(
    input: Res<ButtonInput<KeyCode>>,
    bindings: Res<InputBinding>,
    mut intents: MessageWriter<InputIntent>,
) {
    let pressed = bindings.action_just_pressed(Actions::Interact, input.as_ref());
    let held = bindings.action_pressed(Actions::Interact, input.as_ref());
    let released = bindings.action_just_released(Actions::Interact, input.as_ref());

    if pressed || held || released {
        intents.write(InputIntent::Interact {
            pressed,
            held,
            released,
        });
    }
}

pub fn keyboard_primary_action_input(
    input: Res<ButtonInput<KeyCode>>,
    bindings: Res<InputBinding>,
    mut intents: MessageWriter<InputIntent>,
) {
    let pressed = bindings.action_just_pressed(Actions::PrimaryAction, input.as_ref());
    let held = bindings.action_pressed(Actions::PrimaryAction, input.as_ref());
    let released = bindings.action_just_released(Actions::PrimaryAction, input.as_ref());

    if pressed || held || released {
        intents.write(InputIntent::PrimaryAction {
            pressed,
            held,
            released,
        });
    }
}

pub fn keyboard_secondary_action_input(
    input: Res<ButtonInput<KeyCode>>,
    bindings: Res<InputBinding>,
    mut intents: MessageWriter<InputIntent>,
) {
    let pressed = bindings.action_just_pressed(Actions::SecondaryAction, input.as_ref());
    let held = bindings.action_pressed(Actions::SecondaryAction, input.as_ref());
    let released = bindings.action_just_released(Actions::SecondaryAction, input.as_ref());

    if pressed || held || released {
        intents.write(InputIntent::SecondaryAction {
            pressed,
            held,
            released,
        });
    }
}

pub fn keyboard_pause_input(
    input: Res<ButtonInput<KeyCode>>,
    bindings: Res<InputBinding>,
    mut intents: MessageWriter<InputIntent>,
) {
    if bindings.action_just_pressed(Actions::Pause, input.as_ref()) {
        intents.write(InputIntent::Pause);
    }
}

pub fn keyboard_menu_input(
    input: Res<ButtonInput<KeyCode>>,
    bindings: Res<InputBinding>,
    mut intents: MessageWriter<InputIntent>,
) {
    if bindings.action_just_pressed(Actions::Menu, input.as_ref()) {
        intents.write(InputIntent::Menu);
    }
}

pub fn keyboard_accept_input(
    input: Res<ButtonInput<KeyCode>>,
    bindings: Res<InputBinding>,
    mut intents: MessageWriter<InputIntent>,
) {
    if bindings.action_just_pressed(Actions::Accept, input.as_ref()) {
        intents.write(InputIntent::Accept);
    }
}

pub fn keyboard_cancel_input(
    input: Res<ButtonInput<KeyCode>>,
    bindings: Res<InputBinding>,
    mut intents: MessageWriter<InputIntent>,
) {
    if bindings.action_just_pressed(Actions::Cancel, input.as_ref()) {
        intents.write(InputIntent::Cancel);
    }
}
