use std::str::FromStr;

use bevy::prelude::*;
use bevy_simple_text_input::TextInputValue;
use playground_ui::{
    DebugLog, DebugPanelText, Header, InputField, InputFieldLabel, InputFieldType,
    InputUISubmitEvent, InputUInitialValue, MaxWidth, Panel, PanelTitle, PlaygroundUIPlugin,
    TextUI,
};

use crate::config::{Config, ConfigChanged, RelPos};

pub struct UIPlugin;

impl Plugin for UIPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(PlaygroundUIPlugin)
            .init_resource::<DebugLog>()
            .add_systems(Startup, build_ui)
            .add_observer(control_panel_system);
    }
}

fn build_ui(mut cmd: Commands, config: Res<Config>) {
    cmd.spawn((
        Node {
            flex_direction: FlexDirection::Column,
            width: Val::Vw(100.),
            height: Val::Vh(100.),
            border: UiRect::axes(Val::Px(3.), Val::Px(3.)),
            padding: UiRect::all(Val::Px(7.)),
            ..default()
        },
        BorderColor(Color::WHITE),
        Interaction::None,
    ))
    .with_children(|parent| {
        parent
            .spawn((
                Panel,
                PanelTitle::new("Control Panel"),
                MaxWidth(Val::Percent(10.)),
            ))
            .with_children(|parent| {
                parent.spawn(Header::new("Portal"));
                parent.spawn((
                    InputField,
                    InputFieldLabel::new("size"),
                    InputUInitialValue(config.portal.size.to_string()),
                    InputFieldType::F32,
                    Name::new("portal:size"),
                ));
                parent.spawn((
                    InputField,
                    InputFieldLabel::new("position"),
                    InputUInitialValue(config.portal.pos.to_string()),
                    InputFieldType::String,
                    MaxWidth(Val::Px(85.)),
                    Name::new("portal:pos"),
                ));
                parent.spawn((
                    InputField,
                    InputFieldLabel::new("edge offset"),
                    InputUInitialValue(config.portal.edge_offset.to_string()),
                    InputFieldType::F32,
                    Name::new("portal:edge_offset"),
                ));
                parent.spawn(Header::new("Particle"));
                parent.spawn((
                    InputField,
                    InputFieldLabel::new("size"),
                    InputUInitialValue(config.particle.size.to_string()),
                    InputFieldType::F32,
                    Name::new("particle:size"),
                ));
                parent.spawn((
                    InputField,
                    InputFieldLabel::new("spawn interval"),
                    InputUInitialValue(config.particle.spawn_interval.to_string()),
                    InputFieldType::F32,
                    Name::new("particle:spawn_interval"),
                ));
                parent.spawn((
                    InputField,
                    InputFieldLabel::new("move speed"),
                    InputUInitialValue(config.particle.move_speed.to_string()),
                    InputFieldType::F32,
                    Name::new("particle:move_speed"),
                ));
                parent.spawn((
                    InputField,
                    InputFieldLabel::new("spiral angle"),
                    InputUInitialValue(config.particle.spiral_offset_angle.to_string()),
                    InputFieldType::F32,
                    Name::new("particle:spiral_offset_angle"),
                ));
                parent.spawn(Header::new("Particle Trail"));
                parent.spawn((
                    InputField,
                    InputFieldLabel::new("spawn interval"),
                    InputUInitialValue(config.particle.trail.spawn_interval.to_string()),
                    InputFieldType::F32,
                    Name::new("particle:trail:spawn_interval"),
                ));
                parent.spawn((
                    InputField,
                    InputFieldLabel::new("timeout"),
                    InputUInitialValue(config.particle.trail.timeout.to_string()),
                    InputFieldType::F32,
                    Name::new("particle:trail:timeout"),
                ));
                parent
                    .spawn((Panel, PanelTitle::new("Debug")))
                    .with_children(|parent| {
                        parent.spawn((DebugPanelText, TextUI::new("")));
                    });
            });
    });
}

fn control_panel_system(
    trigger: Trigger<InputUISubmitEvent>,
    mut cmd: Commands,
    mut input: Query<(&mut TextInputValue, &Name)>,
    mut config: ResMut<Config>,
) {
    let (mut value, name) = input.get_mut(trigger.entity()).unwrap();
    if name.eq(&Name::new("portal:size")) {
        if let Ok(size) = value.0.parse::<f32>() {
            config.portal.size = size;
            cmd.trigger(ConfigChanged::PortalSize);
        }
    } else if name.eq(&Name::new("portal:pos")) {
        if let Ok(pos) = RelPos::from_str(&value.0) {
            config.portal.pos = pos;
            cmd.trigger(ConfigChanged::PortalPos);
        } else {
            warn!("Invalid portal position");
            value.0 = config.portal.pos.to_string();
        }
    } else if name.eq(&Name::new("portal:edge_offset")) {
        if let Ok(edge_offset) = value.0.parse::<f32>() {
            config.portal.edge_offset = edge_offset;
            cmd.trigger(ConfigChanged::PortalEdgeOffset);
        }
    } else if name.eq(&Name::new("particle:size")) {
        if let Ok(size) = value.0.parse::<u32>() {
            config.particle.size = size;
            cmd.trigger(ConfigChanged::ParticleSize);
        }
    } else if name.eq(&Name::new("particle:spawn_interval")) {
        if let Ok(spawn_interval) = value.0.parse::<f32>() {
            config.particle.spawn_interval = spawn_interval;
            cmd.trigger(ConfigChanged::ParticleSpawnInterval);
        }
    } else if name.eq(&Name::new("particle:move_speed")) {
        if let Ok(move_speed) = value.0.parse::<f32>() {
            config.particle.move_speed = move_speed;
            cmd.trigger(ConfigChanged::ParticleMoveSpeed);
        }
    } else if name.eq(&Name::new("particle:spiral_offset_angle")) {
        if let Ok(spiral_offset_angle) = value.0.parse::<f32>() {
            config.particle.spiral_offset_angle = spiral_offset_angle;
            cmd.trigger(ConfigChanged::ParticleSpiralOffsetAngle);
        }
    } else if name.eq(&Name::new("particle:trail:spawn_interval")) {
        if let Ok(spawn_interval) = value.0.parse::<f32>() {
            config.particle.trail.spawn_interval = spawn_interval;
            cmd.trigger(ConfigChanged::ParticleTrailSpawnInterval);
        }
    } else if name.eq(&Name::new("particle:trail:timeout")) {
        if let Ok(timeout) = value.0.parse::<f32>() {
            config.particle.trail.timeout = timeout;
            cmd.trigger(ConfigChanged::ParticleTrailTimeout);
        }
    }
}
