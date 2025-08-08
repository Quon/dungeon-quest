use bevy::prelude::*;
use std::slice::Iter;

use crate::config::*;
use crate::materials::font::FontMaterials;
use crate::materials::menu_box::MenuBoxMaterials;
use crate::materials::scenes::ScenesMaterials;
use crate::resources::dictionary::Dictionary;
use crate::resources::language::Language;
use crate::resources::{game_mode::GameMode, profile::Profile};
use crate::scenes::SceneState;
use bevy::color::palettes::css::*;

const RETURN_BUTTON_SIDE: f32 = 50.0;
const FONT_SIZE: f32 = 35.0;

const BOX_TILE_SIZE: f32 = 60.0;
const BOX_WIDTH_TILES: f32 = 10.0;
const BOX_HEIGHT_TILES: f32 = 5.0;

const BOX_ARRAY: [[i8; 10]; 5] = [
    [0, 1, 1, 1, 1, 1, 1, 1, 1, 2],
    [3, 4, 4, 4, 4, 4, 4, 4, 4, 5],
    [3, 4, 4, 4, 4, 4, 4, 4, 4, 5],
    [3, 4, 4, 4, 4, 4, 4, 4, 4, 5],
    [6, 7, 7, 7, 7, 7, 7, 7, 7, 8],
];

#[derive(PartialEq, Component, Clone)]
enum ButtonComponent {
    Return,
    ClassicMode,
    SurvivalMode,
}

impl ButtonComponent {
    pub fn iterator() -> Iter<'static, ButtonComponent> {
        [
            ButtonComponent::Return,
            ButtonComponent::ClassicMode,
            ButtonComponent::SurvivalMode,
        ]
        .iter()
    }
}

pub struct GameModeSelectScenePlugin;

#[derive(Resource)]
struct GameModeSelectSceneData {
    user_interface_root: Entity,
}

impl Plugin for GameModeSelectScenePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(SceneState::GameModeSelectScene), setup);
        app.add_systems(
            Update,
            (button_handle_system, return_button_handle)
                .run_if(in_state(SceneState::GameModeSelectScene)),
        );
        app.add_systems(OnExit(SceneState::GameModeSelectScene), cleanup);
    }
}

fn setup(
    scenes_materials: Res<ScenesMaterials>,
    font_materials: Res<FontMaterials>,
    dictionary: Res<Dictionary>,
    mut commands: Commands,
) {
    // user interface root
    let user_interface_root = commands
        .spawn((
            Node {
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                ..Default::default()
            },
            ImageNode::new(scenes_materials.sub_background_image.clone()),
        ))
        .with_children(|parent| {
            menu_box(parent, &scenes_materials.menu_box_materials);
            select_game_mode_text(parent, &font_materials, &dictionary);
            buttons(parent, &scenes_materials, &font_materials, &dictionary);
        })
        .insert(Name::new("UIRoot"))
        .id();

    commands.insert_resource(GameModeSelectSceneData {
        user_interface_root,
    });

    commands.insert_resource(Profile::new());
}

fn cleanup(mut commands: Commands, game_mode_select_scene_data: Res<GameModeSelectSceneData>) {
    commands
        .entity(game_mode_select_scene_data.user_interface_root)
        .despawn_recursive();
}

fn menu_box(root: &mut ChildSpawnerCommands, menu_box_materials: &MenuBoxMaterials) {
    let start_left = (WINDOW_HEIGHT * RESOLUTION - BOX_TILE_SIZE * BOX_WIDTH_TILES) / 2.0;
    let start_top = (WINDOW_HEIGHT - BOX_TILE_SIZE * BOX_HEIGHT_TILES) / 2.0;

    for (row_index, row) in BOX_ARRAY.iter().enumerate() {
        for (column_index, value) in row.iter().enumerate() {
            let image: Handle<Image> = match value {
                0 => menu_box_materials.top_left.clone(),
                1 => menu_box_materials.top_center.clone(),
                2 => menu_box_materials.top_right.clone(),
                3 => menu_box_materials.mid_left.clone(),
                4 => menu_box_materials.mid_center.clone(),
                5 => menu_box_materials.mid_right.clone(),
                6 => menu_box_materials.bottom_left.clone(),
                7 => menu_box_materials.bottom_center.clone(),
                8 => menu_box_materials.bottom_right.clone(),
                _ => panic!("Unknown resources"),
            };

            root.spawn((
                ImageNode::new(image),
                Node {
                    position_type: PositionType::Absolute,
                    left: Val::Px(start_left + BOX_TILE_SIZE * column_index as f32),
                    top: Val::Px(start_top + BOX_TILE_SIZE * row_index as f32),
                    bottom: Val::Auto,
                    right: Val::Auto,
                    width: Val::Px(BOX_TILE_SIZE),
                    height: Val::Px(BOX_TILE_SIZE),
                    ..Default::default()
                },
            ));
        }
    }
}

