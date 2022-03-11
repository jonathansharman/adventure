mod animate;
mod control_hero;
mod move_entities;
mod setup;

pub use {
	animate::{animate_directional, animate_simple},
	control_hero::control_hero,
	move_entities::move_entities,
	setup::setup,
};
