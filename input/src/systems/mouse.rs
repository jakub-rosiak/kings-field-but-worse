use bevy::{
    ecs::{message::MessageWriter, system::Res},
    input::{
        ButtonInput,
        mouse::{AccumulatedMouseMotion, MouseButton},
    },
    math::Vec2,
};

use crate::{
    events::input_intent::InputIntent, resources::input_binding::InputBinding,
    states::actions::Actions, traits::input_map::InputMap,
};

pub fn mouse_look_input(
    accumulated_mouse_motion: Res<AccumulatedMouseMotion>,
    mut intents: MessageWriter<InputIntent>,
) {
    let delta = accumulated_mouse_motion.delta;

    if delta != Vec2::ZERO {
        intents.write(InputIntent::Look {
            dx: delta.x,
            dy: delta.y,
        });
    }
}

pub fn mouse_primary_action_input(
    input: Res<ButtonInput<MouseButton>>,
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

pub fn mouse_secondary_action_input(
    input: Res<ButtonInput<MouseButton>>,
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
