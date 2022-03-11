use crate::{
	component::{Position, Velocity},
	constants::*,
};

use bevy::prelude::Component;

/// Makes a character be knocked back with some velocity for a short duration.
#[derive(Component)]
pub struct KnockedBack {
	velocity: Velocity,
	frames_left: u32,
}

impl KnockedBack {
	/// Knocked back in the direction from `from` to `to`, at standard speed.
	pub fn from_positions(from: &Position, to: &Position) -> Self {
		let mut velocity = Velocity {
			x: to.x - from.x,
			y: to.y - from.y,
		};
		// Normalize knockback velocity to the desired speed.
		if velocity.x != 0.0 || velocity.y != 0.0 {
			let magnitude = f32::sqrt(velocity.x.powi(2) + velocity.y.powi(2));
			velocity.x *= KNOCKBACK_SPEED / magnitude;
			velocity.y *= KNOCKBACK_SPEED / magnitude;
		} else {
			// To and from are direction on top of each other. Knock towards the
			// right arbitrarily.
			velocity.x = KNOCKBACK_SPEED;
		}
		Self {
			velocity,
			frames_left: KNOCKBACK_FRAMES,
		}
	}

	pub fn velocity(&self) -> Velocity {
		self.velocity
	}

	/// Decrements and then returns the number of frames left before the effect
	/// has worn off.
	#[must_use]
	pub fn tick(&mut self) -> u32 {
		self.frames_left -= 1;
		self.frames_left
	}
}
