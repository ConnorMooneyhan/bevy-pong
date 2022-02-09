use crate::prelude::*;

#[derive(Component)]
pub struct Paddle {
    pub up_key: KeyCode,
    pub down_key: KeyCode,
}

#[derive(Component)]
pub struct Ball {
    pub velocity: Vec3,
}
