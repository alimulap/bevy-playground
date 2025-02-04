use avian2d::prelude::*;
use bevy::{input::common_conditions::input_just_pressed, prelude::*};
use modules::{BulletPlugin, ShipPlugin};
use ui::UIPlugin;

pub const WINDOW_HEIGHT: f32 = 600.;
pub const WINDOW_WIDTH: f32 = 900.;

mod modules;
mod ui;

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
        .add_plugins(PhysicsPlugins::default())
        // .add_plugins(PhysicsDebugPlugin::default())
        .add_plugins(UIPlugin)
        .add_plugins(ShipPlugin)
        .add_plugins(BulletPlugin)
        .init_resource::<CursorPosition>()
        .add_systems(Startup, setup)
        .add_systems(
            Update,
            (
                track_cursor_position.run_if(on_event::<CursorMoved>),
                close_window.run_if(input_just_pressed(KeyCode::KeyQ)),
            ),
        )
        .run();
}

fn setup(mut cmd: Commands) {
    cmd.spawn((
        Camera2d,
        Projection::from(OrthographicProjection {
            scale: 3.,
            ..OrthographicProjection::default_2d()
        }),
    ));
}

#[derive(Resource, Default)]
pub struct CursorPosition(Vec2);

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
