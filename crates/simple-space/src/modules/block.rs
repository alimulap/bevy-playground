use avian2d::prelude::*;
use bevy::prelude::*;
use bevy_prototype_lyon::prelude::*;

use crate::constant;

use super::{physics::GameLayer, template::Template};

#[derive(Component)]
#[require(Transform)]
pub struct Block;

pub struct BlockProp {
    pub position: Vec2,
}

impl Template for Block {
    type Prop = BlockProp;
    fn construct(mut cmd: EntityCommands<'_>, prop: Self::Prop) -> EntityCommands<'_> {
        cmd.insert((
            Block,
            ShapeBundle {
                path: GeometryBuilder::build_as(&shapes::Rectangle {
                    extents: Vec2::new(constant::BLOCK_SIZE, constant::BLOCK_SIZE),
                    origin: shapes::RectangleOrigin::Center,
                    radii: None,
                }),
                transform: Transform::from_translation(Vec3::from((prop.position, 0.))),
                ..default()
            },
            Fill::color(Color::WHITE.with_alpha(0.)),
            Stroke::new(Color::WHITE, 3.),
            Collider::rectangle(constant::BLOCK_SIZE, constant::BLOCK_SIZE),
            CollisionLayers::new(
                GameLayer::Block,
                [
                    GameLayer::Default,
                    GameLayer::Player,
                    GameLayer::Enemy,
                    GameLayer::Bullet,
                ],
            ),
            RigidBody::Static,
            SweptCcd::default(),
        ));

        cmd
    }
}
