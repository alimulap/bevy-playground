use avian2d::{
    math::{PI, Vector},
    prelude::*,
};
use bevy::{input::common_conditions::input_just_pressed, prelude::*};
use playground_ui::{DebugLog, DebugPanelText, Panel, PanelTitle, PlaygroundUIPlugin, TextUI};

pub const WINDOW_HEIGHT: f32 = 600.;
pub const WINDOW_WIDTH: f32 = 900.;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                resolution: (WINDOW_WIDTH, WINDOW_HEIGHT).into(),
                title: "Bevy game".to_string(),
                canvas: Some("#bevy".to_owned()),
                fit_canvas_to_parent: true,
                // Tells wasm not to override default event handling, like F5 and Ctrl+R
                prevent_default_event_handling: false,
                ..default()
            }),
            ..default()
        }))
        .add_plugins((PhysicsPlugins::default(), PhysicsDebugPlugin::default()))
        .add_plugins(PlaygroundUIPlugin)
        .init_resource::<CursorPosition>()
        .init_resource::<DebugLog>()
        .add_systems(Startup, (setup, build_ui))
        .add_systems(
            Update,
            (
                ship_strafe,
                track_cursor_position,
                fire_tick,
                shoot_bullet.run_if(fire_button_pressed.and(can_fire)),
                switch_rotate_method.run_if(switch_key_pressed),
                look_at_cursor.run_if(resource_equals(RotateMethod::Cursor)),
                rotate_with_keyboard.run_if(resource_equals(RotateMethod::Keyboard)),
                close_window.run_if(input_just_pressed(KeyCode::KeyQ)),
            ),
        )
        .run();
}

#[derive(Component)]
#[require(Transform, Visibility)]
struct Ship;

#[derive(Component)]
struct Nozzle;

fn setup(mut cmd: Commands, assets: Res<AssetServer>) {
    cmd.spawn((
        Camera2d,
        Projection::from(OrthographicProjection {
            scale: 3.,
            ..OrthographicProjection::default_2d()
        }),
    ));

    let ship_g = assets.load("ship_G.png");

    cmd.insert_resource(RotateMethod::Cursor);
    cmd.insert_resource(FireCooldown(Timer::from_seconds(0.1, TimerMode::Repeating)));

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
struct MaxSpeed(f32);

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
enum RotateMethod {
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
    ship.0.rotation = Quat::from_rotation_z(
        Rotation::radians(ship.0.rotation.to_euler(EulerRot::XYZ).2)
            .nlerp(Rotation::radians(angle), 0.25)
            .as_radians(),
    );
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

#[derive(Component)]
struct Bullet;

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
        Sprite::from_image(image),
        RigidBody::Kinematic,
        Collider::rectangle(30., 30.),
        Sensor,
        LinearVelocity(Vec2 {
            x: angle.cos() * 2000.,
            y: angle.sin() * 2000.,
        }),
        Transform::default()
            .with_translation(nozzle.translation())
            .with_scale(Vec3::splat(0.25))
            .with_rotation(Quat::from_rotation_z(angle + PI / 2.)),
        DebugRender::default(),
    ));
}

fn build_ui(mut cmd: Commands) {
    cmd.spawn((
        Node {
            flex_direction: FlexDirection::Column,
            width: Val::Vw(100.),
            height: Val::Vh(100.),
            border: UiRect::axes(Val::Px(3.), Val::Px(3.)),
            padding: UiRect::all(Val::Px(7.)),
            ..default()
        },
        BorderColor(Color::WHITE),
    ))
    .with_children(|parent| {
        parent
            .spawn((Panel, PanelTitle::new("Panel")))
            .with_children(|parent| {
                parent.spawn(TextUI::new("Test text"));
                parent
                    .spawn((Panel, PanelTitle::new("Debug")))
                    .with_children(|parent| {
                        parent.spawn((DebugPanelText, TextUI::new("")));
                    });
            });
    });
}

#[derive(Resource, Default)]
struct CursorPosition(Vec2);

fn track_cursor_position(
    mut cursor_moved_events: EventReader<CursorMoved>,
    mut cursor_position: ResMut<CursorPosition>,
) {
    for event in cursor_moved_events.read() {
        cursor_position.0 = event.position;
    }
}

fn close_window(focused_windows: Query<(Entity, &Window)>, mut exit: EventWriter<AppExit>) {
    for (_, focus) in focused_windows.iter() {
        if focus.focused {
            exit.send(AppExit::Success);
        }
    }
}
