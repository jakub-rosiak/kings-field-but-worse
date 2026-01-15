use bevy::ecs::resource::Resource;

#[derive(Debug, Clone, Resource)]
pub struct InputPluginConfig {
    pub keyboard_enabled: bool,
    pub mouse_enabled: bool,
    pub gamepad_enabled: bool,
    pub gamepad_deadzone: f32,
}

impl Default for InputPluginConfig {
    fn default() -> Self {
        Self {
            keyboard_enabled: true,
            mouse_enabled: true,
            gamepad_enabled: true,
            gamepad_deadzone: 0.15,
        }
    }
}
