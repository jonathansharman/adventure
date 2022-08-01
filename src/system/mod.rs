mod animate;
mod control_camera;
mod control_hero;
mod handle_static_collisions;
mod move_entities;
mod setup;
mod update_children;

pub use {
	animate::{animate_directed, animate_simple},
	control_camera::control_camera,
	control_hero::control_hero,
	handle_static_collisions::handle_static_collisions,
	move_entities::move_entities,
	setup::setup,
	update_children::update_children,
};
