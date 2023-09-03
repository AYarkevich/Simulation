use bevy::{prelude::*, sprite::MaterialMesh2dBundle};
use rand::prelude::*;

use crate::entities::{seed::*, world_board::*};

const CELLS_STEP: i32 = 32;

pub struct SeedsPlugin;

impl Plugin for SeedsPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(PostStartup, (seeds_spawn_action));
    }
}

fn seeds_spawn_action(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    boards: Query<&WorldBoard>,
    time: Res<Time>,
) {
    let board = boards.single();
    let min_x: i32 = board.settings.rect.min.x as i32 + 3;
    let min_y: i32 = board.settings.rect.min.y as i32 + 3;
    let max_x: i32 = board.settings.rect.max.x as i32 - 3;
    let max_y: i32 = board.settings.rect.max.y as i32 - 3;

    let mut rng = thread_rng();
    for x in (min_x..max_x).step_by(32) {
        for y in (min_y..max_y).step_by(32) {
            let pos_x: f32 = if x + CELLS_STEP < max_x {
                rng.gen_range(x..=x + CELLS_STEP)
            } else {
                rng.gen_range(x..=max_x)
            } as f32;

            let pos_y: f32 = if y + CELLS_STEP < max_y {
                rng.gen_range(y..=y + CELLS_STEP)
            } else {
                rng.gen_range(y..=max_y)
            } as f32;

            commands.spawn((
                MaterialMesh2dBundle {
                    mesh: meshes.add(shape::Cube::new(3.).into()).into(),
                    material: materials.add(ColorMaterial::from(Color::YELLOW)),
                    transform: Transform::from_xyz(pos_x, pos_y, 1.),
                    ..default()
                },
                Seed {},
            ));

            // commands.spawn((
            //     SceneBundle {
            //         scene: assetServer.load("models\\seed.obj"),
            //         transform: Transform::from_xyz(pos_x, pos_y, 1.),
            //         ..default()
            //     },
            //     Seed {},
            // ));
        }
    }
}
