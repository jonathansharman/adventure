use crate::component::Direction;

use bevy::prelude::Component;

/// An animation that can use different sprites for each cardinal direction.
#[derive(Clone, PartialEq, Component)]
pub struct DirectedAnimation {
	frames: Vec<DirectedFrame>,
	frame_index: usize,
	frame_progress: u32,
	direction: Direction,
}

impl DirectedAnimation {
	pub fn new(direction: Direction, frames: Vec<DirectedFrame>) -> Self {
		Self {
			frames,
			frame_index: 0,
			frame_progress: 0,
			direction,
		}
	}

	pub fn sprite_index(&self) -> usize {
		let frame = self.frames[self.frame_index];
		match self.direction {
			Direction::Up => frame.up,
			Direction::Down => frame.down,
			Direction::Left => frame.left,
			Direction::Right => frame.right,
		}
	}

	/// Advances the animation by one game frame.
	pub fn advance(&mut self) {
		if let Some(frame_duration) = self.frames[self.frame_index].duration {
			self.frame_progress += 1;
			if self.frame_progress == frame_duration {
				self.frame_progress = 0;
				self.frame_index = (self.frame_index + 1) % self.frames.len();
			}
		}
	}

	pub fn set_direction(&mut self, direction: Direction) {
		self.direction = direction;
	}
}

#[derive(Clone, Copy, PartialEq, Eq)]
pub struct DirectedFrame {
	/// Sprite number for the up direction.
	pub up: usize,
	/// Sprite number for the down direction.
	pub down: usize,
	/// Sprite number for the left direction.
	pub left: usize,
	/// Sprite number for the right direction.
	pub right: usize,
	/// Duration of this animation frame, in game frames. If `None`, the
	/// animation stops on this frame.
	pub duration: Option<u32>,
}
