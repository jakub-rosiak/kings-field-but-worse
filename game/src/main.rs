use avian3d::PhysicsPlugins;
use bevy::prelude::*;
use avian3d::prelude::*;
use input::{OKInputPlugin, events::input_intent::InputIntent, resources::plugin_config::InputPluginConfig};
use log::debug;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(OKInputPlugin::new(InputPluginConfig::default()))
        .add_plugins(PhysicsPlugins::default())
        .add_systems(Startup, setup)
        .add_systems(Update, log_input)
        .run();
}


fn log_input(mut intent: MessageReader<InputIntent>) {
    let event = intent.read();
    for ev in event {
        debug!("{:?}", ev);
    }
    
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
}