use crate::{
	component::{
		animation::{DirectedAnimation, DirectedFrame},
		Direction, Health, Hero, Layer,
	},
	constants::*,
	resource::{Region, SpriteSheets},
};

use bevy::prelude::*;
use bevy_pixel_camera::PixelCameraBundle;
use bevy_xpbd_2d::{math::Vector, prelude::*};

pub fn setup(
	mut commands: Commands,
	asset_server: Res<AssetServer>,
	mut texture_atlases: ResMut<Assets<TextureAtlas>>,
) {
	let sprite_sheets =
		SpriteSheets::new(asset_server.as_ref(), texture_atlases.as_mut());

	commands.spawn(PixelCameraBundle::from_zoom(2));

	commands.spawn((
		Hero,
		Health::new(HERO_BASE_HEALTH),
		RigidBody::Dynamic,
		Position(Vector::new(TILE_SIZE, -TILE_SIZE)),
		Collider::cuboid(TILE_SIZE, TILE_SIZE),
		Friction::ZERO,
		LockedAxes::new().lock_rotation(),
		Direction::Down,
		Layer::Front,
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
		SpriteSheetBundle {
			texture_atlas: sprite_sheets.character.clone(),
			..Default::default()
		},
	));

	let region =
		Region::load(&mut commands, &sprite_sheets, "assets/regions/test.ron");
	commands.insert_resource(region);

	commands.insert_resource(sprite_sheets);
}
