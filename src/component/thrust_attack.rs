use bevy::prelude::Component;

#[derive(Component)]
pub struct ThrustAttack {
	frames_left: u32,
	is_active: bool,
}

impl ThrustAttack {
	pub fn new() -> Self {
		const THRUST_FRAMES: u32 = 9;

		Self {
			frames_left: THRUST_FRAMES,
			is_active: true,
		}
	}

	/// Whether this thrust attack is able to hit a character.
	pub fn is_active(&self) -> bool {
		self.is_active
	}

	/// Makes the thrust attack inactive (unable to hit a character).
	pub fn deactivate(&mut self) {
		self.is_active = false;
	}

	/// Decrements and then returns the number of frames left before the effect
	/// has worn off.
	#[must_use]
	pub fn tick(&mut self) -> u32 {
		self.frames_left -= 1;
		self.frames_left
	}
}
