use avian2d::{math::PI, prelude::*};
use bevy::prelude::*;

use super::ship::{Nozzle, Ship};

pub struct BulletPlugin;

impl Plugin for BulletPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup).add_systems(
            Update,
            (
                fire_tick,
                shoot_bullet.run_if(fire_button_pressed.and(can_fire)),
            ),
        );
    }
}

fn setup(mut cmd: Commands) {
    cmd.insert_resource(FireCooldown(Timer::from_seconds(0.1, TimerMode::Repeating)));
}

#[derive(Component)]
#[require(Transform, Visibility)]
pub struct Bullet;

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

fn shoot_bullet(
    mut cmd: Commands,
    assets: Res<AssetServer>,
    ship: Single<&Transform, With<Ship>>,
    nozzle: Single<&GlobalTransform, With<Nozzle>>,
) {
    let image = assets.load("effect_yellow.png");
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
            DebugRender::default(),
        ));
    });
}
