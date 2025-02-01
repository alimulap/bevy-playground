use std::time::Duration;

use bevy::prelude::*;
use bevy_simple_text_input::{
    TextInput, TextInputInactive, TextInputPlugin, TextInputSettings, TextInputSystem,
    TextInputTextFont, TextInputValidation, TextInputValue,
};

pub struct PlaygroundUIPlugin;

impl Plugin for PlaygroundUIPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(InputUIFocused(None))
            .insert_resource(DebugLog::new())
            .add_event::<InputUISubmitEvent>()
            .add_plugins(TextInputPlugin)
            .add_systems(
                Update,
                (
                    keyboard_handler,
                    debug_panel_system,
                    focus.before(TextInputSystem),
                ),
            )
            .add_observer(create_panel)
            .add_observer(create_text_ui)
            .add_observer(create_header)
            .add_observer(create_input_ui)
            .add_observer(submit_unfocus)
            .add_observer(create_input_field);
    }
}

#[derive(Event)]
pub struct InputUISubmitEvent;

#[derive(Resource)]
pub struct InputUIFocused(Option<Entity>);

fn keyboard_handler(
    mut cmd: Commands,
    key_input: Res<ButtonInput<KeyCode>>,
    focused: Res<InputUIFocused>,
) {
    if key_input.just_pressed(KeyCode::Enter) {
        cmd.trigger_targets(InputUISubmitEvent, focused.0.unwrap());
    }
}

#[derive(Component)]
#[require(Node, PanelTitle)]
pub struct Panel;

#[derive(Component, Default)]
pub struct PanelTitle(String);

impl PanelTitle {
    pub fn new(title: impl Into<String>) -> Self {
        Self(title.into())
    }
}

#[derive(Component, Clone)]
pub struct MaxWidth(pub Val);

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
pub struct TextUI(String);

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
#[require(Node)]
pub struct Header(String);

impl Header {
    pub fn new(content: impl Into<String>) -> Self {
        Self(content.into())
    }
}

fn create_header(trigger: Trigger<OnAdd, Header>, mut cmd: Commands, header: Query<&Header>) {
    let header = header.get(trigger.entity()).unwrap();
    cmd.entity(trigger.entity()).insert((
        TextUI(format!("=== {} ===", header.0)),
        TextLayout::new_with_justify(JustifyText::Center),
    ));
}

#[derive(Component)]
pub struct InputUI;

#[derive(Component, Default, Clone)]
pub struct InputUInitialValue(pub String);

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
            padding: UiRect::left(Val::Px(3.)),
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
    mut focused: ResMut<InputUIFocused>,
) {
    for (interaction_entity, interaction) in &query {
        if *interaction == Interaction::Pressed {
            for (entity, mut inactive, mut background_color) in &mut text_input_query {
                if entity == interaction_entity {
                    inactive.0 = false;
                    *background_color = BackgroundColor(Color::srgb(0.4, 0.4, 0.4));
                    focused.0 = Some(entity);
                } else if !inactive.0 {
                    inactive.0 = true;
                    *background_color = BackgroundColor(Color::srgb(0.3, 0.3, 0.3));
                }
            }
        }
    }
}

fn submit_unfocus(
    trigger: Trigger<InputUISubmitEvent>,
    mut input_ui: Query<(&mut TextInputInactive, &mut BackgroundColor)>,
    mut focused: ResMut<InputUIFocused>,
) {
    if let Ok((mut inactive, mut background_color)) = input_ui.get_mut(trigger.entity()) {
        focused.0 = None;
        inactive.0 = true;
        *background_color = BackgroundColor(Color::srgb(0.3, 0.3, 0.3));
    }
}

#[derive(Component)]
#[require(Node, InputUInitialValue)]
pub struct InputField;

#[allow(unused)]
#[derive(Component)]
pub enum InputFieldType {
    String,
    I32,
    F32,
}

pub type InputFieldLabel = TextUI;

fn create_input_field(
    trigger: Trigger<OnAdd, InputField>,
    mut cmd: Commands,
    init_value: Query<&InputUInitialValue>,
    label: Query<&InputFieldLabel>,
    input_type: Query<&InputFieldType>,
    name: Query<&Name>,
    width: Query<&MaxWidth>,
) {
    let init_value = init_value.get(trigger.entity()).unwrap();
    cmd.entity(trigger.entity()).remove::<InputUInitialValue>();
    let label = label.get(trigger.entity()).unwrap();
    let input_type = input_type.get(trigger.entity()).unwrap();
    let name = name.get(trigger.entity()).unwrap();
    let max_width = width
        .get(trigger.entity())
        .unwrap_or(&MaxWidth(Val::Px(45.)));
    cmd.entity(trigger.entity())
        .insert((Node {
            flex_direction: FlexDirection::Row,
            justify_content: JustifyContent::SpaceBetween,
            ..default()
        },))
        .with_children(|parent| {
            parent.spawn((label.clone(), Node {
                margin: UiRect::right(Val::Px(3.)),
                ..default()
            }));
            parent.spawn((
                InputUI,
                name.clone(),
                init_value.clone(),
                max_width.clone(),
                match input_type {
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
                },
            ));
        });
}

#[derive(Resource)]
pub struct DebugLog {
    content: Vec<String>,
    timer: Timer,
    ready_push: bool,
}

#[derive(Component)]
pub struct DebugPanelText;

impl DebugLog {
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

    #[allow(unused)]
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

impl Default for DebugLog {
    fn default() -> Self {
        Self::new()
    }
}

fn debug_panel_system(
    time: Res<Time>,
    mut debug_log: ResMut<DebugLog>,
    mut debug_text: Single<&mut Text, With<DebugPanelText>>,
) {
    debug_log.tick(&time);
    if debug_log.is_changed() {
        debug_text.0 = debug_log.get_content();
    }
}
