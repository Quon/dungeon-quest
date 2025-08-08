use bevy::prelude::*;

use crate::components::player::PlayerComponent;
use crate::components::player_list_effects::PlayerListEffectsComponent;
use crate::components::skill::SkillComponent;
use crate::resources::effect::effect_type::EffectType;
use crate::resources::skill::skill_type::SkillType;

pub fn update_stats(
    mut player_query: Query<(
        &mut PlayerComponent,
        &SkillComponent,
        &PlayerListEffectsComponent,
    )>,
) {
    let (mut player, player_skill, player_list_effects) = player_query.single_mut().unwrap();

    let mut speed_percent_bonus = 0.0;
    let mut damage_percent_bonus = 0.0;
    let mut critical_chance_bonus = 0.0;
    let mut dodge_chance_bonus = 0.0;
    let mut restore_chance_bonus = 0.0;

    if player_skill.skill.name == SkillType::TimeToHunt && !player_skill.duration.finished() {
        speed_percent_bonus += player_skill.skill.speed_percent_bonus.unwrap();
        critical_chance_bonus += player_skill.skill.critical_chance_bonus.unwrap();
        dodge_chance_bonus = 1.0;
    }

    if player_skill.skill.name == SkillType::AnimalInstinct && !player_skill.duration.finished() {
        speed_percent_bonus += player_skill.skill.speed_percent_bonus.unwrap();
        critical_chance_bonus += player_skill.skill.critical_chance_bonus.unwrap();
        restore_chance_bonus += player_skill.skill.restore_chance_bonus.unwrap();
        damage_percent_bonus += player_skill.skill.damge_precent_bonus.unwrap();
    }

    for (effect_type, duration) in player_list_effects.activated_effects.iter() {
        if !duration.finished() {
            let bonus = player_list_effects
                .information
                .iter()
                .find(|effect_information| effect_information.name == *effect_type)
                .unwrap()
                .bonus;

            match effect_type {
                EffectType::SpeedUp | EffectType::Slow => {
                    speed_percent_bonus += bonus;
                }
                EffectType::EvasionUp => {
                    dodge_chance_bonus += bonus;
                }
                EffectType::ReduceDamage => {
                    damage_percent_bonus += bonus;
                }
                EffectType::Focus => {
                    critical_chance_bonus += bonus;
                }
                _ => {}
            }
        }
    }

    let base_speed = player.base_stats.speed;
    let base_critical_chance = player.base_stats.critical_chance;
    let base_dodge_chance = player.base_stats.dodge_chance;
    let base_restore_chance = player.base_stats.restore_chance;

    player.speed = base_speed + base_speed * speed_percent_bonus;
    player.critical_chance = base_critical_chance + critical_chance_bonus;
    player.dodge_chance = base_dodge_chance + dodge_chance_bonus;
    player.restore_chance = base_restore_chance + restore_chance_bonus;
    player.damage_percent_bonus = damage_percent_bonus;
}
