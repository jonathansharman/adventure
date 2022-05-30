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
		}
	}
}
