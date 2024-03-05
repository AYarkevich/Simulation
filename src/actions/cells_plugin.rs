use bevy::math::Vec3Swizzles;
use bevy_rapier2d::prelude::*;
use bevy::{
    input::{mouse::MouseButtonInput, ButtonState},
    prelude::*,
    sprite::MaterialMesh2dBundle,
    window::Window,
};
use bevy_rapier2d::geometry::Collider;
use bevy_rapier2d::pipeline::CollisionEvent;

use crate::{
    cameras::main_camera::*,
    core::network::*,
    entities::{cells::*, seed::*, world_board::*},
    ui::info_board::InfoBoardSettings,
};

pub struct CellsPlugin;

impl Plugin for CellsPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, (cells_move_action, cells_spawn_action, update_metrics_system, check_collisions_system));
    }
}

fn distance_squared(point1: Vec2, point2: Vec2) -> f32 {
    let dx = point1.x - point2.x;
    let dy = point1.y - point2.y;
    dx * dx + dy * dy
}

fn find_closest_element(points: &[Vec2], center: Vec2) -> Option<(Vec2)> {
    let mut closest_distance = f32::MAX;
    let mut closest_element: Option<Vec2> = None;

    for &point in points {
        let dist = distance_squared(point, center);
        if dist < closest_distance {
            closest_distance = dist;
            closest_element = Some(point);
        }
    }

    closest_element
}

fn cells_move_action(
    mut commands: Commands,
    mut query: Query<(&mut Transform, &mut Cell, Entity), With<Cell>>,
    mut seeds: Query<(&Transform, Entity), (With<Seed>, Without<Cell>)>,
    boards: Query<&WorldBoard>,
) {
    let summ_of_seeds = seeds
        .iter()
        .map(|(transform, _)| transform.translation)
        .fold((0., 0.), |acc, x| (acc.0 + x.x, acc.1 + x.y));
    let len = seeds.iter().len() as f32;
    let center_of_seeds = Vec2::new(summ_of_seeds.0 / len, summ_of_seeds.1 / len);

    let cells_positions: Vec<Vec2> = seeds
        .iter()
        .map(|(transform, _)| transform.translation.xy())
        .collect();
    let closed_seed = find_closest_element(cells_positions.as_slice(), center_of_seeds);

    //time.seconds_since_starttime();
    for (mut transform, shape, entity) in query.iter_mut() {
        let moving_vect = shape.into_inner().activate(
            transform.translation,
            closed_seed.unwrap_or(Vec2::default()),
        );
        transform.translation = transform.translation + moving_vect;

        let shape_position = transform.translation.xy();
        let world_board_rect = boards.single().settings.rect;

        if !world_board_rect.contains(shape_position) {
            commands.entity(entity).despawn();
        }
    }
}

fn cells_spawn_action(
    mut commands: Commands,
    mut events: EventReader<MouseButtonInput>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    windows: Query<&Window>,
    boards: Query<(&WorldBoard, &Transform), With<WorldBoard>>,
    camera_q: Query<(&Camera, &GlobalTransform), With<GameCapCamera>>,
) {
    for ev in events.read() {
        match ev.state {
            ButtonState::Pressed => {
                let (camera, camera_transform) = camera_q.single();
                let (world_board, _) = boards.single();
                let window = windows.single();

                if let Some(world_position) = window
                    .cursor_position()
                    .and_then(|cursor| camera.viewport_to_world(camera_transform, cursor))
                    .map(|ray| ray.origin.truncate())
                {
                    let board_rect = world_board.settings.rect;
                    if board_rect.contains(world_position) {
                        commands.spawn((
                            MaterialMesh2dBundle {
                                mesh: meshes.add(Circle { radius: 15.0 }).into(),
                                material: materials.add(ColorMaterial::from(Color::PURPLE)),
                                transform: Transform::from_xyz(
                                    world_position.x,
                                    world_position.y,
                                    1.,
                                ),
                                ..default()
                            },
                            Cell {},
                        ))
                            .insert(Collider::ball(15.));
                    }
                }
            }
            ButtonState::Released => {}
        }
    }
}

fn update_metrics_system(
    mut cells: Query<&Cell>,
    mut seeds: Query<&Seed>,
    mut info_board_settings: ResMut<InfoBoardSettings>,
) {
    info_board_settings.world_metrics.cells_count.push(cells.iter().count() as f64);
    info_board_settings.world_metrics.seeds_count.push(seeds.iter().count() as f64);
}

fn check_collisions_system(
    mut commands: Commands,
    mut collision_events: EventReader<CollisionEvent>,
    query: Query<Entity, (With<Collider>, Without<Cell>)>,
) {
    for collision_event in collision_events.read() {
        if let CollisionEvent::Started(entity1, entity2, _) = collision_event {
            let entities = [entity1, entity2];
            for &entity in &entities {
                if let Ok(entity) = query.get(*entity) {
                    commands.entity(entity).despawn();
                }
            }
        }
    }
}