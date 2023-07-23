// Complex types are extremely common in Bevy queries.
#![allow(clippy::type_complexity)]

mod component;
mod constants;
mod resource;
mod system;

use crate::{
	component::{Direction, SlashAttack, ThrustAttack},
	constants::*,
	system::*,
};

use bevy::prelude::*;
use bevy_pixel_camera::PixelCameraPlugin;
use bevy_xpbd_2d::{math::Vector, prelude::*};

fn main() {
	App::new()
		.add_systems(Startup, setup)
		.add_systems(Update, control_hero)
		.add_systems(
			FixedUpdate,
			(
				(slash, thrust),
				update_children,
				(animate_simple, (animate_directed, control_camera).chain()),
			)
				.chain(),
		)
		.insert_resource(FixedTime::new_from_secs(TIMESTEP))
		.insert_resource(ClearColor(Color::BLACK))
		// Disable anti-aliasing.
		.insert_resource(Msaa::Off)
		.insert_resource(Gravity(Vector::ZERO))
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
			PhysicsPlugins::default(),
		))
		.run();
}

fn slash(
	mut commands: Commands,
	mut query: Query<(Entity, &mut LinearVelocity, &mut SlashAttack)>,
) {
	for (id, mut velocity, mut slash_attack) in query.iter_mut() {
		*velocity = LinearVelocity(Vector::ZERO);
		if slash_attack.tick() == 0 {
			commands.entity(id).remove::<SlashAttack>();
			commands.entity(slash_attack.sword_id()).despawn_recursive();
		}
	}
}

fn thrust(
	mut commands: Commands,
	mut query: Query<(
		Entity,
		&mut LinearVelocity,
		&Direction,
		&mut ThrustAttack,
	)>,
) {
	for (id, mut velocity, direction, mut thrust_attack) in query.iter_mut() {
		// Rush forward if the thrust is active or else stand still.
		*velocity = if thrust_attack.is_active() {
			match direction {
				Direction::Up => LinearVelocity(Vector::new(0.0, THRUST_SPEED)),
				Direction::Down => {
					LinearVelocity(Vector::new(0.0, -THRUST_SPEED))
				}
				Direction::Left => {
					LinearVelocity(Vector::new(-THRUST_SPEED, 0.0))
				}
				Direction::Right => {
					LinearVelocity(Vector::new(THRUST_SPEED, 0.0))
				}
			}
		} else {
			LinearVelocity(Vector::ZERO)
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
