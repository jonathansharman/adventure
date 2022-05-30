use crate::{
	component::{
		animation::{DirectedAnimation, DirectedFrame},
		collider::RectangleCollider,
		Direction, Faction, Health, Hero, Position, Terrain, Velocity,
	},
	constants::*,
	resource::{Region, SpriteSheets},
};

use bevy::prelude::*;

pub fn setup(
	mut commands: Commands,
	asset_server: Res<AssetServer>,
	mut texture_atlases: ResMut<Assets<TextureAtlas>>,
) {
	let sprite_sheets =
		SpriteSheets::new(asset_server.as_ref(), texture_atlases.as_mut());

	commands.spawn_bundle(OrthographicCameraBundle::new_2d());

	commands
		.spawn_bundle((
			Hero,
			Faction::Ally,
			Health::new(HERO_BASE_HEALTH),
			Position { x: 0.0, y: 0.0 },
			Velocity::zero(),
			Direction::Down,
			RectangleCollider {
				half_width: 0.5 * TILE_SIZE,
				half_height: 0.5 * TILE_SIZE,
			},
			DirectedAnimation::new(vec![DirectedFrame {
				up: 0,
				down: 1,
				left: 2,
				right: 3,
				duration: None,
			}]),
		))
		.insert_bundle(SpriteSheetBundle {
			texture_atlas: sprite_sheets.character.clone(),
			..Default::default()
		});

	let region =
		Region::load(&mut commands, &sprite_sheets, "assets/regions/test.ron");
	commands.insert_resource(region);

	commands.insert_resource(sprite_sheets);
}
