use crate::component::{
	animation::{DirectedAnimation, SimpleAnimation},
	Direction,
};

use bevy::prelude::*;

/// Updates simple animations.
pub fn animate_simple(
	mut query: Query<(&mut SimpleAnimation, &mut TextureAtlasSprite)>,
) {
	for (mut animation, mut sprite) in query.iter_mut() {
		// Update animation.
		animation.advance();
		// Set the current sprite.
		sprite.index = animation.sprite_index();
	}
}

/// Updates directed animations.
pub fn animate_directed(
	mut query: Query<(
		&mut DirectedAnimation,
		&mut TextureAtlasSprite,
		&Direction,
	)>,
) {
	// Update directional animations.
	for (mut animation, mut sprite, direction) in query.iter_mut() {
		// Update animation.
		animation.advance();
		animation.set_direction(*direction);
		// Set the current sprite.
		sprite.index = animation.sprite_index();
	}
}
