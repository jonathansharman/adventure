use bevy::prelude::*;

#[derive(Component)]
pub struct Hero;

#[derive(Component)]
pub struct Enemy;

pub type WithCharacter = Or<(With<Hero>, With<Enemy>)>;
