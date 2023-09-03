use bevy::{
    prelude::*,
    window::{PresentMode, Window, WindowPlugin},
};

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
                fit_canvas_to_parent: true,
                prevent_default_event_handling: false,
                ..default()
            }),
            ..default()
        }))
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
