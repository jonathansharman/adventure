use crate::constants::TILE_SIZE;

use bevy::math::Vec2;
use serde::Deserialize;

#[derive(Clone, Copy, PartialEq, Eq, Deserialize)]
pub struct TileCoords {
	pub row: usize,
	pub col: usize,
}

impl From<TileCoords> for Vec2 {
	fn from(tile_coords: TileCoords) -> Self {
		Vec2::new(
			tile_coords.col as f32 * TILE_SIZE,
			tile_coords.row as f32 * -TILE_SIZE,
		)
	}
}

impl TileCoords {
	pub fn from_position(position: Vec2) -> Option<TileCoords> {
		if position.x < -0.5 || position.y > 0.5 {
			None
		} else {
			Some(TileCoords {
				row: (position.y / -TILE_SIZE + 0.5) as usize,
				col: (position.x / TILE_SIZE + 0.5) as usize,
			})
		}
	}
}
