use crate::component::Direction;

use bevy::prelude::*;

/// Updates children with respect to their parents.
pub fn update_children(
	parent_query: Query<(&Children, &Direction)>,
	// TODO: Without<Children> is here to ensure the two queries are disjoint
	// since they would otherwise conflict due to the mutable access of
	// Direction. This assumes second-order children will never exist (which
	// might be a reasonable assumption). Can this function be reasonably
	// written such that concurrent parent/child direction access is not
	// required?
	mut child_query: Query<&mut Direction, Without<Children>>,
) {
	// TODO: Set the child's (relative) translation according to the parent
	// animation's hotspots. (Animation hotspots are not yet implemented.)
	for (children, parent_direction) in parent_query.iter() {
		for child in children.iter() {
			if let Ok(mut child_direction) = child_query.get_mut(*child) {
				*child_direction = *parent_direction;
			}
		}
	}
}
