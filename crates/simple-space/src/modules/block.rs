use avian2d::prelude::*;
use bevy::prelude::*;
use bevy_prototype_lyon::prelude::*;

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

    for _ in 0..1 {
        pool.put(cmd.template::<Block>(true));
    }

    cmd.insert_resource(pool);
}

#[derive(Component)]
// #[require(Transform)]
pub struct Block;

impl PoolMarker for Block {}

impl Template for Block {
    type Prop = bool;
    fn construct(cmd: &mut Commands, active: Self::Prop) -> Entity {
        let mut entt = cmd.spawn((
            Block,
            ShapeBundle {
                path: GeometryBuilder::build_as(&shapes::Rectangle {
                    extents: Vec2::new(50., 50.),
                    origin: shapes::RectangleOrigin::Center,
                    radii: None,
                }),
                transform: Transform::from_xyz(0., 300., 0.),
                ..default()
            },
            Fill::color(Color::WHITE.with_alpha(0.)),
            Stroke::new(Color::WHITE, 3.),
            Collider::rectangle(50., 50.),
            RigidBody::Static,
            SweptCcd::default(),
        ));

        if !active {
            entt.insert((Visibility::Hidden, ColliderDisabled));
        }

        entt.id()
    }
}
