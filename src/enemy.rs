use bevy::prelude::*;

#[derive(Component)]
pub struct Enemy {
    pub damage: f32, //% modifier
    pub health: f32,
    pub speed: f32, //% modifier
}
