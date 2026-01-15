use bevy::ecs::message::Message;

#[derive(Debug, Message)]
pub enum InputIntent {
    Move {
        x: f32,
        z: f32,
    },
    Look {
        dx: f32,
        dy: f32,
    },
    Jump {
        pressed: bool,
        held: bool,
        released: bool,
    },
    Crouch {
        pressed: bool,
        held: bool,
        released: bool,
    },
    Sprint {
        pressed: bool,
        held: bool,
        released: bool,
    },
    Interact {
        pressed: bool,
        held: bool,
        released: bool,
    },
    PrimaryAction {
        pressed: bool,
        held: bool,
        released: bool,
    },
    SecondaryAction {
        pressed: bool,
        held: bool,
        released: bool,
    },
    Pause,
    Menu,
    Accept,
    Cancel,
}
