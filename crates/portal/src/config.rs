use std::{
    env,
    fmt::{self, Display},
    fs,
    str::FromStr,
};

use bevy::prelude::*;
use serde::{
    Deserialize,
    de::{self, MapAccess, Visitor},
};

use crate::{ParticleMesh, ParticleSpawnTimer, Portal, WINDOW_HEIGHT, WINDOW_WIDTH};

pub struct ConfigPlugin;

impl Plugin for ConfigPlugin {
    fn build(&self, app: &mut App) {
        let config_filepath = env::var("CARGO_MANIFEST_DIR").unwrap() + "/config.toml";
        let config =
            toml::from_str::<Config>(&fs::read_to_string(config_filepath).unwrap()).unwrap();

        app.insert_resource(config)
            .add_event::<ConfigChanged>()
            .add_observer(config_sync);
    }
}

#[derive(Resource, Deserialize)]
pub struct Config {
    pub portal: PortalConfig,
    pub particle: ParticleConfig,
}

#[derive(Deserialize)]
pub struct PortalConfig {
    pub size: f32,
    pub pos: RelPos,
    pub edge_offset: f32,
}

#[derive(Deserialize)]
pub struct ParticleConfig {
    pub size: u32,
    pub spawn_interval: f32,
    pub move_speed: f32,
    pub spiral_offset_angle: f32,
    pub trail: TrailConfig,
}

#[derive(Deserialize)]
pub struct TrailConfig {
    pub spawn_interval: f32,
    pub timeout: f32,
}

#[derive(Event)]
pub enum ConfigChanged {
    PortalSize,
    PortalPos,
    PortalEdgeOffset,
    ParticleSize,
    ParticleSpawnInterval,
    ParticleMoveSpeed,
    ParticleSpiralOffsetAngle,
    ParticleTrailSpawnInterval,
    ParticleTrailTimeout,
}

#[derive(Debug, Default)]
pub enum RelPos {
    #[default]
    Center,
    TopRight,
    TopLeft,
    BottomRight,
    BottomLeft,
    Custom(f32, f32),
}

impl FromStr for RelPos {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "center" => Ok(RelPos::Center),
            "topright" => Ok(RelPos::TopRight),
            "topleft" => Ok(RelPos::TopLeft),
            "bottomright" => Ok(RelPos::BottomRight),
            "bottomleft" => Ok(RelPos::BottomLeft),
            _ => Err("invalid variant"),
        }
    }
}

impl Display for RelPos {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            RelPos::Center => write!(f, "center"),
            RelPos::TopRight => write!(f, "topright"),
            RelPos::TopLeft => write!(f, "topleft"),
            RelPos::BottomRight => write!(f, "bottomright"),
            RelPos::BottomLeft => write!(f, "bottomleft"),
            RelPos::Custom(x, y) => write!(f, "custom x: {}, y: {}", x, y),
        }
    }
}

impl<'de> serde::Deserialize<'de> for RelPos {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct RelPosVisitor;

        impl<'de> Visitor<'de> for RelPosVisitor {
            type Value = RelPos;

            fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                formatter.write_str("a string or map representing RelPos")
            }

            fn visit_str<E>(self, value: &str) -> Result<Self::Value, E>
            where
                E: de::Error,
            {
                match value.to_lowercase().as_str() {
                    "center" => Ok(RelPos::Center),
                    "topright" => Ok(RelPos::TopRight),
                    "topleft" => Ok(RelPos::TopLeft),
                    "bottomright" => Ok(RelPos::BottomRight),
                    "bottomleft" => Ok(RelPos::BottomLeft),
                    _ => Err(E::custom("invalid variant")),
                }
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'de>,
            {
                let mut x = None;
                let mut y = None;

                while let Some(key) = map.next_key::<String>()? {
                    match key.to_lowercase().as_str() {
                        "x" => x = Some(map.next_value()?),
                        "y" => y = Some(map.next_value()?),
                        _ => return Err(de::Error::unknown_field(&key, &["x", "y"])),
                    }
                }

                let x = x.ok_or_else(|| de::Error::missing_field("x"))?;
                let y = y.ok_or_else(|| de::Error::missing_field("y"))?;

                Ok(RelPos::Custom(x, y))
            }
        }

        deserializer.deserialize_any(RelPosVisitor)
    }
}

fn config_sync(
    trigger: Trigger<ConfigChanged>,
    config: ResMut<Config>,
    mut particle_spawn_timer: ResMut<ParticleSpawnTimer>,
    mut particle_mesh: ResMut<ParticleMesh>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut portal: Single<&mut Transform, With<Portal>>,
) {
    match *trigger {
        ConfigChanged::ParticleSpawnInterval => {
            particle_spawn_timer.0 =
                Timer::from_seconds(config.particle.spawn_interval, TimerMode::Repeating);
            info!("kocag2");
        }
        ConfigChanged::ParticleSize => {
            particle_mesh.0 = meshes.add(Circle::new(config.particle.size as f32));
        }
        ConfigChanged::PortalPos => {
            let portal_pos = match config.portal.pos {
                RelPos::Center => (0., 0.),
                RelPos::TopRight => (WINDOW_WIDTH / 2., WINDOW_HEIGHT / 2.),
                RelPos::TopLeft => (-WINDOW_WIDTH / 2., WINDOW_HEIGHT / 2.),
                RelPos::BottomRight => (WINDOW_WIDTH / 2., -WINDOW_HEIGHT / 2.),
                RelPos::BottomLeft => (-WINDOW_WIDTH / 2., -WINDOW_HEIGHT / 2.),
                RelPos::Custom(x, y) => (x, y),
            };
            portal.translation = Vec3::new(portal_pos.0, portal_pos.1, 0.0);
            info!("kocag");
        }
        _ => {}
    }
}
