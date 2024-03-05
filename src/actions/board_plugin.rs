use bevy::{
    prelude::*,
};

use crate::{
  entities::world_board::*,
};

pub struct WorldBoardPlugin;

impl Plugin for WorldBoardPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, board_spawn_action)
            .insert_resource(ClearColor(Color::MAROON));
    }
}

fn board_spawn_action(mut commands: Commands) {
    let board_rect = Rect::new(-250., -250., 250., 250.);

    commands.spawn((
        SpriteBundle {
            sprite: Sprite {
                color: Color::rgb(0.25, 0.25, 0.75),
                rect: Some(board_rect),
                ..default()
            },
            transform: Transform::from_translation(Vec3::new(0., 0., 0.)),
            ..default()
        },
        WorldBoard {
            settings: BoardSettings {
                rect: board_rect
            }
        },
    ));
}