use crate::config::*;
use crate::materials::font::FontMaterials;
use crate::materials::menu_box::MenuBoxMaterials;
use crate::materials::scenes::ScenesMaterials;
use crate::resources::dictionary::Dictionary;
use crate::resources::game_data::PauseSceneData;
use crate::resources::profile::Profile;
use crate::scenes::SceneState;
use bevy::color::palettes::css::GRAY;
use bevy::prelude::*;
use std::slice::Iter;

const BOX_TILE_SIZE: f32 = 60.0;
const BOX_WIDTH_TILES: f32 = 7.0;
const BOX_HEIGHT_TILES: f32 = 3.0;

const BOX_ARRAY: [[i8; 7]; 3] = [
    [0, 1, 1, 1, 1, 1, 2],
    [3, 4, 4, 4, 4, 4, 5],
    [6, 7, 7, 7, 7, 7, 8],
];

#[derive(Component, Copy, Clone, PartialEq, Eq)]
pub enum ButtonComponent {
    Continue,
    Quit,
}

// the clumsy way to differ pause screen from other pausing screens, e.x. reward screen for survival mode
#[derive(Resource)]
pub struct PauseSceneFlag;

impl ButtonComponent {
    pub fn iterator() -> Iter<'static, ButtonComponent> {
        [ButtonComponent::Continue, ButtonComponent::Quit].iter()
    }
}

// works without SceneState now, should consider to move
pub fn pause(
    mut keyboard_input: ResMut<ButtonInput<KeyCode>>,
    mut commands: Commands,
    font_materials: Res<FontMaterials>,
    scenes_materials: Res<ScenesMaterials>,
    dictionary: Res<Dictionary>,
) {
    if keyboard_input.pressed(KeyCode::Escape) {
        keyboard_input.reset(KeyCode::Escape);

        let user_interface_root = commands
            .spawn((
                Node {
                    width: Val::Percent(100.0),
                    height: Val::Percent(100.0),
                    position_type: PositionType::Absolute,
                    ..Default::default()
                },
                BackgroundColor(Color::NONE),
            ))
            .with_children(|parent| {
                menu_box(parent, &scenes_materials.menu_box_materials);
                buttons(parent, &font_materials, &dictionary);
            })
            .insert(Name::new("PauseUI"))
            .id();

        commands.insert_resource(PauseSceneData {
            user_interface_root,
        });
        commands.insert_resource(PauseSceneFlag);
    }
}

fn menu_box(root: &mut ChildSpawnerCommands, menu_box_materials: &MenuBoxMaterials) {
    let start_left = (WINDOW_HEIGHT * RESOLUTION - BOX_TILE_SIZE * BOX_WIDTH_TILES) / 2.0;
    let start_top = (WINDOW_HEIGHT - BOX_TILE_SIZE * BOX_HEIGHT_TILES) / 2.0;

    root.spawn(Node {
        ..Default::default()
    })
    .with_children(|parent| {
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

                parent.spawn((
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
    })
    .insert(Name::new("MenuBox"));
}

fn buttons(root: &mut ChildSpawnerCommands, font_materials: &FontMaterials, dictionary: &Dictionary) {
    let font = font_materials.get_font(dictionary.get_current_language());
    let glossary = dictionary.get_glossary();

    for button in ButtonComponent::iterator() {
        let value = match *button {
            ButtonComponent::Continue => glossary.shared_text.continue_.clone(),
            ButtonComponent::Quit => glossary.shared_text.quit.clone(),
        };

        let top_position = match *button {
            ButtonComponent::Continue => 250.0,
            ButtonComponent::Quit => 300.0,
        };

        root.spawn((
            Button { ..default() },
            Node {
                left: Val::Px((WINDOW_HEIGHT * RESOLUTION - 300.0) / 2.0),
                top: Val::Px(top_position),
                right: Val::Auto,
                bottom: Val::Auto,
                width: Val::Px(300.0),
                height: Val::Px(35.0),
                justify_content: JustifyContent::Center,
                position_type: PositionType::Absolute,
                ..Default::default()
            },
            BackgroundColor(Color::NONE),
        ))
        .with_children(|parent| {
            parent.spawn((
                Text::new(value.clone()),
                TextFont {
                    font: font.clone(),
                    font_size: 35.0,
                    ..Default::default()
                },
                TextColor(Color::from(GRAY)),
                TextLayout::new_with_justify(JustifyText::Center),
            ));
        })
        .insert(Name::new(value.clone()))
        .insert(*button);
    }
}

pub fn button_handle_system(
    mut button_query: Query<
        (&Interaction, &ButtonComponent, &Children),
        (Changed<Interaction>, With<Button>),
    >,
    text_query: Query<Entity>,
    mut profile: ResMut<Profile>,
    mut next_state: ResMut<NextState<SceneState>>,
    mut commands: Commands,
    pause_scene_data: Res<PauseSceneData>,
    mut writer: TextUiWriter,
) {
    for (interaction, button, children) in button_query.iter_mut() {
        let entity = text_query.get(children[0]).unwrap();
        match *interaction {
            Interaction::None => *writer.color(entity, 0) = TextColor::from(GRAY),
            Interaction::Hovered => *writer.color(entity, 0) = TextColor::BLACK,
            Interaction::Pressed => {
                if *button == ButtonComponent::Quit {
                    profile.is_run_finished = true;
                    next_state.set(SceneState::MainMenuScene);
                }

                commands
                    .entity(pause_scene_data.user_interface_root)
                    .despawn();
                commands.remove_resource::<PauseSceneData>();
                commands.remove_resource::<PauseSceneFlag>();
            }
        }
    }
}
