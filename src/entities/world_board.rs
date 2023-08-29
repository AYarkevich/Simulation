use bevy::{
    prelude::*,
};

#[derive(Component)]
pub struct WorldBoard {
    pub(crate) settings: BoardSettings,
}

pub struct BoardSettings {
    pub(crate) rect: Rect,
}