use crate::{
	component::{
		Direction, Hero, KnockedBack, SlashAttack, ThrustAttack, Velocity,
	},
	constants::*,
};

use bevy::prelude::*;

/// Controls the hero character based on player input.
pub fn control_hero(
	mut commands: Commands,
	input: Res<Input<KeyCode>>,
	mut query: Query<
		(Entity, &mut Velocity, &mut Direction),
		(
			With<Hero>,
			// Controls are disabled while knocked back or attacking.
			Without<KnockedBack>,
			Without<SlashAttack>,
			Without<ThrustAttack>,
		),
	>,
) {
	for (id, mut velocity, mut direction) in query.iter_mut() {
		// Movement
		let mut vx: i16 = 0;
		let mut vy: i16 = 0;
		if input.pressed(KeyCode::Up) {
			vy += 1;
		}
		if input.pressed(KeyCode::Down) {
			vy -= 1;
		}
		if input.pressed(KeyCode::Left) {
			vx -= 1;
		}
		if input.pressed(KeyCode::Right) {
			vx += 1;
		}
		// Update direction if needed. There are eight directions of
		// movement but only four for animation.
		if !input.pressed(KeyCode::Space) {
			// Not strafing.
			match (*direction, vx, vy) {
				// Moving in a cardinal direction.
				(_, 0, -1) => *direction = Direction::Down,
				(_, 0, 1) => *direction = Direction::Up,
				(_, -1, 0) => *direction = Direction::Left,
				(_, 1, 0) => *direction = Direction::Right,
				// Facing up but moving down-left, down, or right.
				(Direction::Up, _, -1) => *direction = Direction::Down,
				// Facing down but moving up-left, up, or right.
				(Direction::Down, _, 1) => *direction = Direction::Up,
				// Facing right but moving up-left, left, or down-left.
				(Direction::Right, -1, _) => *direction = Direction::Left,
				// Facing left but moving up-right, right, or down-right.
				(Direction::Left, 1, _) => *direction = Direction::Right,
				// Already facing in a reasonable direction.
				_ => {}
			};
		}
		// Update velocity.
		if vx == 0 || vy == 0 {
			velocity.x = vx as f32 * ORTHOGONAL_SPEED;
			velocity.y = vy as f32 * ORTHOGONAL_SPEED;
		} else {
			velocity.x = vx as f32 * DIAGONAL_SPEED;
			velocity.y = vy as f32 * DIAGONAL_SPEED;
		}
		// Check for sword attack.
		if input.just_pressed(KeyCode::R) {
			let advancing = match *direction {
				Direction::Up => vy == 1,
				Direction::Down => vy == -1,
				Direction::Left => vx == -1,
				Direction::Right => vx == 1,
			};
			if advancing {
				// Hero is advancing -> thrust attack.
				commands.entity(id).insert(ThrustAttack::new());
			} else {
				// Strafing/retreating/standing still -> slash attack.
				commands.entity(id).insert(SlashAttack::new());
			}
		}
	}
}