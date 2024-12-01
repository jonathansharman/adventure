use bevy::prelude::*;

#[derive(Debug)]
pub struct SpriteSheet {
	pub image: Handle<Image>,
	pub layout: Handle<TextureAtlasLayout>,
}

/// Container for all the game's sprite sheets.
#[derive(Resource, Debug)]
pub struct SpriteSheets {
	pub character: SpriteSheet,
	pub slash_attack: SpriteSheet,
	pub thrust_attack: SpriteSheet,
	pub terrain: SpriteSheet,
	pub hearts: SpriteSheet,
	pub arrow_attack: SpriteSheet,
	pub shield: SpriteSheet,
}

impl SpriteSheets {
	pub fn new(
		asset_server: &AssetServer,
		atlases: &mut Assets<TextureAtlasLayout>,
	) -> Self {
		Self {
			character: from_grid(
				asset_server,
				atlases,
				"sprites/character.png",
				(20, 20),
				(4, 1),
			),
			slash_attack: from_cels(
				asset_server,
				atlases,
				"sprites/slash_attack.png",
				(80, 40),
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
				atlases,
				"sprites/thrust_attack.png",
				(20, 20),
				(4, 1),
			),
			terrain: from_grid(
				asset_server,
				atlases,
				"sprites/terrain.png",
				(20, 20),
				(4, 3),
			),
			hearts: from_grid(
				asset_server,
				atlases,
				"sprites/hearts.png",
				(13, 12),
				(3, 1),
			),
			arrow_attack: from_grid(
				asset_server,
				atlases,
				"sprites/arrow_attack.png",
				(20, 20),
				(4, 1),
			),
			shield: from_cels(
				asset_server,
				atlases,
				"sprites/shield.png",
				(32, 20),
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
	atlases: &mut Assets<TextureAtlasLayout>,
	path: &'static str,
	cel_size: (usize, usize),
	grid_size: (usize, usize),
) -> SpriteSheet {
	SpriteSheet {
		image: asset_server.load(path),
		layout: atlases.add(TextureAtlasLayout::from_grid(
			Vec2::new(cel_size.0 as f32, cel_size.1 as f32),
			grid_size.0,
			grid_size.1,
			None,
			None,
		)),
	}
}

fn from_cels(
	asset_server: &AssetServer,
	atlases: &mut Assets<TextureAtlasLayout>,
	path: &'static str,
	texture_size: (usize, usize),
	cels: &[Rect],
) -> SpriteSheet {
	let mut atlas = TextureAtlasLayout::new_empty(Vec2::new(
		texture_size.0 as f32,
		texture_size.1 as f32,
	));
	for cel in cels {
		atlas.add_texture(*cel);
	}
	SpriteSheet {
		image: asset_server.load(path),
		layout: atlases.add(atlas),
	}
}
