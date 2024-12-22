use crate::component::{
	animation::{DirectedAnimation, SimpleAnimation},
	Direction,
};

use bevy::prelude::*;

/// Updates simple animations.
pub fn animate_simple(mut query: Query<(&mut SimpleAnimation, &mut Sprite)>) {
	for (mut animation, mut sprite) in query.iter_mut() {
		// Update animation.
		animation.advance();
		// Set the current sprite.
		if let Some(atlas) = sprite.texture_atlas.as_mut() {
			atlas.index = animation.sprite_index();
		}
	}
}

/// Updates directed animations.
pub fn animate_directed(
	mut query: Query<(&mut DirectedAnimation, &mut Sprite, &Direction)>,
) {
	// Update directional animations.
	for (mut animation, mut sprite, direction) in query.iter_mut() {
		// Update animation.
		animation.advance();
		animation.set_direction(*direction);
		// Set the current sprite.
		if let Some(atlas) = sprite.texture_atlas.as_mut() {
			atlas.index = animation.sprite_index();
		}
	}
}
