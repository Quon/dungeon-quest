use bevy::prelude::*;

use crate::components::player::PlayerComponent;
use crate::components::skill::SkillComponent;
use crate::resources::dungeon::Dungeon;
use crate::resources::player::player_dungeon_stats::PlayerDungeonStats;
use crate::resources::skill::skill_type::SkillType;

pub fn unlock_room_cheat(
    mut player_dungeon_stats: ResMut<PlayerDungeonStats>,
    mut keyboard_input: ResMut<ButtonInput<KeyCode>>,
    mut dungeon: ResMut<Dungeon>,
) {
    if keyboard_input.pressed(KeyCode::KeyC) {
        let current_position = dungeon.current_floor.current_position;
        dungeon
            .current_floor
            .cleared_positions
            .insert(current_position, 1);
        player_dungeon_stats.is_room_cleared = true;
        keyboard_input.reset(KeyCode::KeyC);
    }
}

pub fn knight_skill_cheat(
    mut keyboard_input: ResMut<ButtonInput<KeyCode>>,
    mut player_skill_query: Query<&mut SkillComponent>,
) {
    if keyboard_input.pressed(KeyCode::KeyM) {
        let mut player_skill = player_skill_query.single_mut().unwrap();
        if player_skill.skill.name == SkillType::Armor {
            player_skill.monster_counter += 1;
        }
        keyboard_input.reset(KeyCode::KeyM);
    }
}

pub fn damage_player_cheat(
    mut keyboard_input: ResMut<ButtonInput<KeyCode>>,
    mut player_query: Query<&mut PlayerComponent>,
) {
    if keyboard_input.pressed(KeyCode::KeyN) {
        let mut player = player_query.single_mut().unwrap();
        player.current_health_points -= 1.0;
        keyboard_input.reset(KeyCode::KeyN);
    }
}
