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

use bevy::{prelude::*, time::FixedTimestep};

fn main() {
	App::new()
		.add_startup_system(setup)
		.add_system(control_hero)
		.add_stage_before(
			CoreStage::Update,
			"fixed_update",
			SystemStage::parallel()
				.with_run_criteria(FixedTimestep::step(TIMESTEP))
				.with_system(slash)
				.with_system(thrust)
				.with_system(move_entities.after(slash).after(thrust))
				.with_system(handle_static_collisions.after(move_entities))
				.with_system(update_children.after(handle_static_collisions))
				.with_system(animate_simple.after(update_children))
				.with_system(animate_directed.after(update_children))
				.with_system(control_camera.after(animate_directed)),
		)
		.insert_resource(WindowDescriptor {
			title: "Adventure".to_string(),
			width: 800.0,
			height: 600.0,
			..Default::default()
		})
		.insert_resource(ClearColor(Color::BLACK))
		.add_plugins(DefaultPlugins)
		// TODO: Reenable if dependency incompatibility is resolved.
		//.add_plugin(PixelCameraPlugin)
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
			commands.entity(slash_attack.sword_id()).despawn_recursive();
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
			commands
				.entity(thrust_attack.sword_id())
				.despawn_recursive();
		}
	}
}
