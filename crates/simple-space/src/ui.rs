use bevy::prelude::*;
use playground_ui::{DebugLog, DebugPanelText, Panel, PanelTitle, PlaygroundUIPlugin, TextUI};

pub struct UIPlugin;

impl Plugin for UIPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(PlaygroundUIPlugin)
            .init_resource::<DebugLog>()
            .add_systems(Startup, build_ui);
    }
}

fn build_ui(mut cmd: Commands) {
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
    ))
    .with_children(|parent| {
        parent
            .spawn((Panel, PanelTitle::new("Panel")))
            .with_children(|parent| {
                parent.spawn(TextUI::new("Test text"));
                parent
                    .spawn((Panel, PanelTitle::new("Debug")))
                    .with_children(|parent| {
                        parent.spawn((DebugPanelText, TextUI::new("")));
                    });
            });
    });
}
