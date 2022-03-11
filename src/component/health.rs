use bevy::prelude::Component;

/// Represents a character's health or hit points.
#[derive(Component)]
pub struct Health {
	current: i32,
	max: i32,
}

impl Health {
	/// Health with current and maximum set to `max`.
	pub fn new(max: i32) -> Self {
		Self { current: max, max }
	}

	pub fn current(&self) -> i32 {
		self.current
	}

	pub fn max(&self) -> i32 {
		self.max
	}

	/// Decrease health by `amount`, to a minimum of 0.
	pub fn damage(&mut self, amount: u32) {
		self.current = i32::max(0, self.current - amount as i32);
	}

	/// Increase health by `amount`, up to the maximum.
	pub fn heal(&mut self, amount: u32) {
		self.current = i32::min(self.max, self.current + amount as i32);
	}
}
