use bevy::prelude::Component;

#[derive(Clone, Copy, PartialEq, Component)]
pub struct Velocity {
	pub x: f32,
	pub y: f32,
}

impl Default for Velocity {
	fn default() -> Self {
		Self { x: 0.0, y: 0.0 }
	}
}

impl Velocity {
	pub fn zero() -> Self {
		Self::default()
	}
}
