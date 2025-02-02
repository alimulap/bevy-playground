use avian2d::{math::Vector, prelude::*};
use bevy::{input::common_conditions::input_just_pressed, prelude::*};
use playground_ui::{Panel, PanelTitle, PlaygroundUIPlugin, TextUI};

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
        .add_plugins((
            PhysicsPlugins::default(),
            //    PhysicsDebugPlugin::default()
        ))
        .add_plugins(PlaygroundUIPlugin)
        .init_resource::<CursorPosition>()
        .add_systems(Startup, (setup, build_ui))
        .add_systems(
            Update,
            (
                ship_strafe,
                track_cursor_position,
                shoot_bullet,
                switch_rotate_method,
                look_at_cursor.run_if(resource_equals(RotateMethod::Cursor)),
                rotate_with_keyboard.run_if(resource_equals(RotateMethod::Keyboard)),
                close_window.run_if(input_just_pressed(KeyCode::KeyQ)),
            ),
        )
        .run();
}

#[derive(Component)]
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

    cmd.spawn((
        Ship,
        Sprite::from_image(ship_g),
        MaxSpeed(1000.),
        Rotation::radians(0.),
        RigidBody::Kinematic,
        Collider::compound(vec![
            (
                Position::new(Vec2::default()),
                Rotation::default(),
                Collider::triangle(
                    Vector::new(0., 50.),
                    Vector::new(48., -31.),
                    Vector::new(-48., -31.),
                ),
            ),
            (
                Position::new(Vec2::new(32., -31.)),
                Rotation::default(),
                Collider::triangle(
                    Vector::new(17., 0.),
                    Vector::new(-17., 0.),
                    Vector::new(0., -17.),
                ),
            ),
            (
                Position::new(Vec2::new(-32., -31.)),
                Rotation::default(),
                Collider::triangle(
                    Vector::new(17., 0.),
                    Vector::new(-17., 0.),
                    Vector::new(0., -17.),
                ),
            ),
        ]),
        DebugRender::default(),
    ))
    .with_children(|parent| {
        parent.spawn((Nozzle, Transform::from_xyz(0., 50., 0.)));
    });

    cmd.insert_resource(RotateMethod::Cursor);
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

fn switch_rotate_method(
    keyboard: Res<ButtonInput<KeyCode>>,
    mut rotate_method: ResMut<RotateMethod>,
) {
    if keyboard.just_pressed(KeyCode::KeyY) {
        *rotate_method = match *rotate_method {
            RotateMethod::Cursor => RotateMethod::Keyboard,
            RotateMethod::Keyboard => RotateMethod::Cursor,
        }
    }
}

fn look_at_cursor(
    cursor_position: Res<CursorPosition>,
    camera: Single<(&Camera, &GlobalTransform)>,
    mut ship: Single<(&mut Rotation, &GlobalTransform), With<Ship>>,
) {
    let cursor_position = cursor_position.0;
    let ship_on_viewport = camera
        .0
        .world_to_viewport(camera.1, ship.1.translation())
        .unwrap();
    let angle = (ship_on_viewport.y - cursor_position.y)
        .atan2(cursor_position.x - ship_on_viewport.x)
        - std::f32::consts::PI / 2.;
    *ship.0 = ship.0.nlerp(Rotation::radians(angle), 0.25)
}

fn rotate_with_keyboard(
    keyboard: Res<ButtonInput<KeyCode>>,
    mut ship: Single<(&mut Rotation, &GlobalTransform), With<Ship>>,
) {
    let mut angle = ship.0.as_radians();
    if keyboard.pressed(KeyCode::KeyK) {
        angle -= 0.1;
    }
    if keyboard.pressed(KeyCode::KeyN) {
        angle += 0.1;
    }
    *ship.0 = Rotation::radians(angle);
}

#[derive(Component)]
struct Bullet;

fn shoot_bullet(
    mut cmd: Commands,
    keyboard: Res<ButtonInput<KeyCode>>,
    assets: Res<AssetServer>,
    ship_rotaion: Single<&Rotation, With<Ship>>,
    nozzle: Single<&GlobalTransform, With<Nozzle>>,
) {
    if keyboard.just_pressed(KeyCode::KeyJ) {
        let image = assets.load("effect_yellow.png");
        let angle = ship_rotaion.as_radians() + std::f32::consts::PI / 2.;
        cmd.spawn((
            Bullet,
            Sprite::from_image(image),
            RigidBody::Kinematic,
            Collider::rectangle(30., 30.),
            Sensor,
            LinearVelocity(Vec2 {
                x: angle.cos() * 1000.,
                y: angle.sin() * 1000.,
            }),
            Transform::default()
                .with_translation(nozzle.translation())
                .with_scale(Vec3::splat(0.25)),
            DebugRender::default(),
        ));
    }
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
