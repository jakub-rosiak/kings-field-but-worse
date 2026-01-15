use avian3d::PhysicsPlugins;
use bevy::prelude::*;
use input::{
    OKInputPlugin, events::input_intent::InputIntent, resources::plugin_config::InputPluginConfig,
};
use log::debug;

use crate::pooga::Pooga;

mod pooga;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(OKInputPlugin::new(InputPluginConfig::default()))
        .add_plugins(PhysicsPlugins::default())
        .add_plugins(Pooga::new())
        .add_systems(Update, log_input)
        .run();
}

fn log_input(mut intent: MessageReader<InputIntent>) {
    let event = intent.read();
    for ev in event {
        debug!("{:?}", ev);
    }
}
