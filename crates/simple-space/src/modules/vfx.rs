use std::f32::consts::TAU;

use bevy::prelude::*;
use bevy_rand::prelude::{Entropy, WyRand};
use bevy_vector_shapes::prelude::*;
use rand_core::RngCore;

use super::template::Template;

pub struct VfxPlugin;

impl Plugin for VfxPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, update_vfx)
            .add_systems(Update, draw_vfx);
    }
}

#[derive(Component)]
pub struct Vfx;

pub enum VfxType {
    Explosion,
}

pub struct VfxProp {
    pub vfx_type: VfxType,
    pub position: Vec3,
    pub rng: Entropy<WyRand>,
}

#[derive(Component)]
pub struct VFXDuration(Timer);

#[derive(Component)]
struct Particles(Vec<Particle>);

impl Particles {
    fn new(count: u32, rng: &mut Entropy<WyRand>) -> Self {
        let mut particles = vec![];
        for _ in 0..count {
            let rotation = rng.next_u32() as f32 / u32::MAX as f32 * TAU;
            let angle = rng.next_u32() as f32 / u32::MAX as f32 * TAU;
            let direction = Vec2::new(angle.cos(), angle.sin());
            let speed = rng.next_u32() as f32 / u32::MAX as f32 * 10. + 20.;
            let size = rng.next_u32() as f32 / u32::MAX as f32 * 17. + 8.;
            particles.push(Particle::new(Vec2::ZERO, rotation, direction, speed, size));
        }
        Self(particles)
    }
}

struct Particle {
    position: Vec2,
    rotation: f32,
    direction: Vec2,
    speed: f32,
    size: f32,
}

impl Particle {
    fn new(position: Vec2, rotation: f32, direction: Vec2, speed: f32, size: f32) -> Self {
        Self {
            position,
            rotation,
            direction,
            speed,
            size,
        }
    }
}

impl Template for Vfx {
    type Prop = VfxProp;
    fn construct(mut cmd: EntityCommands<'_>, mut prop: Self::Prop) -> EntityCommands<'_> {
        match prop.vfx_type {
            VfxType::Explosion => {
                cmd.insert((
                    Vfx,
                    Transform::from_translation(prop.position),
                    Particles::new(5, &mut prop.rng),
                    VFXDuration(Timer::from_seconds(
                        prop.rng.next_u32() as f32 / u32::MAX as f32 * 0.1 + 0.2,
                        TimerMode::Once,
                    )),
                ));
            }
        }
        cmd
    }
}

fn update_vfx(
    mut cmd: Commands,
    mut vfx: Query<(Entity, &mut Particles, &mut VFXDuration)>,
    time: Res<Time>,
) {
    for (id, mut particles, mut duration) in vfx.iter_mut() {
        let elapsed = duration.0.tick(time.delta()).elapsed_secs();
        let particle_count = particles.0.len();
        for i in 0..particle_count {
            let Particle {
                position,
                direction,
                speed,
                ..
            } = &mut particles.0[i];
            position.x += direction.x * elapsed * *speed;
            position.y += direction.y * elapsed * *speed;
        }
        if duration.0.finished() {
            cmd.entity(id).despawn_recursive();
        }
    }
}

fn draw_vfx(
    mut painter: ShapePainter,
    mut vfx: Query<(&Transform, &mut Particles, &mut VFXDuration)>,
) {
    for (transform, particles, duration) in vfx.iter_mut() {
        let particle_count = particles.0.len();
        let t = duration.0.elapsed_secs() / duration.0.duration().as_secs_f32();
        let ti = duration.0.remaining_secs() / duration.0.duration().as_secs_f32();
        for i in 0..particle_count {
            let mut points = Vec::with_capacity(3);
            for j in 0..3 {
                let angle = particles.0[i].rotation + (120. * j as f32).to_radians();
                let x = particles.0[i].position.x + particles.0[i].size * t * angle.cos();
                let y = particles.0[i].position.y + particles.0[i].size * t * angle.sin();
                points.push(Vec2::new(x, y));
            }

            painter.hollow = true;
            painter.transform = *transform;
            painter.translate(Vec3::new(
                particles.0[i].position.x,
                particles.0[i].position.y,
                0.,
            ));
            painter.color = Color::WHITE.with_alpha(ti);
            painter.triangle(points[0], points[1], points[2]);
        }
    }
}
