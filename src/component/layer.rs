use bevy::{math::Vec3, prelude::*};

/// A layer for the purpose of z-ordering.
#[derive(Component)]
pub enum Layer {
	Back,
	Mid,
	Front,
	Camera,
}

impl Layer {
	/// The z-coordinate of this layer.
	pub fn z(&self) -> f32 {
		match self {
			Layer::Back => -1.0,
			Layer::Mid => 0.0,
			Layer::Front => 1.0,
			Layer::Camera => 2.0,
		}
	}
}
