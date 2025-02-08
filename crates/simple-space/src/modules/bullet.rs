use avian2d::{math::PI, prelude::*};
use bevy::prelude::*;
use bevy_prototype_lyon::prelude::*;

use super::{
    object_pool::{ObjectPool, PoolMarker},
    template::{Template, TemplateExt},
};

pub struct BulletPlugin;

impl Plugin for BulletPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup);
    }
}

fn setup(mut cmd: Commands) {
    let mut pool = ObjectPool::<YellowBullet>::new();

    for _ in 0..20 {
        pool.put(cmd.template::<Bullet>(BulletProp::Inactive));
    }

    cmd.insert_resource(pool);
}

#[derive(Component, Clone)]
#[require(Transform, Visibility)]
pub struct Bullet;

pub struct YellowBullet;

impl PoolMarker for YellowBullet {}

pub enum BulletProp {
    Active(f32, Vec3),
    Inactive,
}

impl Template for Bullet {
    type Prop = BulletProp;
    fn construct(cmd: &mut Commands, prop: Self::Prop) -> Entity {
        match prop {
            BulletProp::Active(angle, nozzle_position) => cmd
                .spawn((
                    Bullet,
                    RigidBody::Kinematic,
                    LinearVelocity(Vec2 {
                        x: angle.cos() * 2000.,
                        y: angle.sin() * 2000.,
                    }),
                    Transform::default()
                        .with_translation(nozzle_position)
                        .with_rotation(Quat::from_rotation_z(angle - PI / 2.)),
                ))
                .with_children(|parent| {
                    parent.spawn((
                        ShapeBundle {
                            path: GeometryBuilder::build_as(&shapes::RegularPolygon {
                                sides: 3,
                                feature: shapes::RegularPolygonFeature::Radius(17.),
                                ..default()
                            }),
                            ..default()
                        },
                        Fill::color(Color::WHITE.with_alpha(0.)),
                        Stroke::new(Color::WHITE, 3.),
                    ));
                    parent.spawn((Collider::regular_polygon(10., 3), Sensor));
                })
                .id(),
            BulletProp::Inactive => cmd
                .spawn((
                    Bullet,
                    RigidBody::Kinematic,
                    RigidBodyDisabled,
                    Visibility::Hidden,
                ))
                .with_children(|parent| {
                    parent.spawn((
                        ShapeBundle {
                            path: GeometryBuilder::build_as(&shapes::RegularPolygon {
                                sides: 3,
                                feature: shapes::RegularPolygonFeature::Radius(17.),
                                ..default()
                            }),
                            ..default()
                        },
                        Fill::color(Color::WHITE.with_alpha(0.)),
                        Stroke::new(Color::WHITE, 3.),
                    ));
                    parent.spawn((Collider::regular_polygon(10., 3), Sensor));
                })
                .id(),
        }
    }
}
