use bevy::{
    prelude::{Color, Query, Res, Time, With},
    sprite::Sprite,
};
use bevy::sprite::TextureAtlas;
use crate::components::{invinsible_cooldown::InvisibleCooldownComponent, player::PlayerComponent};
use bevy::color::palettes::css::*;

pub fn invincible_cooldown(
    mut invincible_cooldown_query: Query<&mut InvisibleCooldownComponent, With<PlayerComponent>>,
    time: Res<Time>,
) {
    let mut invincible_cooldown = invincible_cooldown_query.single_mut();

    if !invincible_cooldown.duration.finished() {
        invincible_cooldown.duration.tick(time.delta());
    }

    if !invincible_cooldown.hurt_duration.finished() {
        invincible_cooldown.hurt_duration.tick(time.delta());
    }
}

pub fn hurt_duration_color(
    mut invincible_cooldown_query: Query<
        (&InvisibleCooldownComponent, &mut Sprite),
        With<PlayerComponent>,
    >,
) {
    let (invincible_cooldown, mut texture) = invincible_cooldown_query.single_mut();

    if !invincible_cooldown.hurt_duration.finished() {
        texture.color = Color::from(RED);
    } else {
        texture.color = Color::default();
    }
}
