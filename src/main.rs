mod component;
mod constants;
mod resource;
mod system;

use crate::{
	component::{Direction, SlashAttack, ThrustAttack},
	constants::*,
	system::*,
};

use avian2d::{math::Vector, prelude::*};
use bevy::{prelude::*, window::WindowResolution};

fn main() {
	App::new()
		.add_systems(Startup, setup)
		.add_systems(
			Update,
			(
				control_hero,
				(slash, thrust),
				update_children,
				(animate_simple, (animate_directed, control_camera).chain()),
			)
				.chain(),
		)
		.insert_resource(ClearColor(Color::BLACK))
		.insert_resource(Gravity(Vector::ZERO))
		.add_plugins((
			DefaultPlugins
				.set(WindowPlugin {
					primary_window: Some(Window {
						title: "Adventure".to_string(),
						resolution: WindowResolution::new(800.0, 600.0)
							.with_scale_factor_override(2.0),
						..default()
					}),
					..default()
				})
				// Use nearest sampling rather than linear interpolation.
				.set(ImagePlugin::default_nearest()),
			PhysicsPlugins::new(PreUpdate),
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
