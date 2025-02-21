use avian2d::{math::PI, prelude::*};
use bevy::prelude::*;
use bevy_prototype_lyon::prelude::*;
use bevy_rand::{global::GlobalEntropy, prelude::WyRand, traits::ForkableRng};
use playground_ui::DebugLog;

use super::{
    health::Health,
    template::{Template, TemplateExt},
    vfx::{Vfx, VfxProp, VfxType},
};

pub struct BulletPlugin;

impl Plugin for BulletPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, apply_bullet_damage);
    }
}

#[derive(Component, Clone)]
#[require(Transform, Visibility)]
pub struct Bullet;

#[derive(Component)]
pub enum BulletType {
    Standard,
}

pub struct BulletProp {
    pub rotation: f32,
    pub position: Vec3,
    pub bullet_type: BulletType,
    pub layers: CollisionLayers,
}

impl Template for Bullet {
    type Prop = BulletProp;
    fn construct(mut cmd: EntityCommands<'_>, prop: Self::Prop) -> EntityCommands<'_> {
        let point1 = Vec2::new(0., 13.);
        let point2 = Vec2::new(
            210f32.to_radians().cos() * 13.,
            210f32.to_radians().sin() * 13.,
        );
        let point3 = Vec2::new(
            330f32.to_radians().cos() * 13.,
            330f32.to_radians().sin() * 13.,
        );

        let BulletProp {
            rotation,
            position,
            bullet_type,
            layers,
        } = prop;

        cmd.insert((
            Bullet,
            bullet_type,
            RigidBody::Kinematic,
            LinearVelocity(Vec2 {
                x: rotation.cos() * 2000.,
                y: rotation.sin() * 2000.,
            }),
            Transform::default()
                .with_translation(position)
                .with_rotation(Quat::from_rotation_z(rotation - PI / 2.)),
            Collider::regular_polygon(17., 3),
            Sensor,
            layers,
            CollidingEntities::default(),
            // CollisionLayers::new( GameLayer::Bullet,
            //     [GameLayer::Default, GameLayer::Block, GameLayer::Enemy],
            // ),
        ))
        .with_children(|parent| {
            parent.spawn((
                ShapeBundle {
                    path: GeometryBuilder::build_as(&shapes::Polygon {
                        points: vec![point3, point1, point2],
                        closed: false,
                    }),
                    ..default()
                },
                Fill::color(Color::WHITE.with_alpha(0.)),
                Stroke::new(Color::WHITE, 3.),
            ));
        });
        cmd
    }
}

fn apply_bullet_damage(
    mut cmd: Commands,
    bullets: Query<
        (Entity, Ref<CollidingEntities>, &Transform),
        (With<Bullet>, Changed<CollidingEntities>),
    >,
    mut health: Query<&mut Health>,
    mut debug_log: ResMut<DebugLog>,
    mut rng: GlobalEntropy<WyRand>,
) {
    for (id, entities, transform) in bullets.iter() {
        let mut should_despawn = false;
        for entity in entities.iter() {
            debug_log.push(format!("Bullet collided with entity {:?}", entity));
            if let Ok(mut health) = health.get_mut(*entity) {
                debug_log.push(format!("Entity has health {:?}", health.0));
                health.0 -= 10.;
                should_despawn = true;
                cmd.template::<Vfx>(VfxProp {
                    vfx_type: VfxType::Explosion,
                    position: transform.translation,
                    rng: rng.fork_rng(),
                });
            }
        }
        if should_despawn {
            cmd.entity(id).despawn_recursive();
        }
    }
}
