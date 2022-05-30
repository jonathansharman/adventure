mod animate;
mod control_camera;
mod control_hero;
mod handle_static_collisions;
mod move_entities;
mod setup;

pub use {
	animate::{animate_directional, animate_simple},
	control_camera::control_camera,
	control_hero::control_hero,
	handle_static_collisions::handle_static_collisions,
	move_entities::move_entities,
	setup::setup,
};
