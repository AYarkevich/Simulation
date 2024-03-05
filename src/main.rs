use bevy::{
    prelude::*,
    window::{PresentMode, Window, WindowPlugin},
};
use bevy_rapier2d::plugin::{NoUserData, RapierConfiguration, RapierPhysicsPlugin};

use crate::{
    actions::{
        board_plugin::*,
        cells_plugin::*,
        seeds_plugin::*,
    },
    cameras::main_camera::*,
    ui::{
        debug::*,
        info_board::*,
    },
};

mod cameras;
mod ui;
mod core;
mod actions;
mod entities;
mod utils;


fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "Game_cap!".into(),
                resolution: (900., 600.).into(),
                present_mode: PresentMode::AutoVsync,
                prevent_default_event_handling: false,
                ..default()
            }),
            ..default()
        }))
        .add_systems(Startup, setup_system)
        .add_plugins(RapierPhysicsPlugin::<NoUserData>::default())
        .add_plugins((
            GameCapCameraPlugin,
            DebugPlugin,
            InfoBoardPlugin,
            WorldBoardPlugin,
            CellsPlugin,
            SeedsPlugin,
        ))
        .run();
}

fn setup_system(
    mut commands: Commands,
) {
    commands.insert_resource(RapierConfiguration {
        gravity: Vec2::new(0., 0.),
        ..Default::default()
    });
}

