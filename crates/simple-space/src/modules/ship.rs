use avian2d::{
    math::{PI, Vector},
    prelude::*,
};
use bevy::prelude::*;
use bevy_prototype_lyon::prelude::*;
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

fn setup(mut cmd: Commands) {
    cmd.insert_resource(FireCooldown(Timer::from_seconds(0.1, TimerMode::Repeating)));

    let point1_length = 100.;
    let point23_length = 50.;

    let point1 = Vec2::new(point1_length, 0.);
    let point2 = Vec2::new(
        120f32.to_radians().cos() * point23_length,
        120f32.to_radians().sin() * point23_length,
    );
    let point3 = Vec2::new(
        240f32.to_radians().cos() * point23_length,
        240f32.to_radians().sin() * point23_length,
    );

    let shape = shapes::Polygon {
        points: vec![point1, point2, point3],
        closed: true,
    };

    cmd.insert_resource(RotateMethod::Cursor);

    cmd.spawn((
        Ship,
        MaxSpeed(1000.),
        RigidBody::Dynamic,
        GravityScale(0.),
        SweptCcd::default(),
        LockedAxes::ROTATION_LOCKED,
    ))
    .with_children(|parent| {
        parent.spawn((
            ShapeBundle {
                path: GeometryBuilder::build_as(&shape),
                ..default()
            },
            Fill::color(Color::WHITE.with_alpha(0.)),
            Stroke::new(Color::WHITE, 3.),
        ));
        parent.spawn(Collider::triangle(point1, point2, point3));
        parent.spawn((Nozzle, Transform::from_xyz(100., 0., 0.)));
    });
}

#[derive(Component)]
pub struct MaxSpeed(f32);

fn ship_strafe(
    keyboard: Res<ButtonInput<KeyCode>>,
    ship: Single<(&mut LinearVelocity, &MaxSpeed), With<Ship>>,
    mut progress: Local<f32>,
    mut last_direction: Local<Vector>, // Track the previous input direction
    time: Res<Time>,
) {
    let (mut linvel, max_speed) = ship.into_inner();
    let mut input_direction = Vector::ZERO;

    // Horizontal input
    match (
        keyboard.pressed(KeyCode::KeyD),
        keyboard.pressed(KeyCode::KeyA),
    ) {
        (true, false) => input_direction.x = 1.0,
        (false, true) => input_direction.x = -1.0,
        _ => {}
    }

    // Vertical input
    match (
        keyboard.pressed(KeyCode::KeyW),
        keyboard.pressed(KeyCode::KeyS),
    ) {
        (true, false) => input_direction.y = 1.0,
        (false, true) => input_direction.y = -1.0,
        _ => {}
    }

    // Reset progress if the input direction changes significantly.
    if input_direction != Vector::ZERO {
        input_direction = input_direction.normalize();
        if input_direction != *last_direction {
            *progress = 0.0;
            *last_direction = input_direction;
        }
    }

    // Ease out expo function
    fn ease_out_expo(t: f32) -> f32 {
        if t >= 1.0 {
            1.0
        } else {
            1.0 - 2.0f32.powf(-10.0 * t)
        }
    }

    // Define a fixed acceleration duration (in seconds)
    let accel_duration = 0.5;

    // Update progress based on whether there's input
    if input_direction != Vector::ZERO {
        *progress += time.delta().as_secs_f32() / 2.;
        let t_normalized = (*progress / accel_duration).clamp(0.0, 1.0);
        let alpha = ease_out_expo(t_normalized);

        // Calculate target velocity based on max speed and input direction.
        let target_velocity = input_direction * max_speed.0;

        // Smoothly interpolate from current velocity to target velocity.
        linvel.0 = linvel.0.lerp(target_velocity, alpha);
    } else {
        // No input: Decelerate smoothly
        linvel.0 = linvel.0.lerp(Vector::ZERO, 0.1);
        // Optionally reset progress more quickly when stopping.
        *progress = (*progress * 0.9).min(0.01);
        *last_direction = Vector::ZERO;
    }

    // Optionally, clamp velocity to max speed if overshooting.
    if linvel.0.length() > max_speed.0 {
        linvel.0 = linvel.0.normalize() * max_speed.0;
    }
}

#[derive(Resource, PartialEq, Eq)]
pub enum RotateMethod {
    Cursor,
    Keyboard,
}

fn switch_key_pressed(keyboard: Res<ButtonInput<KeyCode>>) -> bool {
    keyboard.just_pressed(KeyCode::ShiftRight)
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
    if keyboard.pressed(KeyCode::ArrowRight) {
        ship.0.rotate_local_z(-0.1);
    }
    if keyboard.pressed(KeyCode::ArrowLeft) {
        ship.0.rotate_local_z(0.1);
    }
}

#[derive(Resource)]
struct FireCooldown(Timer);

fn fire_button_pressed(
    keyboard: Res<ButtonInput<KeyCode>>,
    mouse: Res<ButtonInput<MouseButton>>,
) -> bool {
    keyboard.pressed(KeyCode::KeyJ)
        || mouse.pressed(MouseButton::Left)
        || keyboard.pressed(KeyCode::ArrowUp)
        || keyboard.pressed(KeyCode::ArrowDown)
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
    ship: Single<Entity, With<Ship>>,
    mut transform: Query<&mut Transform>,
    nozzle: Single<&GlobalTransform, With<Nozzle>>,
) {
    let ship = transform.get_mut(*ship).unwrap();
    let angle = ship.rotation.to_euler(EulerRot::XYZ).2;
    cmd.template::<Bullet>(BulletProp::Active(angle, nozzle.translation()));
}
