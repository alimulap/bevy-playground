use std::time::Duration;

use bevy::prelude::*;
use bevy_simple_text_input::{
    TextInput, TextInputInactive, TextInputPlugin, TextInputSettings, TextInputSystem,
    TextInputTextFont, TextInputValidation, TextInputValue,
};

pub struct UIPlugin;

impl Plugin for UIPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(TextInputPlugin)
            .add_systems(Startup, build_ui)
            .add_systems(
                Update,
                (
                    debug_panel_system,
                    // input_field_validation_system,
                    focus.before(TextInputSystem),
                ),
            )
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
        BorderColor(Color::WHITE),
        Interaction::None,
    ))
    .with_children(|parent| {
        parent
            .spawn((Panel, PanelTitle::new("Control Panel")))
            .with_children(|parent| {
                parent.spawn(TextUI::new("Portal"));
                parent.spawn((
                    InputField,
                    InputFieldLabel::new("size"),
                    InputUInitialValue("1".into()),
                    InputFieldType::F32,
                ));
                parent.spawn((
                    InputField,
                    InputFieldLabel::new("edge offset"),
                    InputUInitialValue("1".into()),
                    InputFieldType::F32,
                ));
                parent.spawn(TextUI::new("Particle"));
                parent.spawn((
                    InputField,
                    InputFieldLabel::new("size"),
                    InputUInitialValue("1".into()),
                    InputFieldType::F32,
                ));
                parent.spawn((
                    InputField,
                    InputFieldLabel::new("spawn interval"),
                    InputUInitialValue("1".into()),
                    InputFieldType::F32,
                ));
                parent.spawn((
                    InputField,
                    InputFieldLabel::new("move speed"),
                    InputUInitialValue("1".into()),
                    InputFieldType::F32,
                ));
                parent.spawn((
                    InputField,
                    InputFieldLabel::new("spiral angle"),
                    InputUInitialValue("1".into()),
                    InputFieldType::F32,
                ));
                parent.spawn(TextUI::new("Particle Trail"));
                parent.spawn((
                    InputField,
                    InputFieldLabel::new("spawn interval"),
                    InputUInitialValue("1".into()),
                    InputFieldType::F32,
                ));
                parent.spawn((
                    InputField,
                    InputFieldLabel::new("timeout"),
                    InputUInitialValue("1".into()),
                    InputFieldType::F32,
                ));
                parent
                    .spawn((Panel, PanelTitle::new("Debug"), DebugPanel::new()))
                    .with_children(|parent| {
                        parent.spawn((DebugPanelText, TextUI::new("")));
                    });
                parent.spawn((InputUI, InputUInitialValue("1".into())));
            });
    });
}

#[derive(Component)]
#[require(Node, PanelTitle)]
struct Panel;

#[derive(Component, Default)]
struct PanelTitle(String);

impl PanelTitle {
    pub fn new(title: impl Into<String>) -> Self {
        Self(title.into())
    }
}

#[derive(Component)]
struct MaxWidth(Val);

fn create_panel(
    trigger: Trigger<OnAdd, Panel>,
    mut cmd: Commands,
    title: Query<&PanelTitle>,
    max_width: Query<&MaxWidth>,
) {
    let PanelTitle(title) = title.get(trigger.entity()).unwrap();
    let MaxWidth(width) = max_width
        .get(trigger.entity())
        .unwrap_or(&MaxWidth(Val::Vw(30.)));
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

#[derive(Component, Clone)]
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

#[derive(Component, Default, Clone)]
struct InputUInitialValue(String);

fn create_input_ui(
    trigger: Trigger<OnAdd, InputUI>,
    mut cmd: Commands,
    // input_value: Query<&Parent, With<InputUI>>,
    init_value: Query<&InputUInitialValue>,
    max_width: Query<&MaxWidth>,
) {
    let InputUInitialValue(value) = init_value.get(trigger.entity()).unwrap();
    let MaxWidth(width) = max_width
        .get(trigger.entity())
        .unwrap_or(&MaxWidth(Val::Px(45.)));
    cmd.entity(trigger.entity()).insert((
        Node {
            width: *width,
            max_width: *width,
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
        TextInputInactive(true),
        // FocusPolicy::Block,
    ));
}

fn focus(
    query: Query<(Entity, &Interaction), Changed<Interaction>>,
    mut text_input_query: Query<(Entity, &mut TextInputInactive, &mut BackgroundColor)>,
) {
    for (interaction_entity, interaction) in &query {
        if *interaction == Interaction::Pressed {
            for (entity, mut inactive, mut background_color) in &mut text_input_query {
                if entity == interaction_entity {
                    inactive.0 = false;
                    *background_color = BackgroundColor(Color::srgb(0.4, 0.4, 0.4));
                } else {
                    inactive.0 = true;
                    *background_color = BackgroundColor(Color::srgb(0.3, 0.3, 0.3));
                }
            }
        }
    }
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

type InputFieldLabel = TextUI;

fn create_input_field(
    trigger: Trigger<OnAdd, InputField>,
    mut cmd: Commands,
    init_value: Query<&InputUInitialValue>,
    label: Query<&InputFieldLabel>,
    input_type: Query<&InputFieldType>,
) {
    let init_value = init_value.get(trigger.entity()).unwrap();
    cmd.entity(trigger.entity()).remove::<InputUInitialValue>();
    let label = label.get(trigger.entity()).unwrap();
    let input_type = input_type.get(trigger.entity()).unwrap();
    cmd.entity(trigger.entity())
        .insert((Node {
            flex_direction: FlexDirection::Row,
            ..default()
        },))
        .with_children(|parent| {
            parent.spawn((label.clone(), Node {
                margin: UiRect::right(Val::Px(3.)),
                ..default()
            }));
            parent.spawn((InputUI, init_value.clone(), match input_type {
                InputFieldType::String => TextInputValidation(Box::new(|_, _, _| true)),
                InputFieldType::I32 => TextInputValidation(Box::new(|text, i, str| {
                    let mut text = text.clone();
                    text.insert_str(i, str);
                    text.parse::<i32>().is_ok()
                })),
                InputFieldType::F32 => TextInputValidation(Box::new(|text, i, str| {
                    let mut text = text.clone();
                    text.insert_str(i, str);
                    text.parse::<f32>().is_ok()
                })),
            }));
        });
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
