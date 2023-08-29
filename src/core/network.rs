use bevy::math::Vec2Swizzles;
use bevy::prelude::*;

use crate::entities::cells::*;

pub trait CapNetwork {
    fn activate(&mut self, position: Vec3, center_of_mass: Vec2) -> Vec3;
}

impl CapNetwork for Cell {
    fn activate(&mut self, position: Vec3, center_of_mass: Vec2) -> Vec3 {
        let dx = center_of_mass.x - position.x;
        let dy = center_of_mass.y - position.y;
        let vector_to_point = Vec2::new(dx, dy);
        // let length = (dx * dx + dy * dy).sqrt(); // vector length
        // let normalized_dx = dx / length;
        // let normalized_dy = dy / length;

        // let mut rng = thread_rng();
        // let x: f32 = rng.gen_range(-2.0..=2.0);
        // let y: f32 = rng.gen_range(-2.0..=2.0);

        vector_to_point.normalize_or_zero().extend(0.)
    }
}