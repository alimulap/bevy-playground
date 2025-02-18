use bevy::{prelude::*, utils::hashbrown::HashSet};
use bevy_rand::{global::GlobalEntropy, prelude::WyRand};
use rand_core::RngCore;

use crate::constant;

use super::{
    block::{Block, BlockProp},
    template::TemplateExt,
};

pub struct AsteroidPlugin;

impl Plugin for AsteroidPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup);
    }
}

fn setup(mut cmd: Commands, rng: GlobalEntropy<WyRand>) {
    let asteroid_blocks = generate_asteroid(60, rng);
    cmd.spawn((Asteroid, Transform::from_xyz(300., 300., 0.)))
        .with_children(|parent| {
            for (x, y) in asteroid_blocks {
                parent.template::<Block>(BlockProp {
                    position: Vec2::new(
                        x as f32 * constant::BLOCK_SIZE,
                        y as f32 * constant::BLOCK_SIZE,
                    ),
                });
            }
        });
}

#[derive(Component)]
#[require(Transform, Visibility)]
pub struct Asteroid;

fn generate_asteroid(num_blocks: usize, mut rng: GlobalEntropy<WyRand>) -> Vec<(i32, i32)> {
    let mut blocks = HashSet::new();
    blocks.insert((0i32, 0i32));
    let mut frontier = vec![(0i32, 0i32)];

    while blocks.len() < num_blocks && !frontier.is_empty() {
        let idx = rng.next_u32() as usize % frontier.len();
        let (x, y) = frontier[idx];
        let mut neighbors = [(x + 1, y), (x - 1, y), (x, y + 1), (x, y - 1)];

        fastrand::shuffle(&mut neighbors);

        let mut new_found = false;

        for &(x, y) in &neighbors {
            if !blocks.contains(&(x, y)) {
                blocks.insert((x, y));
                frontier.push((x, y));
                new_found = true;
                break;
            }
        }

        if !new_found {
            frontier.swap_remove(idx);
        }
    }

    blocks.into_iter().collect()
}
