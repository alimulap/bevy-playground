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
            .add_systems(Update, (debug_panel_system, input_field_validation_system))
            .add_observer(create_panel)
            .add_observer(create_text_ui)
            .add_observer(create_input_ui)
            .add_observer(create_input_field);
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
                parent.spawn((
                    InputField,
                    InputUInitialValue("1".into()),
                    InputFieldType::F32,
                ));
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

fn create_panel(
    trigger: Trigger<OnAdd, Panel>,
    mut cmd: Commands,
    added_panel: Query<(&PanelTitle, &PanelMaxWidth)>,
) {
    let (PanelTitle(title), PanelMaxWidth(width)) = added_panel.get(trigger.entity()).unwrap();
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
    cmd.entity(trigger.entity())
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

#[derive(Component)]
#[require(Node, Text)]
struct TextUI(String);

impl TextUI {
    pub fn new(content: impl Into<String>) -> Self {
        Self(content.into())
    }
}

fn create_text_ui(trigger: Trigger<OnAdd, TextUI>, mut cmd: Commands, text_ui: Query<&TextUI>) {
    let TextUI(content) = text_ui.get(trigger.entity()).unwrap();
    cmd.entity(trigger.entity())
        .insert((Text::new(content.clone()), TextFont::from_font_size(11.)));
}

#[derive(Component)]
struct InputUI;

#[derive(Component, Default)]
struct InputUInitialValue(String);

fn create_input_ui(
    trigger: Trigger<OnAdd, InputUI>,
    mut cmd: Commands,
    input_ui: Query<&InputUInitialValue>,
) {
    let InputUInitialValue(value) = input_ui.get(trigger.entity()).unwrap();
    cmd.entity(trigger.entity()).insert((
        Node {
            border: UiRect::all(Val::Px(1.)),
            ..default()
        },
        BorderColor(Color::WHITE),
        BackgroundColor(Color::srgb(0.3, 0.3, 0.3)),
        TextInput,
        TextInputValue(value.clone()),
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

#[derive(Component)]
#[require(Node, InputUInitialValue)]
struct InputField;

#[allow(unused)]
#[derive(Component)]
enum InputFieldType {
    String,
    I32,
    F32,
}

#[derive(Component)]
struct InputFieldOldValue(String);

fn create_input_field(
    trigger: Trigger<OnAdd, InputField>,
    mut cmd: Commands,
    init_value: Query<&InputUInitialValue>,
) {
    let init_value = init_value.get(trigger.entity()).unwrap();
    cmd.entity(trigger.entity())
        .insert((InputUI, InputFieldOldValue(init_value.0.clone())));
}

#[allow(clippy::type_complexity)]
fn input_field_validation_system(
    mut input_field: Query<
        (
            Mut<TextInputValue>,
            &mut InputFieldOldValue,
            &InputFieldType,
        ),
        (With<InputField>, Changed<TextInputValue>),
    >,
) {
    for (mut value, mut old, input_type) in input_field.iter_mut() {
        if match input_type {
            InputFieldType::String => true,
            InputFieldType::I32 => value.0.parse::<i32>().is_ok(),
            InputFieldType::F32 => value.0.parse::<f32>().is_ok(),
        } || value.0.is_empty()
        {
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
            timer: Timer::new(Duration::from_secs_f32(0.3), TimerMode::Repeating),
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

fn debug_panel_system(
    time: Res<Time>,
    mut debug_panel: Single<Mut<DebugPanel>>,
    mut debug_text: Single<&mut Text, With<DebugPanelText>>,
) {
    debug_panel.tick(&time);
    if debug_panel.is_changed() {
        debug_text.0 = debug_panel.get_content();
    }
}
