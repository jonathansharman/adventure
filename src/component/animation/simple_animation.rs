use bevy::prelude::*;

/// A basic animation. See also `DirectedAnimation`.
#[derive(Clone, PartialEq, Component)]
pub struct SimpleAnimation {
	frames: Vec<SimpleFrame>,
	frame_index: usize,
	frame_progress: u32,
}

impl SimpleAnimation {
	pub fn new(frames: Vec<SimpleFrame>) -> Self {
		Self {
			frames,
			frame_index: 0,
			frame_progress: 0,
		}
	}

	pub fn sprite_index(&self) -> usize {
		self.frames[self.frame_index].sprite_index
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
}

#[derive(Clone, Copy, PartialEq, Eq)]
pub struct SimpleFrame {
	pub sprite_index: usize,
	/// Duration of this animation frame, in game frames. If `None`, the
	/// animation stops on this frame.
	pub duration: Option<u32>,
}
