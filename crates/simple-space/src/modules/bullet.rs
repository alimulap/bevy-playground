use avian2d::{math::PI, prelude::*};
use bevy::prelude::*;
use bevy_prototype_lyon::prelude::*;

use super::template::Template;

#[derive(Component, Clone)]
#[require(Transform, Visibility)]
pub struct Bullet;

#[derive(Component)]
pub enum BulletType {
    Standard,
}

pub struct BulletProp {
    pub rotation: f32,
    pub position: Vec3,
    pub bullet_type: BulletType,
    pub layers: CollisionLayers,
}

impl Template for Bullet {
    type Prop = BulletProp;
    fn construct(mut cmd: EntityCommands<'_>, prop: Self::Prop) -> EntityCommands<'_> {
        let point1 = Vec2::new(0., 13.);
        let point2 = Vec2::new(
            210f32.to_radians().cos() * 13.,
            210f32.to_radians().sin() * 13.,
        );
        let point3 = Vec2::new(
            330f32.to_radians().cos() * 13.,
            330f32.to_radians().sin() * 13.,
        );

        let BulletProp {
            rotation,
            position,
            bullet_type,
            layers,
        } = prop;

        cmd.insert((
            Bullet,
            bullet_type,
            RigidBody::Kinematic,
            LinearVelocity(Vec2 {
                x: rotation.cos() * 2000.,
                y: rotation.sin() * 2000.,
            }),
            Transform::default()
                .with_translation(position)
                .with_rotation(Quat::from_rotation_z(rotation - PI / 2.)),
            Collider::regular_polygon(17., 3),
            Sensor,
            layers,
            CollidingEntities::default(),
            // CollisionLayers::new( GameLayer::Bullet,
            //     [GameLayer::Default, GameLayer::Block, GameLayer::Enemy],
            // ),
        ))
        .with_children(|parent| {
            parent.spawn((
                ShapeBundle {
                    path: GeometryBuilder::build_as(&shapes::Polygon {
                        points: vec![point3, point1, point2],
                        closed: false,
                    }),
                    ..default()
                },
                Fill::color(Color::WHITE.with_alpha(0.)),
                Stroke::new(Color::WHITE, 3.),
            ));
        });
        cmd
    }
}
