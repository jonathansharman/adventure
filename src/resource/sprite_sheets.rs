use bevy::{prelude::*, sprite::Rect};

/// Container for all the game's sprite sheets.
#[derive(Debug)]
pub struct SpriteSheets {
	pub character: Handle<TextureAtlas>,
	pub slash_attack: Handle<TextureAtlas>,
	pub thrust_attack: Handle<TextureAtlas>,
	pub terrain: Handle<TextureAtlas>,
	pub hearts: Handle<TextureAtlas>,
	pub arrow_attack: Handle<TextureAtlas>,
	pub shield: Handle<TextureAtlas>,
}

type Size = bevy::math::Size<usize>;

impl SpriteSheets {
	pub fn new(
		asset_server: &AssetServer,
		texture_atlases: &mut Assets<TextureAtlas>,
	) -> Self {
		Self {
			character: from_grid(
				asset_server,
				texture_atlases,
				"sprites/character.png",
				Size::new(20, 20),
				Size::new(4, 1),
			),
			slash_attack: from_cels(
				asset_server,
				texture_atlases,
				"sprites/slash_attack.png",
				Size::new(80, 40),
				&[
					Rect {
						min: Vec2::new(0.0, 0.0),
						max: Vec2::new(40.0, 20.0),
					},
					Rect {
						min: Vec2::new(0.0, 20.0),
						max: Vec2::new(40.0, 40.0),
					},
					Rect {
						min: Vec2::new(40.0, 0.0),
						max: Vec2::new(60.0, 40.0),
					},
					Rect {
						min: Vec2::new(60.0, 0.0),
						max: Vec2::new(80.0, 40.0),
					},
				],
			),
			thrust_attack: from_grid(
				asset_server,
				texture_atlases,
				"sprites/thrust_attack.png",
				Size::new(20, 20),
				Size::new(4, 1),
			),
			terrain: from_grid(
				asset_server,
				texture_atlases,
				"sprites/terrain.png",
				Size::new(20, 20),
				Size::new(4, 3),
			),
			hearts: from_grid(
				asset_server,
				texture_atlases,
				"sprites/hearts.png",
				Size::new(13, 12),
				Size::new(3, 1),
			),
			arrow_attack: from_grid(
				asset_server,
				texture_atlases,
				"sprites/arrow_attack.png",
				Size::new(20, 20),
				Size::new(4, 1),
			),
			shield: from_cels(
				asset_server,
				texture_atlases,
				"sprites/shield.png",
				Size::new(32, 20),
				&[
					Rect {
						min: Vec2::new(0.0, 0.0),
						max: Vec2::new(20.0, 6.0),
					},
					Rect {
						min: Vec2::new(0.0, 6.0),
						max: Vec2::new(20.0, 12.0),
					},
					Rect {
						min: Vec2::new(20.0, 0.0),
						max: Vec2::new(26.0, 20.0),
					},
					Rect {
						min: Vec2::new(26.0, 0.0),
						max: Vec2::new(32.0, 20.0),
					},
				],
			),
		}
	}
}

fn from_grid(
	asset_server: &AssetServer,
	texture_atlases: &mut Assets<TextureAtlas>,
	path: &str,
	cel_size: Size,
	grid_size: Size,
) -> Handle<TextureAtlas> {
	texture_atlases.add(TextureAtlas::from_grid(
		asset_server.load(path),
		Vec2::new(cel_size.width as f32, cel_size.height as f32),
		grid_size.width,
		grid_size.height,
	))
}

fn from_cels(
	asset_server: &AssetServer,
	texture_atlases: &mut Assets<TextureAtlas>,
	path: &str,
	texture_size: Size,
	cels: &[Rect],
) -> Handle<TextureAtlas> {
	let mut texture_atlas = TextureAtlas::new_empty(
		asset_server.load(path),
		Vec2::new(texture_size.width as f32, texture_size.height as f32),
	);
	for cel in cels {
		texture_atlas.add_texture(*cel);
	}
	texture_atlases.add(texture_atlas)
}
