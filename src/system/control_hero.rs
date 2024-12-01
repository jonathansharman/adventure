use crate::{
	component::{
		animation::{DirectedAnimation, DirectedFrame},
		Direction, Hero, KnockedBack, SlashAttack, SlashSword, ThrustAttack,
		ThrustSword,
	},
	constants::*,
	resource::SpriteSheets,
};

use bevy::prelude::*;
use bevy_xpbd_2d::prelude::*;

/// Controls the hero character based on player input.
pub fn control_hero(
	mut commands: Commands,
	asset_server: Res<AssetServer>,
	mut atlases: ResMut<Assets<TextureAtlasLayout>>,
	input: Res<ButtonInput<KeyCode>>,
	mut query: Query<
		(Entity, &mut LinearVelocity, &mut Direction),
		(
			With<Hero>,
			// Controls are disabled while knocked back or attacking.
			Without<KnockedBack>,
			Without<SlashAttack>,
			Without<ThrustAttack>,
		),
	>,
) {
	for (hero_id, mut velocity, mut direction) in query.iter_mut() {
		// Movement
		let mut vx: i16 = 0;
		let mut vy: i16 = 0;
		if input.pressed(KeyCode::ArrowUp) {
			vy += 1;
		}
		if input.pressed(KeyCode::ArrowDown) {
			vy -= 1;
		}
		if input.pressed(KeyCode::ArrowLeft) {
			vx -= 1;
		}
		if input.pressed(KeyCode::ArrowRight) {
			vx += 1;
		}
		// Update direction if needed. There are eight directions of
		// movement but only four for animation.
		if !input.pressed(KeyCode::Space) {
			// Not strafing.
			match (*direction, vx, vy) {
				// Moving in a cardinal direction.
				(_, 0, -1) => *direction = Direction::Down,
				(_, 0, 1) => *direction = Direction::Up,
				(_, -1, 0) => *direction = Direction::Left,
				(_, 1, 0) => *direction = Direction::Right,
				// Facing up but moving down-left, down, or down-right.
				(Direction::Up, _, -1) => *direction = Direction::Down,
				// Facing down but moving up-left, up, or up-right.
				(Direction::Down, _, 1) => *direction = Direction::Up,
				// Facing right but moving up-left, left, or down-left.
				(Direction::Right, -1, _) => *direction = Direction::Left,
				// Facing left but moving up-right, right, or down-right.
				(Direction::Left, 1, _) => *direction = Direction::Right,
				// Already facing in a reasonable direction.
				_ => {}
			};
		}
		// Update velocity.
		if vx == 0 || vy == 0 {
			velocity.x = vx as f32 * ORTHOGONAL_SPEED;
			velocity.y = vy as f32 * ORTHOGONAL_SPEED;
		} else {
			velocity.x = vx as f32 * DIAGONAL_SPEED;
			velocity.y = vy as f32 * DIAGONAL_SPEED;
		}
		// Check for sword attack.
		if input.just_pressed(KeyCode::KeyR) {
			let advancing = match *direction {
				Direction::Up => vy == 1,
				Direction::Down => vy == -1,
				Direction::Left => vx == -1,
				Direction::Right => vx == 1,
			};
			let sprite_sheets =
				SpriteSheets::new(asset_server.as_ref(), atlases.as_mut());
			if advancing {
				// Hero is advancing -> thrust attack.
				let sword_id = commands
					.spawn((
						ThrustSword,
						*direction,
						DirectedAnimation::new(
							*direction,
							vec![DirectedFrame {
								up: 0,
								down: 1,
								left: 2,
								right: 3,
								duration: None,
							}],
						),
						SpriteSheetBundle {
							texture: sprite_sheets.thrust_attack.image.clone(),
							atlas: TextureAtlas {
								layout: sprite_sheets
									.thrust_attack
									.layout
									.clone(),
								index: 0,
							},
							..Default::default()
						},
					))
					.id();
				commands
					.entity(hero_id)
					.insert(ThrustAttack::new(sword_id))
					.add_child(sword_id);
			} else {
				// Strafing/retreating/standing still -> slash attack.
				let sword_id = commands
					.spawn((
						SlashSword,
						*direction,
						DirectedAnimation::new(
							*direction,
							vec![DirectedFrame {
								up: 0,
								down: 1,
								left: 2,
								right: 3,
								duration: None,
							}],
						),
						SpriteSheetBundle {
							texture: sprite_sheets.thrust_attack.image.clone(),
							atlas: TextureAtlas {
								layout: sprite_sheets
									.thrust_attack
									.layout
									.clone(),
								index: 0,
							},
							..Default::default()
						},
					))
					.id();
				commands
					.entity(hero_id)
					.insert(SlashAttack::new(sword_id))
					.add_child(sword_id);
			}
		}
	}
}
