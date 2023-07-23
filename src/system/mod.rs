mod animate;
mod control_camera;
mod control_hero;
mod setup;
mod update_children;

pub use {
	animate::{animate_directed, animate_simple},
	control_camera::control_camera,
	control_hero::control_hero,
	setup::setup,
	update_children::update_children,
};
