use bevy::{
    input::mouse::MouseButton,
    prelude::*,
};
use bevy_pancam::*;

#[derive(Component)]
pub struct GameCapCamera;

pub struct GameCapCameraPlugin;

impl Plugin for GameCapCameraPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup_camera)
            .add_plugins(PanCamPlugin::default());
    }
}

fn setup_camera(mut commands: Commands) {
    commands.spawn((Camera2dBundle::default(), GameCapCamera))
        .insert(PanCam {
            grab_buttons: vec![MouseButton::Middle], // which buttons should drag the camera
            enabled: true, // when false, controls are disabled. See toggle example.
            zoom_to_cursor: true, // whether to zoom towards the mouse or the center of the screen
            min_scale: 1., // prevent the camera from zooming too far in
            max_scale: Some(40.),
            ..default()// prevent the camera from zooming too far out
        });
}