fn select_game_mode_text(
    root: &mut ChildSpawnerCommands,
    font_materials: &FontMaterials,
    dictionary: &Dictionary,
) {
    let font = font_materials.get_font(dictionary.get_current_language());
    let glossary = dictionary.get_glossary();

    let left_position = if dictionary.get_current_language() == Language::VI {
        340.0
    } else {
        300.0
    };

    root.spawn((
        Node {
            position_type: PositionType::Absolute,
            left: Val::Px(left_position),
            top: Val::Px(190.0),
            ..Default::default()
        },
        Text::new(glossary.shared_text.select_game_mode),
        TextFont {
            font: font,
            font_size: 50.0,
            ..Default::default()
        },
        TextColor(Color::BLACK),
        TextLayout::new_with_justify(JustifyText::Center),
    ));
}

fn buttons(
    root: &mut ChildSpawnerCommands,
    scenes_materials: &ScenesMaterials,
    font_materials: &FontMaterials,
    dictionary: &Dictionary,
) {
    let font = font_materials.get_font(dictionary.get_current_language());
    let glossary = dictionary.get_glossary();

    for (index, button) in ButtonComponent::iterator().enumerate() {
        match button {
            ButtonComponent::Return => {
                let handle_image = scenes_materials.icon_materials.home_icon_normal.clone();
                root.spawn((
                    Button { ..default() },
                    Node {
                        left: Val::Px(RETURN_BUTTON_SIDE / 2.0),
                        top: Val::Px(RETURN_BUTTON_SIDE / 2.0),
                        right: Val::Auto,
                        bottom: Val::Auto,
                        width: Val::Px(RETURN_BUTTON_SIDE),
                        height: Val::Px(RETURN_BUTTON_SIDE),
                        justify_content: JustifyContent::Center,
                        position_type: PositionType::Absolute,
                        ..Default::default()
                    },
                    ImageNode::new(handle_image),
                ))
                .insert(button.clone());
            }
            _ => {
                root.spawn((
                    Button { ..default() },
                    Node {
                        left: Val::Px((WINDOW_HEIGHT * RESOLUTION - 300.0) / 2.0),
                        top: Val::Px(if index == 1 { 270.0 } else { 330.0 }),
                        right: Val::Auto,
                        bottom: Val::Auto,
                        width: Val::Px(300.0),
                        height: Val::Px(FONT_SIZE),
                        justify_content: JustifyContent::Center,
                        position_type: PositionType::Absolute,
                        ..Default::default()
                    },
                    BackgroundColor(Color::NONE),
                ))
                .with_children(|parent| {
                    parent.spawn((
                        Text::new(if index == 1 {
                            glossary.shared_text.classic_mode.clone()
                        } else {
                            glossary.shared_text.survival_mode.clone()
                        }),
                        TextFont {
                            font: font.clone(),
                            font_size: FONT_SIZE,
                            ..Default::default()
                        },
                        TextColor(Color::from(GRAY)),
                        TextLayout::new_with_justify(JustifyText::Center),
                    ));
                })
                .insert(button.clone());
            }
        }
    }
}

fn button_handle_system(
    mut button_query: Query<
        (&Interaction, &ButtonComponent, &Children),
        (Changed<Interaction>, With<Button>),
    >,
    mut text_query: Query<Entity>,
    mut profile: ResMut<Profile>,
    mut state: ResMut<NextState<SceneState>>,
    mut writer: TextUiWriter,
) {
    for (interaction, button, children) in button_query.iter_mut() {
        let entity = text_query.get(children[0]).unwrap();
        match *interaction {
            Interaction::None => *writer.color(entity, 0) = TextColor::from(GRAY),
            Interaction::Hovered => *writer.color(entity, 0) = TextColor::BLACK,
            Interaction::Pressed => {
                if *button == ButtonComponent::ClassicMode {
                    profile.set_game_mode(GameMode::ClassicMode);
                    state.set(SceneState::HeroSelectScene);
                } else if *button == ButtonComponent::SurvivalMode {
                    profile.set_game_mode(GameMode::SurvivalMode);
                    state.set(SceneState::HeroSelectScene);
                }
            }
        }
    }
}

fn return_button_handle(
    mut button_query: Query<
        (&Interaction, &ButtonComponent, &mut ImageNode),
        (Changed<Interaction>, With<Button>),
    >,
    scenes_materials: Res<ScenesMaterials>,
    mut state: ResMut<NextState<SceneState>>,
) {
    for (interaction, button, mut ui_image) in button_query.iter_mut() {
        if *button == ButtonComponent::Return {
            match *interaction {
                Interaction::None => {
                    ui_image.image = scenes_materials.icon_materials.home_icon_normal.clone()
                }
                Interaction::Hovered => {
                    ui_image.image = scenes_materials.icon_materials.home_icon_hovered.clone()
                }
                Interaction::Pressed => {
                    ui_image.image = scenes_materials.icon_materials.home_icon_clicked.clone();
                    state.set(SceneState::MainMenuScene);
                }
            }
        }
    }
}
