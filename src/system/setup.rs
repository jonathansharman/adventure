use crate::{
	component::{
		animation::{DirectedAnimation, DirectedFrame},
		Direction, Hero, Position,
	},
	resource::SpriteSheets,
};

use bevy::prelude::*;

pub fn setup(
	mut commands: Commands,
	asset_server: Res<AssetServer>,
	mut texture_atlases: ResMut<Assets<TextureAtlas>>,
) {
	let sprite_sheets = SpriteSheets::new(asset_server.as_ref(), texture_atlases.as_mut());

	commands.spawn_bundle(OrthographicCameraBundle::new_2d());
	commands
		.spawn_bundle((
			Hero,
			Position { x: 0.0, y: 0.0 },
			Direction::Down,
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

	commands.insert_resource(sprite_sheets);
}
