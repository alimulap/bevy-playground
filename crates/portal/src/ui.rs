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
            .add_systems(Update, (debug_text_system, text_ui_setup, panel_setup));
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
            .spawn((Panel, PanelTitle("Kocag".into())))
            .with_children(|parent| {
                parent.spawn((TextUI, TextUIContent("Control Panel".into())));
                parent.spawn((DebugText::new(), TextUI, TextUIContent("Debug".into())));
                parent.spawn(InputUI::from("1"));
                parent
                    .spawn((Panel, PanelTitle("Kocag".into())))
                    .with_children(|parent| {
                        parent.spawn((TextUI, TextUIContent("Kocag".into())));
                        parent.spawn(InputUI::from("2"));
                    });
            });
    });
}

#[derive(Component)]
#[require(PanelTitle, PanelMaxWidth)]
struct Panel;

#[derive(Component, Default)]
struct PanelTitle(String);

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
            .spawn((TextUI, TextUIContent(title.clone()), TextLayout {
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
#[require(Node)]
struct TextUI;

#[derive(Component)]
struct TextUIContent(String);

fn text_ui_setup(mut cmd: Commands, added_text_ui: Query<(Entity, &TextUIContent), Added<TextUI>>) {
    for (entity, content) in added_text_ui.iter() {
        cmd.entity(entity).insert((
            Text::new(content.0.clone()),
            TextFont::from_font_size(11.),
            Node::default(),
        ));
    }
}

#[derive(Component)]
struct InputUI;

impl InputUI {
    fn from(initial_value: impl Into<String>) -> impl Bundle {
        (
            InputUI,
            Node {
                border: UiRect::all(Val::Px(1.)),
                ..default()
            },
            BorderColor(Color::WHITE),
            BackgroundColor(Color::srgb(0.3, 0.3, 0.3)),
            TextInput,
            TextInputTextFont(TextFont::from_font_size(11.)),
            TextInputValue(initial_value.into()),
            TextInputSettings {
                retain_on_submit: true,
                ..Default::default()
            },
        )
    }
}

#[derive(Component)]
pub struct DebugText {
    content: Vec<String>,
    timer: Timer,
    ready_push: bool,
}

impl DebugText {
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

fn debug_text_system(mut debug_text: Query<(&mut Text, Mut<DebugText>)>, time: Res<Time>) {
    for debug_text in debug_text.iter_mut() {
        let (mut text, mut debug_text) = debug_text;
        debug_text.tick(&time);
        if debug_text.is_changed() {
            text.0 = format!("Debug:\n{}", debug_text.get_content());
        }
    }
}
