use bevy::prelude::Component;

/// A character's relationship to the hero. All characters have a faction, so
/// this component can be used to query for characters.
#[derive(Component)]
pub enum Faction {
	Ally,
	Enemy,
}
