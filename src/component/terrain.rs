use bevy::prelude::Component;
use serde::Deserialize;

#[derive(Clone, Copy, PartialEq, Eq, Component, Deserialize)]
#[repr(u16)]
pub enum Terrain {
	Floor,
	Wall,
	Grass,
	Forest,
	Stone,
	Mountain,
	Snow,
	Glacier,
	Sand,
	Ocean,
}

impl Terrain {
	pub fn blocks_movement(&self) -> bool {
		match self {
			Self::Floor
			| Self::Grass
			| Self::Stone
			| Self::Snow
			| Self::Sand => false,
			Self::Wall
			| Self::Forest
			| Self::Mountain
			| Self::Glacier
			| Self::Ocean => true,
		}
	}
}
