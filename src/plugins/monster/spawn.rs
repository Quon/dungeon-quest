use crate::plugins::player::{PLAYER_SIZE_HEIGHT, PLAYER_SIZE_WIDTH};
use crate::{
    components::{
        invinsible_cooldown::InvisibleCooldownComponent, monster::MonsterComponent,
        monster_animation::MonsterAnimationComponent,
        monster_list_effects::MonsterListEffectsComponent,
    },
    materials::ingame::InGameMaterials,
    resources::{
        animation_state::AnimationState,
        dungeon::wave::Wave,
        game_data::GameData,
        monster::{
            Monster, monster_class::MonsterClass, monster_spawn_controller::MonsterSpawnController,
        },
        player::player_dungeon_stats::PlayerDungeonStats,
    },
};
use bevy::prelude::*;
use bevy::render::render_resource::Texture;
use rand::Rng;
use std::time::Duration;

pub fn spawn_monsters_classic_mode(
    mut monster_spawn_controller: ResMut<MonsterSpawnController>,
    mut texture_atlases: ResMut<Assets<TextureAtlasLayout>>,
    player_dungeon_stats: Res<PlayerDungeonStats>,
    ingame_materials: Res<InGameMaterials>,
    game_data: Res<GameData>,
    mut commands: Commands,
) {
    if player_dungeon_stats.is_room_cleared {
        return;
    } else {
        let max_avalible_monsters = monster_spawn_controller.max_avalible_monsters;
        let require_monsters = monster_spawn_controller.require_monster;
        let killed_monsters = monster_spawn_controller.killed_monsters;

        let monsters_max_level = {
            if player_dungeon_stats.current_floor_index < 2 {
                1
            } else if player_dungeon_stats.current_floor_index < 4 {
                2
            } else {
                3
            }
        };

        let raw_monsters = game_data.get_monsters();

        let raw_selected_monsters: Vec<Monster> = raw_monsters
            .iter()
            .filter(|raw_monster| raw_monster.level <= monsters_max_level)
            .cloned()
            .collect();

        let start_x = monster_spawn_controller.spawn_area_start_x;
        let start_y = monster_spawn_controller.spawn_area_start_y;
        let end_x = monster_spawn_controller.spawn_area_end_x;
        let end_y = monster_spawn_controller.spawn_area_end_y;

        loop {
            if monster_spawn_controller.alive_monsters < max_avalible_monsters
                && (require_monsters - (monster_spawn_controller.alive_monsters + killed_monsters)
                    != 0)
            {
                let mut rng = rand::thread_rng();
                let random_raw_monster_index = rng.gen_range(0..raw_selected_monsters.len());

                let raw_monster = raw_selected_monsters.get(random_raw_monster_index).unwrap();

                let x = rng.gen_range(start_x..end_x);
                let y = rng.gen_range(end_y..start_y);

                let (texture_atlas, image) = get_texture(&raw_monster, &ingame_materials);
                let texture_atlas_handle = texture_atlases.add(texture_atlas);
                let mut sprite = Sprite::from_atlas_image(
                    image,
                    TextureAtlas {
                        layout: texture_atlas_handle,
                        index: 0,
                    },
                );
                sprite.custom_size = Some(Vec2::new(
                    raw_monster.origin_width * 3.5,
                    raw_monster.origin_height * 3.5,
                ));
                let component_name = format!("Monster {}", monster_spawn_controller.alive_monsters);

                commands
                    .spawn((
                        sprite,
                        Transform {
                            translation: Vec3::new(x, y, 0.16),
                            ..Default::default()
                        },
                    ))
                    .insert(MonsterComponent {
                        current_health_points: raw_monster.health_points,
                        max_health_points: raw_monster.health_points,
                        damage: raw_monster.damage,
                        speed: raw_monster.speed,
                        level: raw_monster.level,
                        class: raw_monster.class.clone(),
                        trigger_effect: raw_monster.trigger_effect,
                        trigger_chance: raw_monster.trigger_chance.unwrap_or(0.0),
                        skill: raw_monster.skill.clone(),
                        width: raw_monster.origin_width * 3.5,
                        height: raw_monster.origin_height * 3.5,
                    })
                    .insert(MonsterListEffectsComponent::new())
                    .insert(MonsterAnimationComponent {
                        total_tiles: match raw_monster.class {
                            MonsterClass::Zombie | MonsterClass::Swampy => 4,
                            _ => 8,
                        },
                        animation_state: AnimationState::Idle,
                        animation_timer: Timer::from_seconds(0.1, TimerMode::Repeating),
                    })
                    .insert(InvisibleCooldownComponent {
                        hurt_duration: Timer::new(Duration::from_secs(0), TimerMode::Once),
                        duration: Timer::new(Duration::from_secs(0), TimerMode::Once),
                    })
                    .insert(Name::new(component_name));

                monster_spawn_controller.alive_monsters += 1;
            } else {
                break;
            }
        }
    }
}

