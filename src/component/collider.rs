use crate::component::{Direction, Position};

use bevy::prelude::Component;

/// Axis-aligned rectangle collider.
#[derive(Component)]
pub struct RectangleCollider {
	pub half_width: f32,
	pub half_height: f32,
}

/// Axis-aligned half-disk collider.
#[derive(Component)]
pub struct HalfDiskCollider {
	pub radius: f32,
}

/// Determines the area of intersection between two rectangle colliders.
pub fn rect_rect_intersection_area(
	first: (&RectangleCollider, &Position),
	second: (&RectangleCollider, &Position),
) -> f32 {
	let (collider1, pos1) = first;
	let (collider2, pos2) = second;
	let min_right =
		(pos1.x + collider1.half_width).min(pos2.x + collider2.half_width);
	let max_left =
		(pos1.x - collider1.half_width).max(pos2.x - collider2.half_width);
	let min_top =
		(pos1.y + collider1.half_height).min(pos2.y + collider2.half_height);
	let max_bottom =
		(pos1.y - collider1.half_height).max(pos2.y - collider2.half_height);
	(min_right - max_left).max(0.0) * (min_top - max_bottom).max(0.0)
}

/// Determines whether `rectangle` and `half_disk` intersect.
pub fn rect_intersects_half_disk(
	rectangle: (&RectangleCollider, &Position),
	half_disk: (&HalfDiskCollider, &Position, &Direction),
) -> bool {
	let (rect_collider, rect_pos) = rectangle;
	let (half_disk_collider, disk_pos, disk_direction) = half_disk;
	// Find the point inside the rectangle that is closest to the disk's center.
	let closest = Position {
		x: disk_pos
			.x
			.max(rect_pos.x - rect_collider.half_width)
			.min(rect_pos.x + rect_collider.half_width),
		y: disk_pos
			.y
			.max(rect_pos.y - rect_collider.half_height)
			.min(rect_pos.y + rect_collider.half_height),
	};
	// If the closest point is in a different half plane than the half disk,
	// there is no intersection.
	if match disk_direction {
		Direction::Up => closest.y < disk_pos.y,
		Direction::Down => closest.y > disk_pos.y,
		Direction::Left => closest.x > disk_pos.x,
		Direction::Right => closest.x < disk_pos.x,
	} {
		return false;
	}
	// There's an intersection iff the closest point is within the disk.
	(closest.x - disk_pos.x).powi(2) + (closest.y - disk_pos.y).powi(2)
		<= half_disk_collider.radius.powi(2)
}
