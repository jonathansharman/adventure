use crate::component::{
	animation::{DirectedAnimation, SimpleAnimation},
	Direction, Position,
};

use bevy::prelude::*;

/// Updates simple animations.
pub fn animate_simple(
	mut query: Query<(
		&mut SimpleAnimation,
		&mut Transform,
		&mut TextureAtlasSprite,
		&Position,
	)>,
) {
	// Update simple animations.
	for (mut animation, mut transform, mut sprite, position) in query.iter_mut() {
		// Update animation.
		animation.advance();
		// Set transform according to position.
		*transform = Transform::from_scale(Vec3::new(2.0, 2.0, 1.0)).mul_transform(
			Transform::from_translation(Vec3::new(position.x.round(), position.y.round(), 0.5)),
		);
		// Set the current sprite.
		sprite.index = animation.sprite_index();
	}
}

/// Updates directional animations.
pub fn animate_directional(
	mut query: Query<(
		&mut DirectedAnimation,
		&mut Transform,
		&mut TextureAtlasSprite,
		&Position,
		&Direction,
	)>,
) {
	// Update directional animations.
	for (mut animation, mut transform, mut sprite, position, direction) in query.iter_mut() {
		// Update animation.
		animation.advance();
		animation.set_direction(*direction);
		// Set transform according to position.
		*transform = Transform::from_scale(Vec3::new(2.0, 2.0, 1.0)).mul_transform(
			Transform::from_translation(Vec3::new(position.x.round(), position.y.round(), 0.5)),
		);
		// Set the current sprite.
		sprite.index = animation.sprite_index();
	}
}
