use crate::component::Hero;

use bevy::{prelude::*, render::camera::Camera};

/// Causes the camera to follow the hero.
pub fn control_camera(
	mut camera_query: Query<&mut Transform, (With<Camera>, Without<Hero>)>,
	hero_query: Query<&Transform, (With<Hero>, Without<Camera>)>,
) {
	if let Ok(&hero_translation) = hero_query.get_single() {
		for mut camera_transform in camera_query.iter_mut() {
			camera_transform.translation = hero_translation.translation;
			// TODO: Rounding the camera position ensures background tiles
			// remain aligned (and don't become blurry) but creates a weird
			// jittering effect on the hero while it moves. Need a more robust
			// pixel-aligned camera solution.
			camera_transform.translation.x =
				camera_transform.translation.x.round();
			camera_transform.translation.y =
				camera_transform.translation.y.round();
		}
	}
}
