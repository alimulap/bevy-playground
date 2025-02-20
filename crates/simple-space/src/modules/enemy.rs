use avian2d::{math::TAU, prelude::*};
use bevy::prelude::*;
use bevy_prototype_lyon::prelude::*;

use super::{
    health::{HPBarConfig, Health},
    physics::GameLayer,
    ship::MaxSpeed,
    template::{Template, TemplateExt},
};

pub struct EnemyPlugin;

impl Plugin for EnemyPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup);
    }
}

fn setup(mut cmd: Commands) {
    cmd.template::<Enemy>(());
}

#[derive(Component)]
#[require(Transform, Visibility)]
pub struct Enemy;

impl Template for Enemy {
    type Prop = ();
    fn construct(mut cmd: EntityCommands<'_>, _: Self::Prop) -> EntityCommands<'_> {
        cmd.insert((
            Enemy,
            Health(100.),
            MaxSpeed(1000.),
            RigidBody::Dynamic,
            GravityScale(0.),
            LockedAxes::ROTATION_LOCKED,
            Transform::from_xyz(300., -300., 0.),
            HPBarConfig { y_offset: 75. },
            Collider::regular_polygon(60., 6),
            CollisionLayers::new(
                GameLayer::Enemy,
                [
                    GameLayer::Default,
                    GameLayer::Block,
                    GameLayer::Bullet,
                    GameLayer::Player,
                ],
            ),
        ))
        .with_children(|parent| {
            parent.spawn((
                ShapeBundle {
                    path: GeometryBuilder::build_as(&shapes::RegularPolygon {
                        sides: 6,
                        feature: shapes::RegularPolygonFeature::Radius(60.),
                        ..Default::default()
                    }),
                    transform: Transform::from_rotation(Quat::from_rotation_z(TAU / 12.)),
                    ..Default::default()
                },
                Fill::color(Color::WHITE.with_alpha(0.)),
                Stroke::new(Color::WHITE, 3.),
            ));
        });
        cmd
    }
}
