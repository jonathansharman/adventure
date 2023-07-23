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
use bevy_pixel_camera::PixelCameraPlugin;

fn main() {
	App::new()
		.add_systems(Startup, setup)
		.add_systems(Update, control_hero)
		.add_systems(
			FixedUpdate,
			(
				(slash, thrust),
				move_entities,
				handle_static_collisions,
				update_children,
				(animate_simple, (animate_directed, control_camera).chain()),
			)
				.chain(),
		)
		.insert_resource(FixedTime::new_from_secs(TIMESTEP))
		.insert_resource(ClearColor(Color::BLACK))
		// Disable anti-aliasing.
		.insert_resource(Msaa::Off)
		.add_plugins((
			DefaultPlugins
				.set(WindowPlugin {
					primary_window: Some(Window {
						title: "Adventure".to_string(),
						resolution: (800.0, 600.0).into(),
						..default()
					}),
					..default()
				})
				// Use nearest sampling rather than linear interpolation.
				.set(ImagePlugin::default_nearest()),
			PixelCameraPlugin,
		))
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
