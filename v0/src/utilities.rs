use crate::{physical_quantities};

/// Determines whether a given point is within a given radius of another point.
///
/// # Arguments
/// * `point` - The point that may fall within or outside of some radius.
/// * `radius` - The radius that the point may or may not fall within.
/// * `center_of_radius` - The point on which the radius is centered.
///
/// # Notes
/// * The absolute value of `radius` will be used for all comparisons. I.e., a
///		negative radius will return results identical to a positive radius of
///		the same magnitude.
/// * A point that falls on the border of the radius will be considered to be
///		within the radius. E.g., if `point` = (0.0, 10.0), `radius` = 10.0, and
///		`center_of_radius` = (0.0, 0.0), then `point` will be considered within
///		the radius and the function will return `true`.
// TODO: Consider making this a method of Displacement.
pub fn is_within_radius(
	point: physical_quantities::Displacement,
	radius: f64,
	center_of_radius: physical_quantities::Displacement,
) -> bool {
	// distance = sqrt((x2 - x1)^2 + (y2 - y1)^2)
	let distance =
		(
			(point.x() - center_of_radius.x()).powf(2.0)
			+ (point.y() - center_of_radius.y()).powf(2.0)
		).sqrt();
	
	distance <= radius.abs()
}
