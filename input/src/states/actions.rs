#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Actions {
    // KEYBOARD
    MoveForward,
    MoveBackward,
    MoveLeft,
    MoveRight,
    LookUp,
    LookDown,
    LookLeft,
    LookRight,
    // GAMEPAD
    MoveX,
    MoveZ,
    LookX,
    LookY,

    // GENERIC
    // game
    Jump,
    Crouch,
    Sprint,
    Interact,
    PrimaryAction,
    SecondaryAction,

    // menu
    Pause,
    Menu,
    Accept,
    Cancel,
}
