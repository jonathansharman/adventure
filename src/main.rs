mod component;
mod constants;
mod resource;
mod system;

use crate::{
	component::{Direction, Hero, Position},
	system::*,
};

use bevy::prelude::*;

fn main() {
	App::new()
		.add_startup_system(setup)
		.add_system(control_hero)
		.add_system(animate_simple)
		.add_system(animate_directional)
		.insert_resource(WindowDescriptor {
			title: "Adventure".to_string(),
			width: 800.0,
			height: 600.0,
			..Default::default()
		})
		.insert_resource(ClearColor(Color::BLACK))
		.add_plugins(DefaultPlugins)
		.run();
}

pub fn control_hero(
	input: Res<Input<KeyCode>>,
	mut query: Query<(&mut Position, &mut Direction), With<Hero>>,
) {
	let (mut position, mut direction) = query.single_mut();
	if input.pressed(KeyCode::Left) {
		position.x -= 1.0;
		*direction = Direction::Left;
	}
	if input.pressed(KeyCode::Right) {
		position.x += 1.0;
		*direction = Direction::Right;
	}
	if input.pressed(KeyCode::Up) {
		position.y += 1.0;
		*direction = Direction::Up;
	}
	if input.pressed(KeyCode::Down) {
		position.y -= 1.0;
		*direction = Direction::Down;
	}
}
