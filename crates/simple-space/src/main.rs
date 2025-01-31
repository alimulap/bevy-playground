use bevy::{input::common_conditions::input_just_pressed, prelude::*};

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
        .init_resource::<CursorPosition>()
        .add_systems(Startup, setup)
        .add_systems(
            Update,
            (
                ship_strafe,
                track_cursor_position,
                look_at_cursor,
                close_window.run_if(input_just_pressed(KeyCode::KeyQ)),
            ),
        )
        .run();
}

#[derive(Component)]
struct Ship;

fn setup(mut cmd: Commands, assets: Res<AssetServer>) {
    cmd.spawn(Camera2d);

    let ship_g = assets.load("ship_G.png");

    cmd.spawn((Ship, Sprite::from_image(ship_g)));
}

fn ship_strafe(mut ship: Single<&mut Transform, With<Ship>>, keyboard: Res<ButtonInput<KeyCode>>) {
    let mut direction = Vec3::ZERO;
    match (
        keyboard.pressed(KeyCode::ArrowRight) || keyboard.pressed(KeyCode::KeyD),
        keyboard.pressed(KeyCode::ArrowLeft) || keyboard.pressed(KeyCode::KeyA),
    ) {
        (true, false) => direction.x = 1.0,
        (false, true) => direction.x = -1.0,
        _ => (),
    }
    match (
        keyboard.pressed(KeyCode::ArrowUp) || keyboard.pressed(KeyCode::KeyW),
        keyboard.pressed(KeyCode::ArrowDown) || keyboard.pressed(KeyCode::KeyS),
    ) {
        (true, false) => direction.y = 1.0,
        (false, true) => direction.y = -1.0,
        _ => (),
    }

    ship.translation += direction * 4.0;
}

fn look_at_cursor(
    cursor_position: Res<CursorPosition>,
    camera: Single<(&Camera, &GlobalTransform)>,
    mut ship: Single<(&mut Transform, &GlobalTransform), With<Ship>>,
) {
    let cursor_position = cursor_position.0;
    let ship_on_viewport = camera
        .0
        .world_to_viewport(camera.1, ship.1.translation())
        .unwrap();
    let angle = (ship_on_viewport.y - cursor_position.y)
        .atan2(cursor_position.x - ship_on_viewport.x)
        - std::f32::consts::PI / 2.;
    ship.0.rotation = Quat::from_rotation_z(angle);
    // println!("cursor_position: {:?}", cursor_position);
    // println!("ship_on_viewport: {:?}", ship_on_viewport);
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
