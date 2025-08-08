use bevy::prelude::*;
use std::slice::Iter;

use crate::config::*;
use crate::materials::font::FontMaterials;
use crate::materials::menu_box::MenuBoxMaterials;
use crate::materials::scenes::ScenesMaterials;
use crate::resources::dictionary::Dictionary;
use crate::resources::language::Language;
use crate::resources::setting::Setting;
use crate::scenes::SceneState;

const RETURN_BUTTON_SIZE: f32 = 50.0;
const NORMAL_BUTTON_SIZE: f32 = 32.0;

const MENU_BOX_TILE_SIZE: f32 = 60.0;

const MENU_BOX_WIDTH_TILES: f32 = 8.0;
const MENU_BOX_HEIGHT_TILES: f32 = 6.0;

const MENU_BOX_ARRAY: [[i8; 8]; 6] = [
    [0, 1, 1, 1, 1, 1, 1, 2],
    [3, 4, 4, 4, 4, 4, 4, 5],
    [3, 4, 4, 4, 4, 4, 4, 5],
    [3, 4, 4, 4, 4, 4, 4, 5],
    [3, 4, 4, 4, 4, 4, 4, 5],
    [6, 7, 7, 7, 7, 7, 7, 8],
];

const SELECTED_FLAG_COLOR: Srgba = Srgba {
    red: (160.0 / 255.0),
    green: (170.0 / 255.0),
    blue: (170.0 / 255.0),
    alpha: 1.0,
};

const NORMAL_FLAG_COLOR: Srgba = Srgba {
    red: 1.0,
    green: 1.0,
    blue: 1.0,
    alpha: 1.0,
};

#[derive(Component, Clone)]
enum PairButtonComponent {
    Vietnamese,
    English,
}

impl PairButtonComponent {
    pub fn iterator() -> Iter<'static, PairButtonComponent> {
        [
            PairButtonComponent::Vietnamese,
            PairButtonComponent::English,
        ]
        .iter()
    }
}

#[derive(Component, Copy, Clone)]
enum ButtonComponent {
    Return,
    EnableSound,
    EnableMusic,
}

impl ButtonComponent {
    pub fn iterator() -> Iter<'static, ButtonComponent> {
        [
            ButtonComponent::Return,
            ButtonComponent::EnableSound,
            ButtonComponent::EnableMusic,
        ]
        .iter()
    }
}

#[derive(Component, Clone)]
enum TextComponent {
    Options,
    EnableSound,
    EnableMusic,
    Language,
}

impl TextComponent {
    pub fn iterator() -> Iter<'static, TextComponent> {
        [
            TextComponent::Options,
            TextComponent::EnableSound,
            TextComponent::EnableMusic,
            TextComponent::Language,
        ]
        .iter()
    }
}

pub struct OptionsScenePlugin;

#[derive(Resource)]
struct OptionsSceneData {
    user_interface_root: Entity,
}

impl Plugin for OptionsScenePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(SceneState::OptionsScene), setup);
        app.add_systems(
            Update,
            (
                button_handle_system,
                pair_button_handle_system,
                text_handle_system,
            )
                .run_if(in_state(SceneState::OptionsScene)),
        );
        app.add_systems(OnExit(SceneState::OptionsScene), cleanup);
    }
}

fn setup(
    mut commands: Commands,
    font_materials: Res<FontMaterials>,
    scenes_materials: Res<ScenesMaterials>,
    setting: Res<Setting>,
    dictionary: Res<Dictionary>,
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
            texts(parent, &font_materials, &dictionary);
            buttons(parent, &setting, &scenes_materials);
            pair_buttons(parent, &setting, &scenes_materials);
        })
        .id();
    commands.insert_resource(OptionsSceneData {
        user_interface_root,
    });
}

fn cleanup(
    mut commands: Commands,
    options_scene_data: Res<OptionsSceneData>,
    setting: Res<Setting>,
) {
    setting.store();
    commands
        .entity(options_scene_data.user_interface_root)
        .despawn();
}

fn menu_box(root: &mut ChildSpawnerCommands, menu_box_materials: &MenuBoxMaterials) {
    let start_left = (WINDOW_HEIGHT * RESOLUTION - MENU_BOX_TILE_SIZE * MENU_BOX_WIDTH_TILES) / 2.0;

    let start_top = (WINDOW_HEIGHT - MENU_BOX_TILE_SIZE * MENU_BOX_HEIGHT_TILES) / 2.0;
    root.spawn(Node {
        ..Default::default()
    })
    .with_children(|parent| {
        for (row_index, row) in MENU_BOX_ARRAY.iter().enumerate() {
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

                parent.spawn((
                    ImageNode::new(image),
                    Node {
                        position_type: PositionType::Absolute,
                        left: Val::Px(start_left + MENU_BOX_TILE_SIZE * column_index as f32),
                        top: Val::Px(start_top + MENU_BOX_TILE_SIZE * row_index as f32),
                        bottom: Val::Auto,
                        right: Val::Auto,
                        width: Val::Px(MENU_BOX_TILE_SIZE),
                        height: Val::Px(MENU_BOX_TILE_SIZE),
                        ..Default::default()
                    },
                ));
            }
        }
    })
    .insert(Name::new("MenuBox"));
}

