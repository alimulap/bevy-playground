use bevy::{color::palettes::css::WHITE, prelude::*};
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
                move_spiral_to_center,
                despawner,
                trail_spawner,
                trail_update,
                close_on_q,
            ),
        )
        .run();
}

#[derive(Component)]
struct Portal;

#[derive(Component)]
struct Particle;

#[derive(Resource)]
struct ParticleMesh(Handle<Mesh>);

#[derive(Component)]
struct Trail;

#[derive(Component)]
struct DebugText;
fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    // mut materials: ResMut<Assets<ColorMaterial>>,
    config: Res<Config>,
) {
    commands.spawn(Camera2d);
    commands.insert_resource(ParticleSpawnTimer(Timer::from_seconds(
        config.particle.spawn_interval,
        TimerMode::Repeating,
    )));

    let particle = meshes.add(Circle::new(config.particle.size as f32));

    let portal_pos = match config.portal.pos {
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

    commands.insert_resource(ParticleMesh(particle));

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
struct ParticleSpawnTimer(Timer);

#[derive(Component)]
struct TrailSpawnTimer(Timer);

#[derive(Component)]
struct TrailTimeout(Timer);

fn spawner(
    mut cmd: Commands,
    time: Res<Time>,
    mut timer: ResMut<ParticleSpawnTimer>,
    portal: Single<Entity, With<Portal>>,
    mesh: Res<ParticleMesh>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    config: Res<Config>,
) {
    if timer.0.tick(time.delta()).just_finished() {
        let angle = fastrand::f32() * std::f32::consts::PI * 2.0;
        let distance = (config.portal.size - config.portal.edge_offset)
            + fastrand::f32() * config.portal.edge_offset * 2.;
        let particle = cmd
            .spawn((
                Particle,
                Mesh2d(mesh.0.clone()),
                MeshMaterial2d(materials.add(Color::WHITE)),
                Transform::from_xyz(angle.cos() * distance, angle.sin() * distance, 0.0),
                TrailSpawnTimer(Timer::from_seconds(
                    config.particle.trail.spawn_interval,
                    TimerMode::Repeating,
                )),
            ))
            .id();
        cmd.entity(portal.into_inner()).add_child(particle);
    }
}

fn move_spiral_to_center(
    time: Res<Time>,
    portal: Single<&Transform, (With<Portal>, Without<Particle>)>,
    mut particles: Query<&mut Transform, (With<Particle>, Without<Portal>)>,
    config: Res<Config>,
    // mut debug_text: Single<&mut Text, With<DebugText>>,
) {
    for mut particle in particles.iter_mut() {
        let distance = portal.translation - particle.translation;
        let angle = distance.y.atan2(distance.x) - config.particle.spiral_offset_angle.to_radians();
        particle.translation += Vec3::new(
            angle.cos() * config.particle.move_speed * time.delta_secs(),
            angle.sin() * config.particle.move_speed * time.delta_secs(),
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
        if transform.translation.distance(portal.translation) <= config.particle.size as f32 {
            commands.entity(particle).despawn();
        }
    }
}

fn trail_spawner(
    mut cmd: Commands,
    mut particles: Query<(&Transform, &mut TrailSpawnTimer), With<Particle>>,
    mesh: Res<ParticleMesh>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    time: Res<Time>,
    config: Res<Config>,
) {
    for (particle, mut timer) in particles.iter_mut() {
        if timer.0.tick(time.delta()).just_finished() {
            cmd.spawn((
                Trail,
                Mesh2d(mesh.0.clone()),
                MeshMaterial2d(materials.add(ColorMaterial {
                    color: WHITE.with_alpha(0.5).into(),
                    ..Default::default()
                })),
                Transform::from_translation(particle.translation),
                TrailTimeout(Timer::from_seconds(
                    config.particle.trail.timeout,
                    TimerMode::Once,
                )),
            ));
        }
    }
}

fn trail_update(
    mut cmd: Commands,
    time: Res<Time>,
    mut trails: Query<(Entity, &mut Transform, &mut TrailTimeout), With<Trail>>,
    config: Res<Config>,
) {
    for (trail, mut transform, mut timer) in trails.iter_mut() {
        if timer.0.tick(time.delta()).just_finished() {
            cmd.entity(trail).despawn();
        }
        let scale = timer.0.remaining_secs() / config.particle.trail.timeout;
        transform.scale = Vec3::splat(scale);
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
