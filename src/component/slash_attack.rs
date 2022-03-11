use bevy::prelude::{Component, Entity};

use std::collections::HashSet;

#[derive(Component)]
pub struct SlashAttack {
	frames_left: u32,
	/// IDs of characters this attack has already hit.
	hit_set: HashSet<Entity>,
}

impl SlashAttack {
	pub fn new() -> Self {
		const SLASH_FRAMES: u32 = 9;

		Self {
			frames_left: SLASH_FRAMES,
			hit_set: HashSet::new(),
		}
	}

	/// Checks whether the entity with the given ID has already been hit by this
	/// slash attack.
	pub fn has_been_hit(&self, id: Entity) -> bool {
		self.hit_set.contains(&id)
	}

	/// Marks the entity with the given ID as having been hit by this slash
	/// attack.
	pub fn mark_as_hit(&mut self, id: Entity) {
		self.hit_set.insert(id);
	}

	/// Decrements and then returns the number of frames left before the effect
	/// has worn off.
	#[must_use]
	pub fn tick(&mut self) -> u32 {
		self.frames_left -= 1;
		self.frames_left
	}
}
