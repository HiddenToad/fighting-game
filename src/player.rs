use crate::BASE_SPEED_DIST;
use bevy::prelude::*;

#[derive(Component)]
pub struct Player {
    pub up: KeyCode,
    pub left: KeyCode,
    pub down: KeyCode,
    pub right: KeyCode,

    pub health: f32,

    pub max_health: f32,
    pub hp_regen: f32,
    pub percent_damage: f32, //percent modifier
    pub flat_damage: f32,    //flat rate
    pub attack_speed: f32,   //percent modifier,
    pub range: f32,          //distance
    pub armor: f32,          //% damage reduction
    pub dodge: f32,          //% dodge
    pub speed: f32,          //% multiplier
}

impl Player {
    pub fn move_dist(&self) -> f32 {
        BASE_SPEED_DIST + (BASE_SPEED_DIST * self.speed)
    }
}