fn texts(root: &mut ChildSpawnerCommands, font_materials: &FontMaterials, dictionary: &Dictionary) {
    let font = font_materials.get_font(dictionary.get_current_language());
    let glossary = dictionary.get_glossary();

    let position_of_texts: [[f32; 2]; 4] = [
        [440.0, 160.0],
        [320.0, 230.0],
        [320.0, 290.0],
        [320.0, 350.0],
    ];

    for (index, prevalue) in TextComponent::iterator().enumerate() {
        let value: String = match index {
            0 => glossary.options_scene_text.options.clone(),
            1 => glossary.options_scene_text.enable_music.clone(),
            2 => glossary.options_scene_text.enable_sound.clone(),
            3 => glossary.options_scene_text.language.clone(),
            _ => panic!("Unknown text"),
        };

        let component_name = match index {
            0 => "OptionsText",
            1 => "EnableMusicText",
            2 => "EnableSoundText",
            3 => "LanguageText",
            _ => "Unknown text",
        };

        let font_size: f32 = match index {
            0 => 50.0,
            _ => 35.0,
        };

        root.spawn((
            Node {
                position_type: PositionType::Absolute,
                left: Val::Px(position_of_texts[index][0]),
                top: Val::Px(position_of_texts[index][1]),
                ..Default::default()
            },
            Text::new(value),
            TextFont {
                font: font.clone(),
                font_size,
                ..Default::default()
            },
            TextColor(Color::BLACK),
            TextLayout::new_with_justify(JustifyText::Center),
        ))
        .insert(Name::new(component_name))
        .insert(prevalue.clone());
    }
}

fn buttons(root: &mut ChildSpawnerCommands, setting: &Setting, scenes_materials: &ScenesMaterials) {
    let positions: [UiRect; 3] = [
        UiRect {
            left: Val::Px(RETURN_BUTTON_SIZE / 2.0),
            top: Val::Px(RETURN_BUTTON_SIZE / 2.0),
            right: Val::Auto,
            bottom: Val::Auto,
        },
        UiRect {
            left: Val::Px(610.0),
            top: Val::Px(230.0),
            right: Val::Auto,
            bottom: Val::Auto,
        },
        UiRect {
            left: Val::Px(610.0),
            top: Val::Px(290.0),
            right: Val::Auto,
            bottom: Val::Auto,
        },
    ];

    for (index, button) in ButtonComponent::iterator().enumerate() {
        let handle_image = match button {
            ButtonComponent::Return => scenes_materials.icon_materials.home_icon_normal.clone(),
            ButtonComponent::EnableSound => {
                if setting.get_enable_sound() {
                    scenes_materials.icon_materials.sound_icon_on.clone()
                } else {
                    scenes_materials.icon_materials.sound_icon_off.clone()
                }
            }
            ButtonComponent::EnableMusic => {
                if setting.get_enable_music() {
                    scenes_materials.icon_materials.music_icon_on.clone()
                } else {
                    scenes_materials.icon_materials.music_icon_off.clone()
                }
            }
        };

        let component_name = match button {
            ButtonComponent::Return => "Return",
            ButtonComponent::EnableSound => "EnableSound",
            ButtonComponent::EnableMusic => "EnableMusic",
        };

        let (width, height) = match button {
            ButtonComponent::Return => (Val::Px(RETURN_BUTTON_SIZE), Val::Px(RETURN_BUTTON_SIZE)),
            _ => (Val::Px(NORMAL_BUTTON_SIZE), Val::Px(NORMAL_BUTTON_SIZE)),
        };

        let rect = positions[index];
        root.spawn((
            Button { ..default() },
            Node {
                left: rect.left,
                right: rect.right,
                top: rect.top,
                bottom: rect.bottom,
                width,
                height,
                justify_content: JustifyContent::Center,
                position_type: PositionType::Absolute,
                ..Default::default()
            },
            ImageNode::new(handle_image),
        ))
        .insert(Name::new(component_name))
        .insert(*button);
    }
}

fn pair_buttons(root: &mut ChildSpawnerCommands, setting: &Setting, scenes_materials: &ScenesMaterials) {
    let positions: [UiRect; 2] = [
        UiRect {
            left: Val::Px(570.0),
            top: Val::Px(350.0),
            right: Val::Auto,
            bottom: Val::Auto,
        },
        UiRect {
            left: Val::Px(620.0),
            top: Val::Px(350.0),
            right: Val::Auto,
            bottom: Val::Auto,
        },
    ];

    for (index, pair_button) in PairButtonComponent::iterator().enumerate() {
        let component_name = match pair_button {
            PairButtonComponent::Vietnamese => "Vietnamese",
            PairButtonComponent::English => "English",
        };

        let handle_image = match pair_button {
            PairButtonComponent::Vietnamese => scenes_materials.flag_materials.vietnam.clone(),
            PairButtonComponent::English => scenes_materials.flag_materials.united_states.clone(),
        };

        let color = match pair_button {
            PairButtonComponent::Vietnamese => match setting.get_language() {
                Language::VI => SELECTED_FLAG_COLOR,
                Language::EN => NORMAL_FLAG_COLOR,
            },
            PairButtonComponent::English => match setting.get_language() {
                Language::VI => NORMAL_FLAG_COLOR,
                Language::EN => SELECTED_FLAG_COLOR,
            },
        };

        let rect = positions[index];
        root.spawn((
            Button { ..default() },
            Node {
                left: rect.left,
                right: rect.right,
                top: rect.top,
                bottom: rect.bottom,
                width: Val::Px(NORMAL_BUTTON_SIZE),
                height: Val::Px(NORMAL_BUTTON_SIZE),
                justify_content: JustifyContent::Center,
                position_type: PositionType::Absolute,
                ..Default::default()
            },
            // BackgroundColor(Color::from(color)),
            ImageNode::new(handle_image).with_color(Color::from(color)),
        ))
        .insert(Name::new(component_name))
        .insert(pair_button.clone());
    }
}

