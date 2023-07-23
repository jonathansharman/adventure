use bevy::{math::Vec3, prelude::*};

/// A layer for the purpose of z-ordering.
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

/// A [`SpatialBundle`], using `layer` to determine the z-coordinate. Note that
/// for a child entity, this transform is relative to its parent entity.
pub fn spatial_bundle(x: f32, y: f32, layer: Layer) -> SpatialBundle {
	SpatialBundle::from_transform(Transform::from_translation(Vec3::new(
		x,
		y,
		layer.z(),
	)))
}
