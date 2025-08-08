use bevy::prelude::*;
use std::time::Duration;

use crate::materials::font::FontMaterials;
use crate::resources::dictionary::Dictionary;
use crate::resources::game_data::PauseSceneData;
use crate::resources::player::player_dungeon_stats::PlayerDungeonStats;
use crate::scenes::SceneState;

pub struct ClassicModeUIPlugin;

#[derive(Component)]
pub struct CenterTextComponent {
    pub timer: Timer,
}

#[derive(Component)]
struct FloorTextComponent;

#[derive(Resource)]
struct ClassicModeUIData {
    pub user_interface_root: Entity,
}

impl Plugin for ClassicModeUIPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(SceneState::InGameClassicMode), setup);

        app.add_systems(
            Update,
            (
                center_text_handle_system,
                top_right_conner_text_handle_system,
            )
                .run_if(
                    in_state(SceneState::InGameClassicMode)
                        .and(not(resource_exists::<PauseSceneData>)),
                ),
        );

        app.add_systems(OnExit(SceneState::InGameClassicMode), cleanup);
    }
}

fn setup(mut commands: Commands, font_materials: Res<FontMaterials>, dictionary: Res<Dictionary>) {
    let user_interface_root = commands
        .spawn((
            Node {
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                position_type: PositionType::Absolute,
                align_items: AlignItems::Center,
                justify_content: JustifyContent::Center,
                align_content: AlignContent::Center,
                ..Default::default()
            },
            BackgroundColor(Color::NONE),
        ))
        .with_children(|parent| {
            center_text(parent, &font_materials, &dictionary);
            floor_text(parent, &font_materials, &dictionary);
        })
        .insert(Name::new("ClassicModeUI"))
        .id();

    commands.insert_resource(ClassicModeUIData {
        user_interface_root,
    });
}

fn cleanup(mut commands: Commands, classic_mode_ui_data: Res<ClassicModeUIData>) {
    commands
        .entity(classic_mode_ui_data.user_interface_root)
        .despawn_recursive();
}

fn center_text(root: &mut ChildSpawnerCommands, font_materials: &FontMaterials, dictionary: &Dictionary) {
    let font = font_materials.get_font(dictionary.get_current_language());
    let glossary = dictionary.get_glossary();

    let value = format!("{} {}", glossary.ingame_text.floor.clone(), 1);

    root.spawn((
        Node {
            position_type: PositionType::Absolute,
            ..Default::default()
        },
        Text::new(value),
        TextFont {
            font: font.clone(),
            font_size: 50.0,
            ..default()
        },
        TextColor(Color::WHITE),
        TextLayout::new_with_justify(JustifyText::Center),
    ))
    .insert(CenterTextComponent {
        timer: Timer::new(Duration::from_secs(1), TimerMode::Once),
    })
    .insert(Name::new("CenterText"));
}

fn center_text_handle_system(
    mut text_query: Query<(Entity, &mut CenterTextComponent, &mut Visibility)>,
    player_dungeon_stats: Res<PlayerDungeonStats>,
    dictionary: Res<Dictionary>,
    time: Res<Time>,
    mut writer: TextUiWriter,
) {
    let (mut entity, mut center_text, mut visibility) = text_query.single_mut().unwrap();
    center_text.timer.tick(time.delta());

    if center_text.timer.finished() {
        *visibility = Visibility::Hidden;
    } else {
        let glossary = dictionary.get_glossary();
        let current_floor_index = player_dungeon_stats.current_floor_index;

        let value = format!(
            "{} {}",
            glossary.ingame_text.floor.clone(),
            current_floor_index + 1
        );

        *writer.text(entity, 0) = value;
        *visibility = Visibility::Visible;
    }
}

fn floor_text(root: &mut ChildSpawnerCommands, font_materials: &FontMaterials, dictionary: &Dictionary) {
    let font = font_materials.get_font(dictionary.get_current_language());
    root.spawn((
        Node {
            position_type: PositionType::Absolute,
            top: Val::Px(0.0),
            right: Val::Px(10.0),
            ..Default::default()
        },
        Text::new("1"),
        TextFont {
            font: font.clone(),
            font_size: 35.0,
            ..Default::default()
        },
        TextColor(Color::WHITE),
        TextLayout::new_with_justify(JustifyText::Center),
    ))
    .insert(FloorTextComponent)
    .insert(Name::new("FloorTextComponent"));
}

fn top_right_conner_text_handle_system(
    mut text_query: Query<Entity, With<FloorTextComponent>>,
    player_dungeon_stats: Res<PlayerDungeonStats>,
    mut writer: TextUiWriter,
) {
    let entity = text_query.single().unwrap();

    if player_dungeon_stats.is_changed() {
        *writer.text(entity, 0) = (player_dungeon_stats.current_floor_index + 1).to_string();
    }
}
