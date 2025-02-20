use avian2d::prelude::*;
use bevy::{ecs::observer::TriggerTargets, prelude::*};
use bevy_prototype_lyon::prelude::*;

pub struct HealthPlugin;

impl Plugin for HealthPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup)
            .add_systems(Update, (sync_hpbar_position, sync_health_hpbar));
    }
}

#[derive(Component)]
pub struct Health(pub f32);

#[derive(Component, Default)]
pub struct MaxHealth(pub f32);

#[derive(Component)]
#[require(Transform, Visibility)]
pub struct HPBar;

#[derive(Component, Clone)]
pub struct HPBarConfig {
    pub y_offset: f32,
}

#[derive(Component)]
struct HPBarRef(Entity);

#[derive(Component)]
struct HPBarRemaining;

const HP_BAR_WIDTH: f32 = 100.;

fn setup(world: &mut World) {
    world
        .register_component_hooks::<Health>()
        .on_add(|mut world, id, _cid| {
            let HPBarConfig { y_offset } = world
                .entity(id)
                .get_components::<&HPBarConfig>()
                .unwrap_or(&HPBarConfig { y_offset: 0. })
                .to_owned();
            let hpbar_id = world
                .commands()
                .spawn(HPBar)
                .with_children(|parent| {
                    parent.spawn((
                        ShapeBundle {
                            path: GeometryBuilder::build_as(&shapes::Rectangle {
                                extents: Vec2::new(HP_BAR_WIDTH, 5.),
                                ..default()
                            }),
                            transform: Transform::from_xyz(0., y_offset, 0.),
                            ..default()
                        },
                        Fill::color(Color::BLACK),
                        Stroke::new(Color::WHITE, 1.),
                    ));
                    parent.spawn((
                        HPBarRemaining,
                        ShapeBundle {
                            path: GeometryBuilder::build_as(&shapes::Rectangle {
                                extents: Vec2::new(HP_BAR_WIDTH, 5.),
                                ..default()
                            }),
                            transform: Transform::from_xyz(-5., y_offset - 5., 0.),
                            ..default()
                        },
                        Fill::color(Color::WHITE),
                        Stroke::new(Color::BLACK, 1.),
                    ));
                })
                .id();

            world.commands().entity(id).insert(HPBarRef(hpbar_id));

            if world.entity(id).get_components::<&MaxHealth>().is_none() {
                let health = world
                    .entity(id)
                    .get_components::<&Health>()
                    .unwrap()
                    .0
                    .to_owned();
                world.commands().entity(id).insert(MaxHealth(health));
            }
        });
}

fn sync_hpbar_position(
    owner_position: Query<(Ref<Position>, &HPBarRef), (Changed<Position>, With<Health>)>,
    mut hp_bar: Query<&mut Transform, With<HPBar>>,
) {
    for (position, HPBarRef(hpbar_id)) in owner_position.iter() {
        let mut hp_bar_transform = hp_bar.get_mut(*hpbar_id).unwrap();
        hp_bar_transform.translation.x = position.x;
        hp_bar_transform.translation.y = position.y;
    }
}

fn sync_health_hpbar(
    health: Query<(Ref<Health>, &MaxHealth, &HPBarRef), Changed<Health>>,
    hp_bar: Query<Entity, With<HPBar>>,
    mut hp_bar_remaining: Query<(&Parent, &mut Path), With<HPBarRemaining>>,
) {
    for (hp, max_hp, HPBarRef(hpbar_id)) in health.iter() {
        let hp_bar_entity = hp_bar.get(*hpbar_id).unwrap();
        for (parent, mut path) in hp_bar_remaining.iter_mut() {
            if parent.entities()[0] == hp_bar_entity {
                let width = hp.0 / max_hp.0 * HP_BAR_WIDTH;
                // info!("HP: {}, Max HP: {}, Width: {}", hp.0, max_hp.0, width);
                *path = ShapePath::build_as(&shapes::Rectangle {
                    extents: Vec2::new(width, 5.),
                    ..default()
                });
            }
        }
    }
}
