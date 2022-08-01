use bevy::{math::Vec3, prelude::Transform, transform::TransformBundle};

/// A layer for the purpose of z-ordering.
pub enum Layer {
	Back,
	Mid,
	Top,
}

impl Layer {
	/// The z-coordinate of this layer.
	fn z(&self) -> f32 {
		match self {
			Layer::Back => -1.0,
			Layer::Mid => 0.0,
			Layer::Top => 1.0,
		}
	}
}

/// A [`TransformBundle`], using `layer` to determine the z-coordinate. Note
/// that for a child entity, this transform is relative to its parent entity.
pub fn transform_bundle(x: f32, y: f32, layer: Layer) -> TransformBundle {
	TransformBundle::from_transform(Transform::from_translation(Vec3::new(
		x,
		y,
		layer.z(),
	)))
}
