use bevy::prelude::*;
use bevy::window::{Window, WindowPlugin};

mod board;
mod cell;
mod config;
mod systems;

use crate::config::{WINDOW_HEIGHT, WINDOW_WIDTH};
use crate::systems::{auto_reveal, click_system, setup, update_node_appearance};

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                resolution: (WINDOW_WIDTH as u32, WINDOW_HEIGHT as u32).into(),
                ..default()
            }),
            ..default()
        }))
        .add_systems(Startup, setup)
        .add_systems(Update, (update_node_appearance, click_system, auto_reveal))
        .run();
}
