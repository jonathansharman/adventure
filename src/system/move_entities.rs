use crate::component::Velocity;

use bevy::prelude::*;

/// Moves entities with velocity.
pub fn move_entities(mut query: Query<(&mut Transform, &Velocity)>) {
	for (mut transform, velocity) in query.iter_mut() {
		transform.translation.x += velocity.x;
		transform.translation.y += velocity.y;
	}
}
