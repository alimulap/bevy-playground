use std::time::Duration;

use bevy::{color::palettes::css::WHITE, prelude::*};
use bevy_simple_text_input::{
    TextInput, TextInputPlugin, TextInputSettings, TextInputTextFont, TextInputValue,
};

pub struct UIPlugin;

impl Plugin for UIPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(TextInputPlugin)
            .add_systems(Startup, build_ui)
            .add_systems(
                Update,
                (
                    debug_text_system,
                    text_ui_setup,
                    panel_setup,
                    input_ui_setup,
                    input_field_i32_setup,
                    input_field_i32_system,
                ),
            );
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
        BorderColor(WHITE.into()),
        Interaction::None,
    ))
    .with_children(|parent| {
        parent
            .spawn((Panel, PanelTitle::new("Control Panel")))
            .with_children(|parent| {
                parent.spawn(TextUI::new("Kocag"));
                parent.spawn(InputFieldI32UI);
                parent
                    .spawn((Panel, PanelTitle::new("Debug"), DebugPanel::new()))
                    .with_children(|parent| {
                        parent.spawn((DebugPanelText, TextUI::new("")));
                    });
                parent.spawn((InputUI, InputUInitialValue("1".into())));
                parent
                    .spawn((Panel, PanelTitle::new("Kocag")))
                    .with_children(|parent| {
                        parent.spawn(TextUI::new("Kocag"));
                        parent.spawn((InputUI, InputUInitialValue("2".into())));
                    });
            });
    });
}

#[derive(Component)]
#[require(Node, PanelTitle, PanelMaxWidth)]
struct Panel;

#[derive(Component, Default)]
struct PanelTitle(String);

impl PanelTitle {
    pub fn new(title: impl Into<String>) -> Self {
        Self(title.into())
    }
}

#[derive(Component)]
struct PanelMaxWidth(Val);

impl Default for PanelMaxWidth {
    fn default() -> Self {
        Self(Val::Vw(30.))
    }
}

fn panel_setup(
    mut cmd: Commands,
    added_panel: Query<(Entity, &PanelTitle, &PanelMaxWidth), Added<Panel>>,
) {
    for (entity, PanelTitle(title), PanelMaxWidth(width)) in added_panel.iter() {
        let title = cmd
            .spawn((TextUI::new(title), TextLayout {
                justify: JustifyText::Center,
                ..default()
            }))
            .id();
        let separator = cmd
            .spawn((
                Node {
                    border: UiRect::bottom(Val::Px(1.)),
                    margin: UiRect::bottom(Val::Px(5.)),
                    ..default()
                },
                BorderColor(Color::WHITE),
            ))
            .id();
        cmd.entity(entity)
            .insert((
                Panel,
                Node {
                    max_width: *width,
                    flex_direction: FlexDirection::Column,
                    justify_content: JustifyContent::SpaceBetween,
                    border: UiRect::all(Val::Px(1.)),
                    padding: UiRect::all(Val::Px(5.)),
                    row_gap: Val::Px(3.),
                    ..default()
                },
                BackgroundColor(Color::srgb(0.2, 0.2, 0.2)),
                BorderColor(Color::WHITE),
            ))
            .insert_children(0, &[title, separator]);
    }
}

#[derive(Component)]
#[require(Node, Text)]
struct TextUI(String);

impl TextUI {
    pub fn new(content: impl Into<String>) -> Self {
        Self(content.into())
    }
}

fn text_ui_setup(mut cmd: Commands, added_text_ui: Query<(Entity, Ref<TextUI>), Added<TextUI>>) {
    for (entity, content) in added_text_ui.iter() {
        cmd.entity(entity)
            .insert((Text::new(content.0.clone()), TextFont::from_font_size(11.)));
    }
}

#[derive(Component)]
struct InputUI;

#[derive(Component)]
struct InputUInitialValue(String);

fn input_ui_setup(
    mut cmd: Commands,
    added_input_ui: Query<(Entity, &InputUInitialValue), Added<InputUI>>,
) {
    for (entity, value) in added_input_ui.iter() {
        cmd.entity(entity).insert((
            Node {
                border: UiRect::all(Val::Px(1.)),
                ..default()
            },
            BorderColor(Color::WHITE),
            BackgroundColor(Color::srgb(0.3, 0.3, 0.3)),
            TextInput,
            TextInputValue(value.0.clone()),
            TextInputTextFont(TextFont::from_font_size(11.)),
            // TextInputTextColor(TextColor::WHITE),
            TextInputSettings {
                retain_on_submit: true,
                ..Default::default()
            },
            // TextInputInactive(false),
            // FocusPolicy::Block,
        ));
    }
}

#[derive(Component)]
#[require(Node)]
struct InputFieldI32UI;

#[derive(Component)]
struct InputFieldI32UIOldValue(String);

fn input_field_i32_setup(mut cmd: Commands, input_field: Query<Entity, Added<InputFieldI32UI>>) {
    for entity in input_field.iter() {
        cmd.entity(entity).insert((
            InputUI,
            InputUInitialValue("1".into()),
            InputFieldI32UIOldValue("1".into()),
        ));
    }
}

#[allow(clippy::type_complexity)]
fn input_field_i32_system(
    mut input_field: Query<
        (Mut<TextInputValue>, &mut InputFieldI32UIOldValue),
        (With<InputFieldI32UI>, Changed<TextInputValue>),
    >,
) {
    for (mut value, mut old) in input_field.iter_mut() {
        if value.0.parse::<i32>().is_ok() || value.0.is_empty() {
            old.0 = value.0.clone();
        } else {
            value.0 = old.0.clone();
        }
    }
}

#[derive(Component)]
pub struct DebugPanel {
    content: Vec<String>,
    timer: Timer,
    ready_push: bool,
}

#[derive(Component)]
pub struct DebugPanelText;

impl DebugPanel {
    pub fn new() -> Self {
        Self {
            content: vec![String::from(" "); 5],
            timer: Timer::new(Duration::from_secs_f32(0.1), TimerMode::Repeating),
            ready_push: false,
        }
    }

    pub fn push(&mut self, content: impl Into<String>) {
        self.content.remove(0);
        self.content.push(content.into());
    }

    pub fn push_timed(&mut self, content: impl Into<String>) {
        if self.ready_push {
            self.push(content);
            self.ready_push = false;
        }
    }

    fn get_content(&self) -> String {
        self.content.join("\n")
    }

    fn tick(&mut self, time: &Time) {
        if self.timer.tick(time.delta()).just_finished() && !self.ready_push {
            self.ready_push = true;
        }
    }
}

fn debug_text_system(
    time: Res<Time>,
    mut debug_panel: Query<(Entity, Mut<DebugPanel>)>,
    mut debug_text: Query<(Entity, &Parent, &mut Text), With<DebugPanelText>>,
) {
    for (panel_entt, mut debug_panel) in debug_panel.iter_mut() {
        if debug_panel.is_changed() {
            let mut debug_text = debug_text
                .iter_mut()
                .find(|(_, parent, _)| ***parent == panel_entt)
                .unwrap()
                .2;
            debug_panel.tick(&time);
            debug_text.0 = debug_panel.get_content();
        }
    }
}
