use avian2d::prelude::*;
use bevy::prelude::*;
use bevy_prototype_lyon::prelude::*;

use crate::constant;

use super::{
    object_pool::{ObjectPool, PoolMarker},
    template::{Template, TemplateExt},
};

pub struct BlockPlugin;

impl Plugin for BlockPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup);
    }
}

fn setup(mut cmd: Commands) {
    let mut pool = ObjectPool::<Block>::new();

    for _ in 0..0 {
        pool.put(
            cmd.template::<Block>(BlockProp::new(Vec2::ZERO, false))
                .id(),
        );
    }

    cmd.insert_resource(pool);
}

#[derive(Component)]
#[require(Transform)]
pub struct Block;

impl PoolMarker for Block {}

pub struct BlockProp {
    active: bool,
    position: Vec2,
}

impl BlockProp {
    pub fn new(position: Vec2, active: bool) -> Self {
        Self { active, position }
    }
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
            RigidBody::Static,
            SweptCcd::default(),
        ));

        if !prop.active {
            cmd.insert((Visibility::Hidden, ColliderDisabled));
        }

        cmd
    }
}