pub fn spawn_monsters_survival_mode(
    mut monster_spawn_controller: ResMut<MonsterSpawnController>,
    mut texture_atlases: ResMut<Assets<TextureAtlasLayout>>,
    ingame_materials: Res<InGameMaterials>,
    game_data: Res<GameData>,
    mut commands: Commands,
    wave: Res<Wave>,
) {
    let max_avalible_monsters = monster_spawn_controller.max_avalible_monsters;

    if wave.timer.finished() {
        return;
    }

    if monster_spawn_controller.alive_monsters < max_avalible_monsters {
        let monsters_max_level = {
            if wave.wave_number <= 3 {
                1
            } else if wave.wave_number <= 5 {
                2
            } else {
                3
            }
        };

        let raw_monsters = game_data.get_monsters();

        let raw_selected_monsters: Vec<Monster> = raw_monsters
            .iter()
            .filter(|raw_monster| raw_monster.level <= monsters_max_level)
            .cloned()
            .collect();

        let start_x = monster_spawn_controller.spawn_area_start_x;
        let end_x = monster_spawn_controller.spawn_area_end_x;

        let start_y = monster_spawn_controller.spawn_area_start_y;
        let end_y = monster_spawn_controller.spawn_area_end_y;

        loop {
            if monster_spawn_controller.alive_monsters < max_avalible_monsters {
                let mut rng = rand::thread_rng();
                let random_raw_monster_index = rng.gen_range(0..raw_selected_monsters.len());

                let raw_monster = raw_selected_monsters.get(random_raw_monster_index).unwrap();

                let x = rng.gen_range(start_x..end_x);
                let y = rng.gen_range(end_y..start_y);

                let (texture_atlas, image) = get_texture(&raw_monster, &ingame_materials);
                let texture_atlas_handle = texture_atlases.add(texture_atlas);
                let mut sprite = Sprite::from_atlas_image(
                    image,
                    TextureAtlas {
                        layout: texture_atlas_handle,
                        index: 0,
                    },
                );
                sprite.custom_size = Some(Vec2::new(
                    raw_monster.origin_width * 3.5,
                    raw_monster.origin_height * 3.5,
                ));
                let component_name = format!("Monster {}", monster_spawn_controller.alive_monsters);

                commands
                    .spawn((
                        sprite,
                        Transform {
                            translation: Vec3::new(x, y, 0.16),
                            ..Default::default()
                        },
                    ))
                    .insert(MonsterComponent {
                        current_health_points: raw_monster.health_points,
                        max_health_points: raw_monster.health_points,
                        damage: raw_monster.damage,
                        speed: raw_monster.speed,
                        level: raw_monster.level,
                        class: raw_monster.class.clone(),
                        trigger_effect: raw_monster.trigger_effect,
                        trigger_chance: raw_monster.trigger_chance.unwrap_or(0.0),
                        skill: raw_monster.skill.clone(),
                        width: raw_monster.origin_width * 3.5,
                        height: raw_monster.origin_height * 3.5,
                    })
                    .insert(MonsterAnimationComponent {
                        total_tiles: match raw_monster.class {
                            MonsterClass::Zombie | MonsterClass::Swampy => 4,
                            _ => 8,
                        },
                        animation_state: AnimationState::Idle,
                        animation_timer: Timer::from_seconds(0.1, TimerMode::Repeating),
                    })
                    .insert(MonsterListEffectsComponent::new())
                    .insert(InvisibleCooldownComponent {
                        hurt_duration: Timer::new(Duration::from_secs(0), TimerMode::Once),
                        duration: Timer::new(Duration::from_secs(0), TimerMode::Once),
                    })
                    .insert(Name::new(component_name));

                monster_spawn_controller.alive_monsters += 1;
            } else {
                break;
            }
        }
    }
}

fn get_texture(
    monster: &Monster,
    ingame_materials: &InGameMaterials,
) -> (TextureAtlasLayout, Handle<Image>) {
    let monster_tileset = ingame_materials
        .monsters_materials
        .get_texture(monster.class.clone());

    let columns = match monster.class {
        MonsterClass::Zombie | MonsterClass::Swampy => 4,
        _ => 8,
    };

    (
        TextureAtlasLayout::from_grid(
            UVec2::new(monster.origin_width as u32, monster.origin_height as u32),
            columns,
            1,
            None,
            None,
        ),
        monster_tileset,
    )
}
