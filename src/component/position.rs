use crate::constants::TILE_SIZE;

use bevy::prelude::Component;

#[derive(Clone, Copy, PartialEq, Component)]
pub struct Position {
	pub x: f32,
	pub y: f32,
}

#[derive(Clone, Copy, PartialEq, Eq)]
pub struct TileCoords {
	pub row: usize,
	pub col: usize,
}

impl From<TileCoords> for Position {
	fn from(tile_coords: TileCoords) -> Self {
		Self {
			x: tile_coords.col as f32 * TILE_SIZE,
			y: tile_coords.row as f32 * -TILE_SIZE,
		}
	}
}

impl From<Position> for Option<TileCoords> {
	fn from(position: Position) -> Self {
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
