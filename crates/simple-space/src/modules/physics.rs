// use bevy::prelude::*;
use avian2d::prelude::*;

#[allow(unused)]
#[derive(PhysicsLayer, Default)]
pub enum GameLayer {
    #[default]
    Default,
    Player,
    Enemy,
    Block,
    Bullet,
}
