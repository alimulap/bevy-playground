use std::{env, fmt, fs};

use bevy::prelude::*;
use serde::{
    Deserialize,
    de::{self, MapAccess, Visitor},
};

pub struct ConfigPlugin;

impl Plugin for ConfigPlugin {
    fn build(&self, app: &mut App) {
        let config_filepath = env::var("CARGO_MANIFEST_DIR").unwrap() + "/config.toml";
        let config =
            toml::from_str::<Config>(&fs::read_to_string(config_filepath).unwrap()).unwrap();

        app.insert_resource(config);
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
}

#[derive(Deserialize)]
pub struct ParticleConfig {
    pub size: u32,
    #[allow(dead_code)]
    pub count: u32,
    pub spawn_interval: f32,
    pub move_speed: f32,
    pub spiral_offset_angle: f32,
    pub trail: TrailConfig,
}

#[derive(Deserialize)]
pub struct TrailConfig {
    pub spawn_interval: f32,
    pub timeout: f32,
    #[allow(dead_code)]
    pub count: u32,
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
