use crate::component::{Position, Velocity};

use bevy::prelude::*;

/// Moves entities with velocity.
pub fn move_entities(mut query: Query<(&mut Position, &Velocity)>) {
	for (mut position, velocity) in query.iter_mut() {
		position.x += velocity.x;
		position.y += velocity.y;
	}
}
