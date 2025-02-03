use avian2d::{math::PI, prelude::*};
use bevy::{ecs::observer::TriggerTargets, prelude::*};

use super::{
    object_pool::{ObjectPool, pool_empty},
    ship::{Nozzle, Ship},
};

pub struct BulletPlugin;

impl Plugin for BulletPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup).add_systems(
            Update,
            (
                fire_tick,
                shoot_bullet_from_pool.run_if(
                    fire_button_pressed
                        .and(can_fire)
                        .and(not(pool_empty::<Bullet>)),
                ),
                shoot_bullet.run_if(fire_button_pressed.and(can_fire).and(pool_empty::<Bullet>)),
            ),
        );
    }
}

fn setup(mut cmd: Commands, assets: Res<AssetServer>) {
    cmd.insert_resource(FireCooldown(Timer::from_seconds(0.1, TimerMode::Repeating)));

    let mut pool = ObjectPool::<Bullet>::new();

    let image = assets.load("effect_yellow.png");
    for _ in 0..20 {
        pool.put(
            cmd.spawn((Bullet, RigidBody::Kinematic, RigidBodyDisabled))
                .with_children(|parent| {
                    parent.spawn((
                        BulletImage,
                        Sprite::from_image(image.clone()),
                        Transform::from_scale(Vec3::new(0.25, 1., 1.)),
                        Visibility::Hidden,
                    ));
                    parent.spawn((
                        Transform::from_translation(Vec3::new(0., 32., 0.)),
                        Collider::rectangle(16., 64.),
                        Sensor,
                    ));
                })
                .id(),
        );
    }

    cmd.insert_resource(pool);
}

#[derive(Component, Clone)]
#[require(Transform, Visibility)]
pub struct Bullet;

#[derive(Component)]
struct BulletImage;

#[derive(Resource)]
struct FireCooldown(Timer);

fn fire_button_pressed(
    keyboard: Res<ButtonInput<KeyCode>>,
    mouse: Res<ButtonInput<MouseButton>>,
) -> bool {
    keyboard.pressed(KeyCode::KeyJ) || mouse.pressed(MouseButton::Left)
}

fn fire_tick(mut fire_cooldown: ResMut<FireCooldown>, time: Res<Time>) {
    fire_cooldown.0.tick(time.delta());
}

fn can_fire(fire_cooldown: Res<FireCooldown>) -> bool {
    fire_cooldown.0.just_finished()
}

fn shoot_bullet_from_pool(
    mut cmd: Commands,
    mut pool: ResMut<ObjectPool<Bullet>>,
    mut bullet_query: Query<(Entity, &mut LinearVelocity), With<Bullet>>,
    ship: Single<Entity, With<Ship>>,
    mut transform: Query<&mut Transform>,
    nozzle: Single<&GlobalTransform, With<Nozzle>>,
    mut bullet_image: Query<(&Parent, &mut Visibility), With<BulletImage>>,
) {
    let bullet = pool.get().unwrap();
    let ship = transform.get_mut(*ship).unwrap();
    let angle = ship.rotation.to_euler(EulerRot::XYZ).2;
    let (entity, mut linvel) = bullet_query.get_mut(bullet).unwrap();
    let mut transform = transform.get_mut(entity).unwrap();
    for (id, mut visibility) in bullet_image.iter_mut() {
        if id.entities()[0] == entity {
            *visibility = Visibility::Visible;
        }
    }
    transform.translation = nozzle.translation();
    transform.rotation = Quat::from_rotation_z(angle - PI / 2.);
    linvel.0 = Vec2 {
        x: angle.cos() * 2000.,
        y: angle.sin() * 2000.,
    };
    cmd.entity(bullet).remove::<RigidBodyDisabled>();
}

fn shoot_bullet(
    mut cmd: Commands,
    assets: Res<AssetServer>,
    ship: Single<Entity, With<Ship>>,
    mut transform: Query<&mut Transform>,
    nozzle: Single<&GlobalTransform, With<Nozzle>>,
) {
    let image = assets.load("effect_yellow.png");
    let ship = transform.get_mut(*ship).unwrap();
    let angle = ship.rotation.to_euler(EulerRot::XYZ).2;
    cmd.spawn((
        Bullet,
        RigidBody::Kinematic,
        LinearVelocity(Vec2 {
            x: angle.cos() * 2000.,
            y: angle.sin() * 2000.,
        }),
        Transform::default()
            .with_translation(nozzle.translation())
            .with_rotation(Quat::from_rotation_z(angle - PI / 2.)),
    ))
    .with_children(|parent| {
        parent.spawn((
            Sprite::from_image(image),
            Transform::from_scale(Vec3::new(0.25, 1., 1.)),
        ));
        parent.spawn((
            Transform::from_translation(Vec3::new(0., 32., 0.)),
            Collider::rectangle(16., 64.),
            Sensor,
        ));
    });
}
