use bevy::prelude::*;

use crate::config::*;
use crate::materials::font::FontMaterials;
use crate::materials::menu_box::MenuBoxMaterials;
use crate::materials::scenes::ScenesMaterials;
use crate::resources::dictionary::Dictionary;
use crate::resources::language::Language;
use crate::scenes::SceneState;

const RETURN_BUTTON_SIDE: f32 = 50.0;
const MENU_BOX_TILE_SIZE: f32 = 60.0;

const HELP_BOX_WIDTH_TILES: f32 = 9.0;
const HELP_BOX_HEIGHT_TILES: f32 = 8.0;

const HELP_BOX_ARRAY: [[i8; 9]; 8] = [
    [0, 1, 1, 1, 1, 1, 1, 1, 2],
    [3, 4, 4, 4, 4, 4, 4, 4, 5],
    [3, 4, 4, 4, 4, 4, 4, 4, 5],
    [3, 4, 4, 4, 4, 4, 4, 4, 5],
    [3, 4, 4, 4, 4, 4, 4, 4, 5],
    [3, 4, 4, 4, 4, 4, 4, 4, 5],
    [3, 4, 4, 4, 4, 4, 4, 4, 5],
    [6, 7, 7, 7, 7, 7, 7, 7, 8],
];

#[derive(Component, PartialEq)]
struct ReturnButtonComponent;

pub struct HelpScenePlugin;

#[derive(Resource)]
struct HelpSceneData {
    user_interface_root: Entity,
}

impl Plugin for HelpScenePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(SceneState::HelpScene), setup);
        app.add_systems(
            Update,
            button_handle_system.run_if(in_state(SceneState::HelpScene)),
        );
        app.add_systems(OnExit(SceneState::HelpScene), cleanup);
    }
}

fn setup(
    mut commands: Commands,
    font_materials: Res<FontMaterials>,
    scenes_materials: Res<ScenesMaterials>,
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
            help_menu_box(parent, &scenes_materials.menu_box_materials);
            texts(parent, &font_materials, &dictionary);
            control_texts(parent, &font_materials, &dictionary);
            return_button_component(parent, &scenes_materials)
        })
        .id();

    commands.insert_resource(HelpSceneData {
        user_interface_root,
    });
}

fn cleanup(mut commands: Commands, help_scene_data: Res<HelpSceneData>) {
    commands
        .entity(help_scene_data.user_interface_root)
        .despawn_recursive();
}

fn help_menu_box(root: &mut ChildBuilder, menu_box_materials: &MenuBoxMaterials) {
    let start_left = (WINDOW_HEIGHT * RESOLUTION - MENU_BOX_TILE_SIZE * HELP_BOX_WIDTH_TILES) / 2.0;

    let start_top = (WINDOW_HEIGHT - MENU_BOX_TILE_SIZE * HELP_BOX_HEIGHT_TILES) / 2.0;

    for (row_index, row) in HELP_BOX_ARRAY.iter().enumerate() {
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
}

fn texts(root: &mut ChildBuilder, font_materials: &FontMaterials, dictionary: &Dictionary) {
    let font = font_materials.get_font(dictionary.get_current_language());
    let glossary = dictionary.get_glossary();

    let position_of_texts: [[f32; 2]; 8] = [
        [465.0, 100.0],
        [300.0, 160.0],
        [300.0, 205.0],
        [300.0, 250.0],
        [300.0, 295.0],
        [300.0, 340.0],
        [300.0, 385.0],
        [300.0, 430.0],
    ];

    for (index, position) in position_of_texts.iter().enumerate() {
        let value: String = match index {
            0 => glossary.help_scene_text.help.clone(),
            1 => glossary.help_scene_text.move_up.clone(),
            2 => glossary.help_scene_text.move_down.clone(),
            3 => glossary.help_scene_text.move_left.clone(),
            4 => glossary.help_scene_text.move_right.clone(),
            5 => glossary.help_scene_text.use_skill.clone(),
            6 => glossary.help_scene_text.attack.clone(),
            7 => glossary.help_scene_text.aim.clone(),
            _ => panic!("Unknown text"),
        };

        let font_size: f32 = match index {
            0 => 50.0,
            _ => 30.0,
        };

        let mut position_left = position[0];
        let position_top = position[1];

        if index == 0 && dictionary.get_current_language() == Language::VI {
            position_left = 438.0;
        }

        root.spawn((
            Node {
                position_type: PositionType::Absolute,
                left: Val::Px(position_left),
                top: Val::Px(position_top),
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
        ));
    }
}

fn control_texts(root: &mut ChildBuilder, font_materials: &FontMaterials, dictionary: &Dictionary) {
    let font = font_materials.get_font(dictionary.get_current_language());

    let position_of_texts: [[f32; 2]; 7] = [
        [645.0, 160.0],
        [650.0, 205.0],
        [650.0, 250.0],
        [650.0, 295.0],
        [620.0, 340.0],
        [620.0, 385.0],
        [620.0, 430.0],
    ];

    for (index, position) in position_of_texts.iter().enumerate() {
        let value: &str = match index {
            0 => "W",
            1 => "S",
            2 => "A",
            3 => "D",
            4 => "SPACE",
            5 => "MOUSE 1",
            6 => "MOUSE",
            _ => panic!("Unknown text"),
        };

        root.spawn((
            Node {
                position_type: PositionType::Absolute,
                left: Val::Px(position[0]),
                top: Val::Px(position[1]),
                ..Default::default()
            },
            Text::new(value),
            TextFont {
                font: font.clone(),
                font_size: 30.0,
                ..Default::default()
            },
            TextColor(Color::BLACK),
            TextLayout::new_with_justify(JustifyText::Center),
        ));
    }
}

fn return_button_component(root: &mut ChildBuilder, scenes_materials: &ScenesMaterials) {
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
    .insert(ReturnButtonComponent);
}

fn button_handle_system(
    mut button_query: Query<
        (&Interaction, &mut ImageNode),
        (Changed<Interaction>, With<ReturnButtonComponent>),
    >,
    scenes_materials: Res<ScenesMaterials>,
    mut state: ResMut<NextState<SceneState>>,
) {
    for (interaction, mut ui_image) in button_query.iter_mut() {
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