fn button_handle_system(
    mut button_query: Query<
        (&Interaction, &ButtonComponent, &mut ImageNode),
        (Changed<Interaction>, With<Button>),
    >,
    mut setting: ResMut<Setting>,
    scenes_materials: Res<ScenesMaterials>,
    mut state: ResMut<NextState<SceneState>>,
) {
    for (interaction, button, mut ui_image) in button_query.iter_mut() {
        match *button {
            ButtonComponent::Return => match *interaction {
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
            },
            ButtonComponent::EnableSound => match *interaction {
                Interaction::None => {
                    if setting.get_enable_sound() {
                        ui_image.image = scenes_materials.icon_materials.sound_icon_on.clone()
                    } else {
                        ui_image.image = scenes_materials.icon_materials.sound_icon_off.clone()
                    }
                }
                Interaction::Hovered => {
                    ui_image.image = scenes_materials.icon_materials.sound_icon_hovered.clone()
                }
                Interaction::Pressed => {
                    let enable_sound = setting.get_enable_sound();
                    setting.set_enable_sound(!enable_sound);
                }
            },
            ButtonComponent::EnableMusic => match *interaction {
                Interaction::None => {
                    if setting.get_enable_music() {
                        ui_image.image = scenes_materials.icon_materials.music_icon_on.clone()
                    } else {
                        ui_image.image = scenes_materials.icon_materials.music_icon_off.clone()
                    }
                }
                Interaction::Hovered => {
                    ui_image.image = scenes_materials.icon_materials.music_icon_hovered.clone()
                }
                Interaction::Pressed => {
                    let enable_music = setting.get_enable_music();
                    setting.set_enable_music(!enable_music);
                }
            },
        }
    }
}

fn pair_button_handle_system(
    mut button_query: Query<(&Interaction, &PairButtonComponent, &mut BackgroundColor)>,
    mut setting: ResMut<Setting>,
    mut dictionary: ResMut<Dictionary>,
) {
    for (interaction, button, mut ui_color) in button_query.iter_mut() {
        match *button {
            PairButtonComponent::Vietnamese => match *interaction {
                Interaction::None | Interaction::Hovered => match setting.get_language() {
                    Language::VI => ui_color.0 = Color::from(SELECTED_FLAG_COLOR),
                    Language::EN => ui_color.0 = Color::from(NORMAL_FLAG_COLOR),
                },
                Interaction::Pressed => {
                    if setting.get_language() != Language::VI {
                        setting.set_language(Language::VI);
                        dictionary.set_current_language(Language::VI);
                    }
                }
            },
            PairButtonComponent::English => match *interaction {
                Interaction::None | Interaction::Hovered => match setting.get_language() {
                    Language::VI => ui_color.0 = Color::from(NORMAL_FLAG_COLOR),
                    Language::EN => ui_color.0 = Color::from(SELECTED_FLAG_COLOR),
                },
                Interaction::Pressed => {
                    if setting.get_language() != Language::EN {
                        setting.set_language(Language::EN);
                        dictionary.set_current_language(Language::EN);
                    }
                }
            },
        };
    }
}

fn text_handle_system(
    mut text_query: Query<(&TextComponent, Entity)>,
    font_materials: Res<FontMaterials>,
    dictionary: Res<Dictionary>,
    mut writer: TextUiWriter,
) {
    let font = font_materials.get_font(dictionary.get_current_language());
    let glossary = dictionary.get_glossary();
    if dictionary.is_changed() {
        for (text_type, mut entity) in text_query.iter_mut() {
            *writer.font(entity, 0) = TextFont::from_font(font.clone());
            match *text_type {
                TextComponent::Options => {
                    *writer.text(entity, 0) = glossary.options_scene_text.options.clone();
                }
                TextComponent::EnableSound => {
                    *writer.text(entity, 0) = glossary.options_scene_text.enable_sound.clone();
                }
                TextComponent::EnableMusic => {
                    *writer.text(entity, 0) = glossary.options_scene_text.enable_music.clone();
                }
                TextComponent::Language => {
                    *writer.text(entity, 0) = glossary.options_scene_text.language.clone();
                }
            }
        }
    }
}
