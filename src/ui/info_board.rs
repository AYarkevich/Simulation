use bevy::{
    diagnostic::{DiagnosticsStore, FrameTimeDiagnosticsPlugin},
    prelude::*,
};

use crate::cameras::main_camera::GameCapCamera;

pub struct InfoBoardPlugin;

#[derive(Component)]
struct FpsDetailsText;

#[derive(Component)]
struct MousePointDetailsText;


impl Plugin for InfoBoardPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(FrameTimeDiagnosticsPlugin)
            .add_systems(Startup, infotext_system)
            .add_systems(Update, (change_text_system, cursor_events));
    }
}

fn infotext_system(mut commands: Commands, asset_server: Res<AssetServer>) {
    let font = asset_server.load("fonts/Roboto-Bold.ttf");
    let font_medium: Handle<Font> = asset_server.load("fonts/Roboto-Medium.ttf");
    commands.spawn((
        TextBundle::from_sections([
            TextSection::new(
                "Details: ",
                TextStyle {
                    font: font.clone(),
                    font_size: 12.0,
                    color: Color::WHITE,
                },
            ),
        ])
            .with_text_alignment(TextAlignment::Right)
            .with_style(Style {
                position_type: PositionType::Absolute,
                left: Val::Percent(80.0),
                bottom: Val::Px(29.0),
                ..default()
            }),
    ));

    commands.spawn((
        TextBundle::from_sections([
            TextSection::new(
                "FPS: ",
                TextStyle {
                    font: font.clone(),
                    font_size: 12.0,
                    color: Color::WHITE,
                },
            ),
            TextSection::from_style(TextStyle {
                font: font_medium.clone(),
                font_size: 12.0,
                color: Color::WHITE,
            }),
        ])
            .with_style(Style {
                position_type: PositionType::Absolute,
                left: Val::Percent(80.0),
                bottom: Val::Px(17.0),
                ..default()
            }),
        FpsDetailsText,
    ));

    commands.spawn((
        TextBundle::from_sections([
            TextSection::new(
                "Cursor: ",
                TextStyle {
                    font: font.clone(),
                    font_size: 12.0,
                    color: Color::WHITE,
                },
            ),
            TextSection::from_style(TextStyle {
                font: font_medium.clone(),
                font_size: 12.0,
                color: Color::WHITE,
            }),
        ])
            .with_style(Style {
                position_type: PositionType::Absolute,
                left: Val::Percent(80.0),
                bottom: Val::Px(5.0),
                ..default()
            }),
        MousePointDetailsText,
    ));
}

fn change_text_system(
    time: Res<Time>,
    diagnostics: Res<DiagnosticsStore>,
    mut query: Query<&mut Text, With<FpsDetailsText>>,
) {
    for mut text in &mut query {
        let mut fps = 0.0;
        if let Some(fps_diagnostic) = diagnostics.get(FrameTimeDiagnosticsPlugin::FPS) {
            if let Some(fps_smoothed) = fps_diagnostic.smoothed() {
                fps = fps_smoothed;
            }
        }

        let mut frame_time = time.delta_seconds_f64();
        if let Some(frame_time_diagnostic) = diagnostics.get(FrameTimeDiagnosticsPlugin::FRAME_TIME)
        {
            if let Some(frame_time_smoothed) = frame_time_diagnostic.smoothed() {
                frame_time = frame_time_smoothed;
            }
        }

        text.sections[1].value = format!("{fps:.1} fps, {frame_time:.3} ms/frame", );
    }
}

fn cursor_events(
    mut cursor_evr: EventReader<CursorMoved>,
    mut query: Query<&mut Text, With<MousePointDetailsText>>,
    windows: Query<&Window>,
    camera_q: Query<(&Camera, &GlobalTransform), With<GameCapCamera>>,
) {
    let (camera, camera_transform) = camera_q.single();
    let window = windows.single();

    if let Some(world_position) = window
        .cursor_position()
        .and_then(|cursor| camera.viewport_to_world(camera_transform, cursor))
        .map(|ray| ray.origin.truncate())
    {
        for _ in cursor_evr.iter() {
            for mut text in &mut query {
                text.sections[1].value = format!("X: {}, Y: {}", world_position.x, world_position.y);
            }
        }
    }
}
