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
        pool.put(cmd.template::<Bullet>(BulletProp::Inactive).id());
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
        match prop {
            BulletProp::Active(angle, nozzle_position) => cmd
                .insert((
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
                            path: GeometryBuilder::build_as(&shapes::Polygon {
                                points: vec![point3, point1, point2],
                                closed: false,
                            }),
                            ..default()
                        },
                        Fill::color(Color::WHITE.with_alpha(0.)),
                        Stroke::new(Color::WHITE, 3.),
                    ));
                    parent.spawn((Collider::regular_polygon(17., 3), Sensor));
                }),
            BulletProp::Inactive => cmd
                .insert((
                    Bullet,
                    RigidBody::Kinematic,
                    RigidBodyDisabled,
                    Visibility::Hidden,
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
                    parent.spawn((Collider::regular_polygon(17., 3), Sensor));
                }),
        };
        cmd
    }
}
