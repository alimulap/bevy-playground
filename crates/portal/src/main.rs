use bevy::prelude::*;
// use bevy::sprite::{Wireframe2dConfig, Wireframe2dPlugin};
use config::{Config, ConfigPlugin, RelPos};

const WINDOW_HEIGHT: f32 = 600.;
const WINDOW_WIDTH: f32 = 900.;

mod config;

fn main() {
    App::new()
        .add_plugins((
            DefaultPlugins.set(WindowPlugin {
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
            }),
            // Wireframe2dPlugin,
            ConfigPlugin,
        ))
        .add_systems(Startup, setup)
        .add_systems(
            Update,
            (
                //toggle_wireframe,
                spawner,
                move_to_center,
                despawner,
                close_on_q,
            ),
        )
        .run();
}

#[derive(Component)]
struct Portal;

#[derive(Component)]
struct Particle;

#[derive(Component)]
struct ParticleMesh(Handle<Mesh>);

#[derive(Component)]
struct DebugText;

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    // mut materials: ResMut<Assets<ColorMaterial>>,
    config: Res<Config>,
) {
    commands.spawn(Camera2d);
    commands.insert_resource(SpawnTimer(Timer::from_seconds(
        config.spawn_interval,
        TimerMode::Repeating,
    )));

    let particle = meshes.add(Circle::new(config.particle_size as f32));

    let portal_pos = match config.portal_pos {
        RelPos::Center => (0., 0.),
        RelPos::TopRight => (WINDOW_WIDTH / 2., WINDOW_HEIGHT / 2.),
        RelPos::TopLeft => (-WINDOW_WIDTH / 2., WINDOW_HEIGHT / 2.),
        RelPos::BottomRight => (WINDOW_WIDTH / 2., -WINDOW_HEIGHT / 2.),
        RelPos::BottomLeft => (-WINDOW_WIDTH / 2., -WINDOW_HEIGHT / 2.),
        RelPos::Custom(x, y) => (x, y),
    };
    commands
        .spawn((
            Portal,
            Transform::from_xyz(portal_pos.0, portal_pos.1, 0.0),
            Visibility::Visible,
        ))
        // .with_children(|parent| {
            // for _ in 0..config.particle_count {
            //     let angle = fastrand::f32() * std::f32::consts::PI * 2.0;
            //     parent.spawn((
            //         Particle,
            //         Mesh2d(particle.clone()),
            //         MeshMaterial2d(materials.add(Color::WHITE)),
            //         Transform::from_xyz(
            //             angle.cos() * config.portal_size,
            //             angle.sin() * config.portal_size,
            //             0.0,
            //         ),
            //     ));
            // }
        // })
        ;

    commands.spawn(ParticleMesh(particle));

    commands.spawn((
        Text::new("Press space to toggle wireframes"),
        TextFont::from_font_size(11.),
        Node {
            position_type: PositionType::Absolute,
            top: Val::Px(12.0),
            left: Val::Px(12.0),
            ..default()
        },
    ));
    commands.spawn((
        DebugText,
        Text::new("Debug"),
        TextFont::from_font_size(11.),
        Node {
            position_type: PositionType::Absolute,
            top: Val::Px(23.0),
            left: Val::Px(12.0),
            ..default()
        },
    ));
}

#[derive(Resource)]
struct SpawnTimer(Timer);

fn spawner(
    mut cmd: Commands,
    time: Res<Time>,
    mut timer: ResMut<SpawnTimer>,
    portal: Single<Entity, With<Portal>>,
    mesh: Single<&ParticleMesh>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    config: Res<Config>,
) {
    if timer.0.tick(time.delta()).just_finished() {
        let angle = fastrand::f32() * std::f32::consts::PI * 2.0;
        let particle = cmd
            .spawn((
                Particle,
                Mesh2d(mesh.0.clone()),
                MeshMaterial2d(materials.add(Color::WHITE)),
                Transform::from_xyz(
                    angle.cos() * config.portal_size,
                    angle.sin() * config.portal_size,
                    0.0,
                ),
            ))
            .id();
        cmd.entity(portal.into_inner()).add_child(particle);
    }
}

fn move_to_center(
    time: Res<Time>,
    portal: Single<&Transform, (With<Portal>, Without<Particle>)>,
    mut particles: Query<&mut Transform, (With<Particle>, Without<Portal>)>,
    config: Res<Config>,
    // mut debug_text: Single<&mut Text, With<DebugText>>,
) {
    for mut particle in particles.iter_mut() {
        let distance = portal.translation - particle.translation;
        let angle = distance.y.atan2(distance.x);
        particle.translation += Vec3::new(
            angle.cos() * config.move_speed * time.delta_secs(),
            angle.sin() * config.move_speed * time.delta_secs(),
            0.0,
        );
    }
}

fn despawner(
    mut commands: Commands,
    portal: Single<&Transform, With<Portal>>,
    particles: Query<(Entity, &Transform), With<Particle>>,
    config: Res<Config>,
) {
    for (particle, transform) in particles.iter() {
        if transform.translation.distance(portal.translation) <= config.particle_size as f32 {
            commands.entity(particle).despawn();
        }
    }
}

// fn toggle_wireframe(
//     mut wireframe_config: ResMut<Wireframe2dConfig>,
//     keyboard: Res<ButtonInput<KeyCode>>,
// ) {
//     if keyboard.just_pressed(KeyCode::Space) {
//         wireframe_config.global = !wireframe_config.global;
//     }
// }

pub fn close_on_q(
    focused_windows: Query<(Entity, &Window)>,
    input: Res<ButtonInput<KeyCode>>,
    mut exit: EventWriter<AppExit>,
) {
    for (_, focus) in focused_windows.iter() {
        if !focus.focused {
            continue;
        }

        if input.just_pressed(KeyCode::KeyQ) {
            exit.send(AppExit::Success);
        }
    }
}
