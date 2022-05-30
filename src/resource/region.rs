use crate::{
	component::{
		animation::{SimpleAnimation, SimpleFrame},
		collider::RectangleCollider,
		Direction, Heart, Position, Terrain, TileCoords,
	},
	constants::*,
	resource::SpriteSheets,
};

use bevy::{ecs::entity::Entity, prelude::*};
use ron::de::from_reader;
use serde::Deserialize;

use std::{fs::File, path::Path};

/// An entrance to a region.
#[derive(Clone, Copy, PartialEq, Eq, Deserialize)]
pub struct Entrance {
	pub location: TileCoords,
	pub direction: Direction,
}

/// An exit from one region to another.
#[derive(PartialEq, Eq, Clone, Deserialize)]
pub struct Exit {
	pub location: TileCoords,
	pub target_region: String,
	pub target_entrance_idx: usize,
}

/// Used for reading region data from a file.
#[derive(Eq, PartialEq, Clone, Deserialize)]
struct RegionData {
	col_count: usize,
	terrain: Vec<Terrain>,
	entrances: Vec<Entrance>,
	exits: Vec<Exit>,
	enemies: Vec<EnemyData>,
	heart_locations: Vec<TileCoords>,
}

/// Used for reading enemy data from a file.
#[derive(Eq, PartialEq, Clone, Deserialize)]
struct EnemyData {
	location: TileCoords,
}

/// A set of tiles representing a game region, with links to other regions.
#[derive(PartialEq, Eq, Clone)]
pub struct Region {
	row_count: usize,
	col_count: usize,
	tiles: Vec<Entity>,
	entrances: Vec<Entrance>,
	exits: Vec<Exit>,
}

impl Region {
	pub fn load<P: AsRef<Path>>(
		commands: &mut Commands,
		sprite_sheets: &SpriteSheets,
		path: P,
	) -> Self {
		// Load region data from file.
		let file = File::open(path).expect("Could not open region file");
		let data: RegionData = from_reader(file).expect("Error in region file");

		let mut tiles: Vec<Entity> = Vec::with_capacity(data.terrain.len());
		let row_count = data.terrain.len() / data.col_count;
		for (i, terrain) in data.terrain.iter().enumerate() {
			// Compute row/column indices.
			let (row, col) = (i / data.col_count, i % data.col_count);
			// Set transform.
			let tile_translation = Transform::from_translation(Vec3::new(
				col as f32 * 2.0 * TILE_SIZE,
				row as f32 * -2.0 * TILE_SIZE,
				0.0,
			));
			let tile_scale = Transform::from_scale(Vec3::new(2.0, 2.0, 1.0));
			let tile_transform = tile_translation.mul_transform(tile_scale);
			// Add the tile to the world and the region's tile list, and track
			// its collisions.
			let tile = commands
				.spawn_bundle(SpriteSheetBundle {
					sprite: TextureAtlasSprite {
						index: *terrain as usize,
						..Default::default()
					},
					texture_atlas: sprite_sheets.terrain.clone(),
					transform: tile_transform,
					..Default::default()
				})
				.insert(*terrain)
				.id();
			tiles.push(tile);
		}

		// Generate hearts.
		for heart_location in data.heart_locations {
			let heart_position: Position = heart_location.into();
			let heart_collider = RectangleCollider {
				half_width: 0.5 * HEART_WIDTH,
				half_height: 0.5 * HEART_HEIGHT,
			};
			let heart_animation = SimpleAnimation::new(vec![SimpleFrame {
				sprite_index: 1,
				duration: None,
			}]);
			commands
				.spawn_bundle((
					Heart,
					heart_position,
					Direction::Down,
					heart_collider,
					heart_animation,
				))
				.insert_bundle(SpriteSheetBundle {
					texture_atlas: sprite_sheets.hearts.clone(),
					..Default::default()
				});
		}

		Self {
			row_count,
			col_count: data.col_count,
			tiles,
			entrances: data.entrances,
			exits: data.exits,
		}
	}

	/// The entrances into this region.
	pub fn entrances(&self) -> &[Entrance] {
		&self.entrances
	}

	/// The exits from this region.
	pub fn exits(&self) -> &[Exit] {
		&self.exits
	}

	/// Gets the tile at the given `row` and `col`, if any.
	pub fn tile_at_tile_coords(
		&self,
		tile_coords: TileCoords,
	) -> Option<Entity> {
		// Ensure coordinates are in bounds.
		if tile_coords.row >= self.row_count
			|| tile_coords.col >= self.col_count
		{
			return None;
		}
		// Compute index.
		let index = tile_coords.row * self.col_count + tile_coords.col;
		// Get terrain.
		self.tiles.get(index).cloned()
	}

	/// Gets the tile at `position`, if any.
	pub fn tile_at_position(&self, position: Position) -> Option<Entity> {
		Option::<TileCoords>::from(position)
			.and_then(|tile_coords| self.tile_at_tile_coords(tile_coords))
	}
}