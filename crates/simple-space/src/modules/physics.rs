use avian2d::prelude::*;

#[derive(PhysicsLayer, Default)]
pub enum GameLayer {
    #[default]
    Default,
    Player,
    Enemy,
    Block,
    Bullet,
}
