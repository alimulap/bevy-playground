use avian2d::{math::PI, prelude::*};
use bevy::prelude::*;

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

fn setup(mut cmd: Commands, assets: Res<AssetServer>) {
    let mut pool = ObjectPool::<YellowBullet>::new();

    let image = assets.load("effect_yellow.png");
    for _ in 0..20 {
        pool.put(cmd.template::<Bullet>(BulletProp::Inactive(image.clone(), Color::WHITE)));
    }

    cmd.insert_resource(pool);
}

#[derive(Component, Clone)]
#[require(Transform, Visibility)]
pub struct Bullet;

pub struct YellowBullet;

impl PoolMarker for YellowBullet {}

pub enum BulletProp {
    Active(Handle<Image>, Color, f32, Vec3),
    Inactive(Handle<Image>, Color),
}

impl Template for Bullet {
    type Prop = BulletProp;
    fn construct(cmd: &mut Commands, prop: Self::Prop) -> Entity {
        match prop {
            BulletProp::Active(image, color, angle, nozzle_position) => cmd
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
                        Sprite {
                            image,
                            color,
                            ..default()
                        },
                        Transform::from_scale(Vec3::new(0.25, 1., 1.)),
                    ));
                    parent.spawn((
                        Transform::from_translation(Vec3::new(0., 32., 0.)),
                        Collider::rectangle(16., 64.),
                        Sensor,
                    ));
                })
                .id(),
            BulletProp::Inactive(image, color) => cmd
                .spawn((
                    Bullet,
                    RigidBody::Kinematic,
                    RigidBodyDisabled,
                    Visibility::Hidden,
                ))
                .with_children(|parent| {
                    parent.spawn((
                        Sprite {
                            image,
                            color,
                            ..default()
                        },
                        Transform::from_scale(Vec3::new(0.25, 1., 1.)),
                        Visibility::Inherited,
                    ));
                    parent.spawn((
                        Transform::from_translation(Vec3::new(0., 32., 0.)),
                        Collider::rectangle(16., 64.),
                        Sensor,
                    ));
                })
                .id(),
        }
    }
}
