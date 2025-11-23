use crate::{physical_quantities};

/// Determines whether a given point is within a given radius of another point.
///
/// # Arguments
/// * `point` - The point that may fall within or outside of some radius.
/// * `radius` - The radius that the point may or may not fall within.
/// * `center_of_radius` - The point on which the radius is centered.
/// * `use_strict_equality` - If `true`, we will consider a point on the
///		circle's border to be outside the radius. Otherwise, we will consider a
///		point on the circle's border to be within the radius.
///
/// # Notes
/// * The absolute value of `radius` will be used for all comparisons. I.e., a
///		negative radius will return results identical to a positive radius of
///		the same magnitude.
/// * If `use_strict_inequality` is `false` a point that falls on the border of the
///		radius will be considered to be within the radius. E.g., if `point` =
///		(0.0, 10.0), `radius` = 10.0, and `center_of_radius` = (0.0, 0.0), then
///		`point` will be considered within the radius and the function will
///		return `true`. However, if `use_strict_equality` is `true`, this same
///		example point will be considered outside the radius and the function
///		will return `false`.
// TODO: Consider making this a method of Displacement.
pub fn is_within_radius(
	point: physical_quantities::Displacement,
	radius: f64,
	center_of_radius: physical_quantities::Displacement,
	use_strict_inequality: bool,
) -> bool {
	// distance = sqrt((x2 - x1)^2 + (y2 - y1)^2)
	let distance = measure_distance(point, center_of_radius);
	if use_strict_inequality {
		return distance < radius.abs()
	} else {
		return distance <= radius.abs()
	}
}

/// Gets the distance between two points (`Displacements`) as an `f64`.
///
/// # Arguments
/// * `point1` - One point.
/// * `point2` - The other point.
pub fn measure_distance(
	point1: physical_quantities::Displacement,
	point2: physical_quantities::Displacement,
) -> f64 {
	(
		(point1.x() - point2.x()).powf(2.0)
		+ (point1.y() - point2.y()).powf(2.0)
	).sqrt()
}

/// Gets the displacement vector from one point to another.
///
/// # Arguments
/// * `from` - The point from which we're measuring/displacing.
/// * `to` - The point to which the vector will point.
pub fn get_displacement_vector(
	from: physical_quantities::Displacement,
	to: physical_quantities::Displacement,
) -> physical_quantities::Displacement {
	to - from
}
