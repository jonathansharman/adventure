use bevy::prelude::*;

pub struct SpriteSheet {
	pub image: Handle<Image>,
	pub layout: Handle<TextureAtlasLayout>,
}

/// Container for all the game's sprite sheets.
#[derive(Resource)]
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
				UVec2::new(20, 20),
				UVec2::new(4, 1),
			),
			slash_attack: from_cels(
				asset_server,
				atlases,
				"sprites/slash_attack.png",
				UVec2::new(80, 40),
				&[
					URect {
						min: UVec2::new(0, 0),
						max: UVec2::new(40, 20),
					},
					URect {
						min: UVec2::new(0, 20),
						max: UVec2::new(40, 40),
					},
					URect {
						min: UVec2::new(40, 0),
						max: UVec2::new(60, 40),
					},
					URect {
						min: UVec2::new(60, 0),
						max: UVec2::new(80, 40),
					},
				],
			),
			thrust_attack: from_grid(
				asset_server,
				atlases,
				"sprites/thrust_attack.png",
				UVec2::new(20, 20),
				UVec2::new(4, 1),
			),
			terrain: from_grid(
				asset_server,
				atlases,
				"sprites/terrain.png",
				UVec2::new(20, 20),
				UVec2::new(4, 3),
			),
			hearts: from_grid(
				asset_server,
				atlases,
				"sprites/hearts.png",
				UVec2::new(13, 12),
				UVec2::new(3, 1),
			),
			arrow_attack: from_grid(
				asset_server,
				atlases,
				"sprites/arrow_attack.png",
				UVec2::new(20, 20),
				UVec2::new(4, 1),
			),
			shield: from_cels(
				asset_server,
				atlases,
				"sprites/shield.png",
				UVec2::new(32, 20),
				&[
					URect {
						min: UVec2::new(0, 0),
						max: UVec2::new(20, 6),
					},
					URect {
						min: UVec2::new(0, 6),
						max: UVec2::new(20, 12),
					},
					URect {
						min: UVec2::new(20, 0),
						max: UVec2::new(26, 20),
					},
					URect {
						min: UVec2::new(26, 0),
						max: UVec2::new(32, 20),
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
	cel_size: UVec2,
	grid_size: UVec2,
) -> SpriteSheet {
	SpriteSheet {
		image: asset_server.load(path),
		layout: atlases.add(TextureAtlasLayout::from_grid(
			cel_size,
			grid_size.x,
			grid_size.y,
			None,
			None,
		)),
	}
}

fn from_cels(
	asset_server: &AssetServer,
	atlases: &mut Assets<TextureAtlasLayout>,
	path: &'static str,
	texture_size: UVec2,
	cels: &[URect],
) -> SpriteSheet {
	let mut atlas = TextureAtlasLayout::new_empty(texture_size);
	for cel in cels {
		atlas.add_texture(*cel);
	}
	SpriteSheet {
		image: asset_server.load(path),
		layout: atlases.add(atlas),
	}
}
