use crate::{
	component::{
		animation::{DirectedAnimation, DirectedFrame},
		Direction, Health, Hero,
	},
	constants::*,
	resource::{Region, SpriteSheets},
};

use avian2d::{math::Vector, prelude::*};
use bevy::prelude::*;

pub fn setup(
	mut commands: Commands,
	asset_server: Res<AssetServer>,
	mut atlases: ResMut<Assets<TextureAtlasLayout>>,
) {
	let sprite_sheets =
		SpriteSheets::new(asset_server.as_ref(), atlases.as_mut());

	commands.spawn((Camera2d, Msaa::Off));

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
		Transform::from_translation(Z_FRONT * Vec3::Z),
		Sprite {
			image: sprite_sheets.character.image.clone(),
			texture_atlas: Some(TextureAtlas {
				layout: sprite_sheets.character.layout.clone(),
				index: 0,
			}),
			..Default::default()
		},
	));

	let region =
		Region::load(&mut commands, &sprite_sheets, "assets/regions/test.ron");
	commands.insert_resource(region);

	commands.insert_resource(sprite_sheets);
}
