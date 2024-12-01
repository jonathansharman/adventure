use crate::{
	component::{
		animation::{DirectedAnimation, DirectedFrame},
		Direction, Health, Hero,
	},
	constants::*,
	resource::{Region, SpriteSheets},
};

use bevy::prelude::*;
use bevy_pixel_camera::PixelZoom;
use bevy_xpbd_2d::{math::Vector, prelude::*};

pub fn setup(
	mut commands: Commands,
	asset_server: Res<AssetServer>,
	mut atlases: ResMut<Assets<TextureAtlasLayout>>,
) {
	let sprite_sheets =
		SpriteSheets::new(asset_server.as_ref(), atlases.as_mut());

	commands.spawn((Camera2dBundle::default(), PixelZoom::Fixed(2)));

	commands.spawn((
		Hero,
		Health::new(HERO_BASE_HEALTH),
		RigidBody::Dynamic,
		Position(Vector::new(TILE_SIZE, -TILE_SIZE)),
		Collider::rectangle(TILE_SIZE, TILE_SIZE),
		Friction::ZERO,
		LockedAxes::new().lock_rotation(),
		Direction::Down,
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
			texture: sprite_sheets.character.image.clone(),
			atlas: TextureAtlas {
				layout: sprite_sheets.character.layout.clone(),
				index: 0,
			},
			transform: Transform::from_translation(Z_FRONT * Vec3::Z),
			..Default::default()
		},
	));

	let region =
		Region::load(&mut commands, &sprite_sheets, "assets/regions/test.ron");
	commands.insert_resource(region);

	commands.insert_resource(sprite_sheets);
}
