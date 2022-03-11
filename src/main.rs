// Complex types are extremely common in Bevy queries.
#![allow(clippy::type_complexity)]

mod component;
mod constants;
mod resource;
mod system;

use crate::{
	component::{Direction, SlashAttack, ThrustAttack, Velocity},
	constants::*,
	system::*,
};

use bevy::prelude::*;

fn main() {
	App::new()
		.add_startup_system(setup)
		.add_system(control_hero)
		.add_system(slash)
		.add_system(thrust)
		.add_system(move_entities)
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

fn slash(
	mut commands: Commands,
	mut query: Query<(Entity, &mut Velocity, &mut SlashAttack)>,
) {
	for (id, mut velocity, mut slash_attack) in query.iter_mut() {
		*velocity = Velocity::zero();
		if slash_attack.tick() == 0 {
			commands.entity(id).remove::<SlashAttack>();
		}
	}
}

fn thrust(
	mut commands: Commands,
	mut query: Query<(Entity, &mut Velocity, &Direction, &mut ThrustAttack)>,
) {
	for (id, mut velocity, direction, mut thrust_attack) in query.iter_mut() {
		// Rush forward if the thrust is active or else stand still.
		*velocity = if thrust_attack.is_active() {
			match direction {
				Direction::Up => Velocity {
					x: 0.0,
					y: THRUST_SPEED,
				},
				Direction::Down => Velocity {
					x: 0.0,
					y: -THRUST_SPEED,
				},
				Direction::Left => Velocity {
					x: -THRUST_SPEED,
					y: 0.0,
				},
				Direction::Right => Velocity {
					x: THRUST_SPEED,
					y: 0.0,
				},
			}
		} else {
			Velocity { x: 0.0, y: 0.0 }
		};
		// Reduce frames left and return control if finished thrusting.
		if thrust_attack.tick() == 0 {
			commands.entity(id).remove::<ThrustAttack>();
		}
	}
}
