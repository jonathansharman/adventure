use crate::{
	component::{
		animation::{DirectedAnimation, DirectedFrame},
		collider::RectangleCollider,
		spatial_bundle, Direction, Health, Hero, Layer, Velocity,
	},
	constants::*,
	resource::{Region, SpriteSheets},
};

use bevy::prelude::*;
use bevy_pixel_camera::PixelCameraBundle;

pub fn setup(
	mut commands: Commands,
	asset_server: Res<AssetServer>,
	mut texture_atlases: ResMut<Assets<TextureAtlas>>,
) {
	let sprite_sheets =
		SpriteSheets::new(asset_server.as_ref(), texture_atlases.as_mut());

	commands.spawn(PixelCameraBundle::from_zoom(2));

	commands
		.spawn((
			Hero,
			Health::new(HERO_BASE_HEALTH),
			Velocity::zero(),
			Direction::Down,
			RectangleCollider {
				half_width: 0.5 * TILE_SIZE,
				half_height: 0.5 * TILE_SIZE,
			},
			DirectedAnimation::new(
				Direction::Down,
				vec![DirectedFrame {
					up: 0,
					down: 1,
					left: 2,
					right: 3,
					duration: None,
				}],
			),
		))
		.insert(SpriteSheetBundle {
			texture_atlas: sprite_sheets.character.clone(),
			..Default::default()
		})
		.insert(spatial_bundle(TILE_SIZE, -TILE_SIZE, Layer::Front));

	let region =
		Region::load(&mut commands, &sprite_sheets, "assets/regions/test.ron");
	commands.insert_resource(region);

	commands.insert_resource(sprite_sheets);
}
