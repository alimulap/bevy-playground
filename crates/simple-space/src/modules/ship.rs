use avian2d::{
    math::{PI, Vector},
    prelude::*,
};
use bevy::prelude::*;
use playground_ui::DebugLog;

use crate::CursorPosition;

use super::{
    bullet::{Bullet, BulletProp, YellowBullet},
    object_pool::{ObjectPool, pool_empty},
    template::TemplateExt,
};

pub struct ShipPlugin;

impl Plugin for ShipPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup).add_systems(
            Update,
            (
                ship_strafe,
                switch_rotate_method.run_if(switch_key_pressed),
                look_at_cursor.run_if(resource_equals(RotateMethod::Cursor)),
                rotate_with_keyboard.run_if(resource_equals(RotateMethod::Keyboard)),
                fire_tick,
                shoot_bullet_from_pool.run_if(
                    fire_button_pressed
                        .and(can_fire)
                        .and(not(pool_empty::<YellowBullet>)),
                ),
                shoot_bullet.run_if(
                    fire_button_pressed
                        .and(can_fire)
                        .and(pool_empty::<YellowBullet>),
                ),
            ),
        );
    }
}

#[derive(Component)]
#[require(Transform, Visibility)]
pub struct Ship;

#[derive(Component)]
pub struct Nozzle;

fn setup(mut cmd: Commands, assets: Res<AssetServer>) {
    cmd.spawn((
        Camera2d,
        Projection::from(OrthographicProjection {
            scale: 3.,
            ..OrthographicProjection::default_2d()
        }),
    ));
    cmd.insert_resource(FireCooldown(Timer::from_seconds(0.1, TimerMode::Repeating)));

    let ship_g = assets.load("ship_G.png");

    cmd.insert_resource(RotateMethod::Cursor);

    cmd.spawn((Ship, MaxSpeed(1000.), RigidBody::Kinematic))
        .with_children(|parent| {
            parent.spawn((
                Sprite::from_image(ship_g),
                Transform::default().with_rotation(Quat::from_rotation_z(-(PI / 2.))),
            ));
            parent.spawn((
                Collider::compound(vec![
                    (
                        Position::new(Vec2::default()),
                        Rotation::default(),
                        Collider::triangle(
                            Vector::new(50., 0.),
                            Vector::new(-31., -48.),
                            Vector::new(-31., 48.),
                        ),
                    ),
                    (
                        Position::new(Vec2::new(-31., -32.)),
                        Rotation::default(),
                        Collider::triangle(
                            Vector::new(0., -17.),
                            Vector::new(0., 17.),
                            Vector::new(-17., 0.),
                        ),
                    ),
                    (
                        Position::new(Vec2::new(-31., 32.)),
                        Rotation::default(),
                        Collider::triangle(
                            Vector::new(0., -17.),
                            Vector::new(0., 17.),
                            Vector::new(-17., 0.),
                        ),
                    ),
                ]),
                DebugRender::default(),
            ));
            parent.spawn((Nozzle, Transform::from_xyz(50., 0., 0.)));
        });
}

#[derive(Component)]
pub struct MaxSpeed(f32);

fn ship_strafe(
    keyboard: Res<ButtonInput<KeyCode>>,
    ship: Single<(&mut LinearVelocity, &MaxSpeed), With<Ship>>,
) {
    let (mut linvel, max_speed) = ship.into_inner();
    let mut direction = Vector::ZERO;
    match (
        keyboard.pressed(KeyCode::KeyD),
        keyboard.pressed(KeyCode::KeyA),
    ) {
        (true, false) => direction.x = 1.0f32,
        (false, true) => direction.x = -1.0f32,
        _ => (),
    }
    match (
        keyboard.pressed(KeyCode::KeyW),
        keyboard.pressed(KeyCode::KeyS),
    ) {
        (true, false) => direction.y = 1.0f32,
        (false, true) => direction.y = -1.0f32,
        _ => (),
    }

    let linve_magnitude = linvel.0.length();

    if direction != Vector::ZERO {
        if linve_magnitude <= max_speed.0 {
            direction = direction.normalize() * 1000.;
            linvel.0 = linvel.0.lerp(direction, 0.1);
        } else {
            linvel.0 = linvel.0.normalize() * max_speed.0;
        }
    } else if linve_magnitude > 0.1 {
        linvel.0 = linvel.0.lerp(Vector::ZERO, 0.1);
    } else {
        linvel.0 = Vector::ZERO;
    }
}

#[derive(Resource, PartialEq, Eq)]
pub enum RotateMethod {
    Cursor,
    Keyboard,
}

fn switch_key_pressed(keyboard: Res<ButtonInput<KeyCode>>) -> bool {
    keyboard.just_pressed(KeyCode::KeyY)
}

fn switch_rotate_method(mut rotate_method: ResMut<RotateMethod>) {
    *rotate_method = match *rotate_method {
        RotateMethod::Cursor => RotateMethod::Keyboard,
        RotateMethod::Keyboard => RotateMethod::Cursor,
    }
}

fn look_at_cursor(
    cursor_position: Res<CursorPosition>,
    camera: Single<(&Camera, &GlobalTransform)>,
    mut ship: Single<(&mut Transform, &GlobalTransform), With<Ship>>,
    mut debug_log: ResMut<DebugLog>,
) {
    let cursor_position = cursor_position.0;
    let ship_on_viewport = camera
        .0
        .world_to_viewport(camera.1, ship.1.translation())
        .unwrap();
    let angle =
        (ship_on_viewport.y - cursor_position.y).atan2(cursor_position.x - ship_on_viewport.x);
    ship.0.rotation =
        Quat::from(Rotation::from(ship.0.rotation).nlerp(Rotation::radians(angle), 0.5));
    debug_log.push(format!(
        "ship rotation: {:.2?}",
        ship.0.rotation.to_euler(EulerRot::XYZ)
    ));
}

fn rotate_with_keyboard(
    keyboard: Res<ButtonInput<KeyCode>>,
    mut ship: Single<(&mut Transform, &GlobalTransform), With<Ship>>,
) {
    if keyboard.pressed(KeyCode::KeyK) {
        ship.0.rotate_local_z(-0.1);
    }
    if keyboard.pressed(KeyCode::KeyN) {
        ship.0.rotate_local_z(0.1);
    }
}

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
    mut pool: ResMut<ObjectPool<YellowBullet>>,
    mut bullet: Query<(&mut LinearVelocity, &mut Visibility), With<Bullet>>,
    ship: Single<Entity, With<Ship>>,
    mut transform: Query<&mut Transform>,
    nozzle: Single<&GlobalTransform, With<Nozzle>>,
) {
    let bullet_id = pool.get().unwrap();
    let (mut linvel, mut visibility) = bullet.get_mut(bullet_id).unwrap();

    let ship = transform.get_mut(*ship).unwrap();
    let angle = ship.rotation.to_euler(EulerRot::XYZ).2;

    let mut transform = transform.get_mut(bullet_id).unwrap();
    transform.translation = nozzle.translation();
    transform.rotation = Quat::from_rotation_z(angle - PI / 2.);
    *visibility = Visibility::Visible;

    cmd.entity(bullet_id).remove::<RigidBodyDisabled>();
    linvel.0 = Vec2 {
        x: angle.cos() * 2000.,
        y: angle.sin() * 2000.,
    };
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
    cmd.template::<Bullet>(BulletProp::Active(
        image,
        Color::WHITE,
        angle,
        nozzle.translation(),
    ));
}
