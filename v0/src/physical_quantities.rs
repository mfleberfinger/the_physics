#[cfg(test)]
mod tests {
    use super::*;

	/********************* Time ********************/

	#[test]
	fn time_supports_partialEq() {
		assert!(
			Time(-1.0) == Time(-1.0),
			"Time(-1.0) did not equal Time(-1.0)."
		);
		assert!(
			Time(0.0) == Time(0.0),
			"Time(0.0) did not equal Time(0.0)."
		);
		assert!(
			Time(1.0) == Time(1.0),
			"Time(1.0) did not equal Time(1.0)."
		);

		assert!(
			Time(-1.0) != Time(0.0),
			"Time(-1.0) was equal Time(0.0)."
		);
		assert!(
			Time(0.0) != Time(1.0),
			"Time(0.0) was equal to Time(1.0)."
		);
		assert!(
			Time(1.0) != Time(-1.0),
			"Time(1.0) was equal to Time(-1.0)."
		);
	}

	#[test]
	fn time_supports_multiplication_by_a_coefficient() {
		assert_eq!(Time(2.0) * 5.0, Time(10.0));
		assert_eq!(Time(-2.0) * 5.0, Time(-10.0));
	}

	#[test]
	fn time_supports_addition() {
        assert_eq!(Time(0.0), Time(0.0) + Time(0.0));
        assert_eq!(Time(1.0), Time(0.0) + Time(1.0));
        assert_eq!(Time(-1.0), Time(0.0) + Time(-1.0));
        assert_eq!(Time(1.0), Time(1.0) + Time(0.0));
        assert_eq!(Time(2.0), Time(1.0) + Time(1.0));
        assert_eq!(Time(0.0), Time(1.0) + Time(-1.0));
        assert_eq!(Time(-1.0), Time(-1.0) + Time(0.0));
        assert_eq!(Time(0.0), Time(-1.0) + Time(1.0));
        assert_eq!(Time(-2.0), Time(-1.0) + Time(-1.0));
	}

	#[test]
	fn time_supports_subtraction() {
        assert_eq!(Time(0.0), Time(0.0) - Time(0.0));
        assert_eq!(Time(-1.0), Time(0.0) - Time(1.0));
        assert_eq!(Time(1.0), Time(0.0) - Time(-1.0));
        assert_eq!(Time(1.0), Time(1.0) - Time(0.0));
        assert_eq!(Time(0.0), Time(1.0) - Time(1.0));
        assert_eq!(Time(2.0), Time(1.0) - Time(-1.0));
        assert_eq!(Time(-1.0), Time(-1.0) - Time(0.0));
        assert_eq!(Time(-2.0), Time(-1.0) - Time(1.0));
        assert_eq!(Time(0.0), Time(-1.0) - Time(-1.0));
	}

	
	/********************* Vector2 ********************/

	#[test]
	fn new_creates_vector2() {
		let v = Vector2::new(-1.0, 1.0);
		assert_eq!(v.x, -1.0);
		assert_eq!(v.y, 1.0);
	}

	#[test]
	fn vector2_supports_partialEq() {
		// Test (-1, -1) == (-1, -1), (-1, -1) == (-1, 0),
		//	(-1, -1) == (-1, 1), ..., (1, 1,) == (1, 1).
		// There are nine combinations for each vector. 81 total test cases?
		// For each v1, generate and test each v2...
		for i1 in -1..2 {
			for j1 in -1..2 {
				let x1 = i1 as f64;
				let y1 = j1 as f64;
				let v1 = Vector2::new(x1, y1);
				for i2 in -1..2 {
					for j2 in -1..2 {
						let x2 = i2 as f64;
						let y2 = j2 as f64;
						let v2 = Vector2::new(x2, y2);
						if x1 == x2 && y1 == y2 {
							assert_eq!(v1, v2);
						} else {
							assert_ne!(v1, v2);
						}
					}
				}
			}
		}
	}

	#[test]
	fn vector2_supports_scalar_multiplication() {
		assert_eq!(Vector2::new(1.0, 2.0) * 5.0, Vector2::new(5.0, 10.0));
		assert_eq!(5.0 * Vector2::new(1.0, 2.0), Vector2::new(5.0, 10.0));
		assert_eq!(Vector2::new(1.0, 2.0) * (-5.0), Vector2::new(-5.0, -10.0));
		assert_eq!((-5.0) * Vector2::new(1.0, 2.0), Vector2::new(-5.0, -10.0));
	}

	#[test]
	fn vector2_supports_scalar_division() {
		assert_eq!(Vector2::new(5.0, 10.0) / 5.0, Vector2::new(1.0, 2.0));
		assert_eq!(Vector2::new(5.0, 10.0) / (-5.0), Vector2::new(-1.0, -2.0));
	}

	#[test]
	fn vector2_supports_addition() {
		assert_eq!(
			Vector2::new(0.0, 0.0),
			Vector2::new(0.0, 0.0) + Vector2::new(0.0, 0.0),
		);
		assert_eq!(
			Vector2::new(1.0, 0.0) + Vector2::new(0.0, 0.0),
			Vector2::new(1.0, 0.0),
		);
		assert_eq!(
			Vector2::new(0.0, 1.0) + Vector2::new(0.0, 0.0),
			Vector2::new(0.0, 1.0),
		);
		assert_eq!(
			Vector2::new(1.0, 1.0) + Vector2::new(1.0, 1.0),
			Vector2::new(2.0, 2.0),
		);
		assert_eq!(
			Vector2::new(1.0, 1.0) + Vector2::new(-1.0, -1.0),
			Vector2::new(0.0, 0.0),
		);
	}

	#[test]
	fn vector2_supports_subtraction() {
		assert_eq!(
			Vector2::new(0.0, 0.0),
			Vector2::new(0.0, 0.0) - Vector2::new(0.0, 0.0),
		);
		assert_eq!(
			Vector2::new(1.0, 0.0) - Vector2::new(0.0, 0.0),
			Vector2::new(1.0, 0.0),
		);
		assert_eq!(
			Vector2::new(0.0, 1.0) - Vector2::new(0.0, 0.0),
			Vector2::new(0.0, 1.0),
		);
		assert_eq!(
			Vector2::new(1.0, 1.0) - Vector2::new(1.0, 1.0),
			Vector2::new(0.0, 0.0),
		);
		assert_eq!(
			Vector2::new(1.0, 1.0) - Vector2::new(-1.0, -1.0),
			Vector2::new(2.0, 2.0),
		);
		assert_eq!(
			Vector2::new(-5.5, 2.5) - Vector2::new(10.0, 20.0),
			Vector2::new(-15.5, -17.5),
		);
	}

	/********************* Mass ********************/

	#[test]
	fn mass_supports_partialEq() {
		assert_eq!(Mass::new(0.1), Mass::new(0.1));
		assert_eq!(Mass::new(1.0), Mass::new(1.0));

		assert_ne!(Mass::new(1.0), Mass::new(0.1));
		assert_ne!(Mass::new(0.1), Mass::new(1.0));
	}

	#[test]
	#[should_panic(expected = "Mass must be positive.")]
	fn mass_new_panics_if_not_positive() {
		let m = Mass::new(0.0);
	}

    #[test]
    fn mass_supports_multiplication_by_acceleration() {
        assert_eq!(
            Force::new(1.0, 2.0),
            Mass::new(1.0) * Acceleration::new(1.0, 2.0),
        );
        assert_eq!(
            Force::new(1.0, -2.0),
            Mass::new(1.0) * Acceleration::new(1.0, -2.0),
        );
        assert_eq!(
            Force::new(-1.0, 2.0),
            Mass::new(1.0) * Acceleration::new(-1.0, 2.0),
        );
        assert_eq!(
            Force::new(-1.0, -2.0),
            Mass::new(1.0) * Acceleration::new(-1.0, -2.0),
        );
    }

	/********************* Displacement ********************/

	#[test]
	fn new_creates_displacement() {
		let displacement = Displacement::new(-1.0, 1.0);
		assert_eq!(displacement.0.x, -1.0);
		assert_eq!(displacement.0.y, 1.0);
	}

	#[test]
	fn displacement_gets_x_and_y() {
		let displacement = Displacement::new(-1.0, 1.0);
		assert_eq!(displacement.x(), displacement.0.x);
		assert_eq!(displacement.y(), displacement.0.y);
	}

	#[test]
	fn displacement_supports_partialEq() {
		// Test (-1, -1) == (-1, -1), (-1, -1) == (-1, 0),
		//	(-1, -1) == (-1, 1), ..., (1, 1,) == (1, 1).
		// There are nine combinations for each vector. 81 total test cases?
		// For each p1, generate and test each p2...
		for i1 in -1..2 {
			for j1 in -1..2 {
				let x1 = i1 as f64;
				let y1 = j1 as f64;
				let d1 = Displacement::new(x1, y1);
				for i2 in -1..2 {
					for j2 in -1..2 {
						let x2 = i2 as f64;
						let y2 = j2 as f64;
						let d2 = Displacement::new(x2, y2);
						if x1 == x2 && y1 == y2 {
							assert_eq!(d1, d2);
						} else {
							assert_ne!(d1, d2);
						}
					}
				}
			}
		}
	}

	#[test]
	fn displacement_supports_addition() {
		assert_eq!(
			Displacement::new(2.0, 3.0) + Displacement::new(3.0, 4.0),
			Displacement::new(5.0, 7.0),
		);
		assert_eq!(
			Displacement::new(-2.0, 3.0) + Displacement::new(3.0, -4.0),
			Displacement::new(1.0, -1.0),
		);
	}

	#[test]
	fn displacement_supports_add_assign() {
		let mut displacement = Displacement::new(-1.0, -2.0);
		displacement += Displacement::new(10.0, 20.0);
		assert_eq!(displacement, Displacement::new(9.0, 18.0));
		displacement += Displacement::new(-10.0, -20.0);
		assert_eq!(displacement, Displacement::new(-1.0, -2.0));
	}

	#[test]
	fn displacement_supports_subtraction() {
		assert_eq!(
			Displacement::new(0.0, 0.0),
			Displacement::new(0.0, 0.0) - Displacement::new(0.0, 0.0),
		);
		assert_eq!(
			Displacement::new(1.0, 0.0) - Displacement::new(0.0, 0.0),
			Displacement::new(1.0, 0.0),
		);
		assert_eq!(
			Displacement::new(0.0, 1.0) - Displacement::new(0.0, 0.0),
			Displacement::new(0.0, 1.0),
		);
		assert_eq!(
			Displacement::new(1.0, 1.0) - Displacement::new(1.0, 1.0),
			Displacement::new(0.0, 0.0),
		);
		assert_eq!(
			Displacement::new(1.0, 1.0) - Displacement::new(-1.0, -1.0),
			Displacement::new(2.0, 2.0),
		);
		assert_eq!(
			Displacement::new(-5.5, 2.5) - Displacement::new(10.0, 20.0),
			Displacement::new(-15.5, -17.5),
		);
	}

	/********************* Velocity ********************/

	#[test]
	fn new_creates_velocity() {
		let velocity = Velocity::new(-1.0, 1.0);
		assert_eq!(velocity.0.x, -1.0);
		assert_eq!(velocity.0.y, 1.0);
	}

	#[test]
	fn velocity_gets_x_and_y() {
		let velocity = Velocity::new(-1.0, 1.0);
		assert_eq!(velocity.x(), velocity.0.x);
		assert_eq!(velocity.y(), velocity.0.y);
	}

	#[test]
	fn velocity_supports_partialEq() {
		// Test (-1, -1) == (-1, -1), (-1, -1) == (-1, 0),
		//	(-1, -1) == (-1, 1), ..., (1, 1,) == (1, 1).
		// There are nine combinations for each vector. 81 total test cases?
		// For each p1, generate and test each p2...
		for i1 in -1..2 {
			for j1 in -1..2 {
				let x1 = i1 as f64;
				let y1 = j1 as f64;
				let v1 = Velocity::new(x1, y1);
				for i2 in -1..2 {
					for j2 in -1..2 {
						let x2 = i2 as f64;
						let y2 = j2 as f64;
						let v2 = Velocity::new(x2, y2);
						if x1 == x2 && y1 == y2 {
							assert_eq!(v1, v2);
						} else {
							assert_ne!(v1, v2);
						}
					}
				}
			}
		}
	}

	#[test]
	fn velocity_supports_multiplication_by_time() {
		assert_eq!(
			Velocity::new(1.0, 2.0) * Time(5.0),
			Displacement::new(5.0, 10.0)
		);
		assert_eq!(
			Time(5.0) * Velocity::new(1.0, 2.0),
			Displacement::new(5.0, 10.0)
		);
		assert_eq!(
			Velocity::new(1.0, 2.0) * Time(-5.0),
			Displacement::new(-5.0, -10.0)
		);
		assert_eq!(
			Time(-5.0) * Velocity::new(1.0, 2.0),
			Displacement::new(-5.0, -10.0)
		);
	}

	#[test]
	fn velocity_supports_addition() {
		assert_eq!(
			Velocity::new(0.0, 0.0),
			Velocity::new(0.0, 0.0) + Velocity::new(0.0, 0.0),
		);
		assert_eq!(
			Velocity::new(1.0, 0.0) + Velocity::new(0.0, 0.0),
			Velocity::new(1.0, 0.0),
		);
		assert_eq!(
			Velocity::new(0.0, 1.0) + Velocity::new(0.0, 0.0),
			Velocity::new(0.0, 1.0),
		);
		assert_eq!(
			Velocity::new(1.0, 1.0) + Velocity::new(1.0, 1.0),
			Velocity::new(2.0, 2.0),
		);
		assert_eq!(
			Velocity::new(1.0, 1.0) + Velocity::new(-1.0, -1.0),
			Velocity::new(0.0, 0.0),
		);
	}

	#[test]
	fn velocity_supports_subtraction() {
		assert_eq!(
			Velocity::new(0.0, 0.0),
			Velocity::new(0.0, 0.0) - Velocity::new(0.0, 0.0),
		);
		assert_eq!(
			Velocity::new(1.0, 0.0) - Velocity::new(0.0, 0.0),
			Velocity::new(1.0, 0.0),
		);
		assert_eq!(
			Velocity::new(0.0, 1.0) - Velocity::new(0.0, 0.0),
			Velocity::new(0.0, 1.0),
		);
		assert_eq!(
			Velocity::new(1.0, 1.0) - Velocity::new(1.0, 1.0),
			Velocity::new(0.0, 0.0),
		);
		assert_eq!(
			Velocity::new(1.0, 1.0) - Velocity::new(-1.0, -1.0),
			Velocity::new(2.0, 2.0),
		);
		assert_eq!(
			Velocity::new(-5.5, 2.5) - Velocity::new(10.0, 20.0),
			Velocity::new(-15.5, -17.5),
		);
	}

	/********************* Acceleration ********************/

	#[test]
	fn new_creates_acceleration() {
		let acceleration = Acceleration::new(-1.0, 1.0);
		assert_eq!(acceleration.0.x, -1.0);
		assert_eq!(acceleration.0.y, 1.0);
	}

	#[test]
	fn acceleration_gets_x_and_y() {
		let acceleration = Acceleration::new(-1.0, 1.0);
		assert_eq!(acceleration.x(), acceleration.0.x);
		assert_eq!(acceleration.y(), acceleration.0.y);
	}

	#[test]
	fn acceleration_supports_partialEq() {
		// Test (-1, -1) == (-1, -1), (-1, -1) == (-1, 0),
		//	(-1, -1) == (-1, 1), ..., (1, 1,) == (1, 1).
		// There are nine combinations for each vector. 81 total test cases?
		// For each p1, generate and test each p2...
		for i1 in -1..2 {
			for j1 in -1..2 {
				let x1 = i1 as f64;
				let y1 = j1 as f64;
				let a1 = Acceleration::new(x1, y1);
				for i2 in -1..2 {
					for j2 in -1..2 {
						let x2 = i2 as f64;
						let y2 = j2 as f64;
						let a2 = Acceleration::new(x2, y2);
						if x1 == x2 && y1 == y2 {
							assert_eq!(a1, a2);
						} else {
							assert_ne!(a1, a2);
						}
					}
				}
			}
		}
	}

	#[test]
	fn acceleration_supports_scalar_multiplication() {
		assert_eq!(
			Acceleration::new(1.0, 2.0) * 5.0,
			Acceleration::new(5.0, 10.0)
		);
		assert_eq!(
			5.0 * Acceleration::new(1.0, 2.0),
			Acceleration::new(5.0, 10.0)
		);
		assert_eq!(
			Acceleration::new(1.0, 2.0) * (-5.0),
			Acceleration::new(-5.0, -10.0)
		);
		assert_eq!(
			(-5.0) * Acceleration::new(1.0, 2.0),
			Acceleration::new(-5.0, -10.0)
		);
	}

	#[test]
	fn acceleration_supports_multiplication_by_time() {
		assert_eq!(
			Acceleration::new(1.0, 2.0) * Time(5.0),
			Velocity::new(5.0, 10.0)
		);
		assert_eq!(
			Time(5.0) * Acceleration::new(1.0, 2.0),
			Velocity::new(5.0, 10.0)
		);
		assert_eq!(
			Acceleration::new(1.0, 2.0) * Time(-5.0),
			Velocity::new(-5.0, -10.0)
		);
		assert_eq!(
			Time(-5.0) * Acceleration::new(1.0, 2.0),
			Velocity::new(-5.0, -10.0)
		);
	}

	#[test]
	fn acceleration_supports_addition() {
		assert_eq!(
			Acceleration::new(0.0, 0.0),
			Acceleration::new(0.0, 0.0) + Acceleration::new(0.0, 0.0),
		);
		assert_eq!(
			Acceleration::new(1.0, 0.0) + Acceleration::new(0.0, 0.0),
			Acceleration::new(1.0, 0.0),
		);
		assert_eq!(
			Acceleration::new(0.0, 1.0) + Acceleration::new(0.0, 0.0),
			Acceleration::new(0.0, 1.0),
		);
		assert_eq!(
			Acceleration::new(1.0, 1.0) + Acceleration::new(1.0, 1.0),
			Acceleration::new(2.0, 2.0),
		);
		assert_eq!(
			Acceleration::new(1.0, 1.0) + Acceleration::new(-1.0, -1.0),
			Acceleration::new(0.0, 0.0),
		);
	}

	#[test]
	fn acceleration_supports_subtraction() {
		assert_eq!(
			Acceleration::new(0.0, 0.0),
			Acceleration::new(0.0, 0.0) - Acceleration::new(0.0, 0.0),
		);
		assert_eq!(
			Acceleration::new(1.0, 0.0) - Acceleration::new(0.0, 0.0),
			Acceleration::new(1.0, 0.0),
		);
		assert_eq!(
			Acceleration::new(0.0, 1.0) - Acceleration::new(0.0, 0.0),
			Acceleration::new(0.0, 1.0),
		);
		assert_eq!(
			Acceleration::new(1.0, 1.0) - Acceleration::new(1.0, 1.0),
			Acceleration::new(0.0, 0.0),
		);
		assert_eq!(
			Acceleration::new(1.0, 1.0) - Acceleration::new(-1.0, -1.0),
			Acceleration::new(2.0, 2.0),
		);
		assert_eq!(
			Acceleration::new(-5.5, 2.5) - Acceleration::new(10.0, 20.0),
			Acceleration::new(-15.5, -17.5),
		);
	}

	/********************* Force ********************/

	#[test]
	fn new_creates_force() {
		let force = Force::new(-1.0, 1.0);
		assert_eq!(force.0.x, -1.0);
		assert_eq!(force.0.y, 1.0);
	}

	#[test]
	fn force_gets_x_and_y() {
		let force = Force::new(-1.0, 1.0);
		assert_eq!(force.x(), force.0.x);
		assert_eq!(force.y(), force.0.y);
	}

	#[test]
	fn force_supports_partialEq() {
		// Test (-1, -1) == (-1, -1), (-1, -1) == (-1, 0),
		//	(-1, -1) == (-1, 1), ..., (1, 1,) == (1, 1).
		// There are nine combinations for each vector. 81 total test cases?
		// For each p1, generate and test each p2...
		for i1 in -1..2 {
			for j1 in -1..2 {
				let x1 = i1 as f64;
				let y1 = j1 as f64;
				let f1 = Force::new(x1, y1);
				for i2 in -1..2 {
					for j2 in -1..2 {
						let x2 = i2 as f64;
						let y2 = j2 as f64;
						let f2 = Force::new(x2, y2);
						if x1 == x2 && y1 == y2 {
							assert_eq!(f1, f2);
						} else {
							assert_ne!(f1, f2);
						}
					}
				}
			}
		}
	}

	#[test]
	fn force_supports_division_by_mass() {
		assert_eq!(
			Force::new(5.0, 10.0) / Mass(5.0),
			Acceleration::new(1.0, 2.0)
		);
		assert_eq!(
			Force::new(5.0, 10.0) / Mass(-5.0),
			Acceleration::new(-1.0, -2.0)
		);
	}

	#[test]
	fn force_supports_addition() {
		assert_eq!(
			Force::new(0.0, 0.0),
			Force::new(0.0, 0.0) + Force::new(0.0, 0.0),
		);
		assert_eq!(
			Force::new(1.0, 0.0) + Force::new(0.0, 0.0),
			Force::new(1.0, 0.0),
		);
		assert_eq!(
			Force::new(0.0, 1.0) + Force::new(0.0, 0.0),
			Force::new(0.0, 1.0),
		);
		assert_eq!(
			Force::new(1.0, 1.0) + Force::new(1.0, 1.0),
			Force::new(2.0, 2.0),
		);
		assert_eq!(
			Force::new(1.0, 1.0) + Force::new(-1.0, -1.0),
			Force::new(0.0, 0.0),
		);
	}

	#[test]
	fn force_supports_subtraction() {
		assert_eq!(
			Force::new(0.0, 0.0),
			Force::new(0.0, 0.0) - Force::new(0.0, 0.0),
		);
		assert_eq!(
			Force::new(1.0, 0.0) - Force::new(0.0, 0.0),
			Force::new(1.0, 0.0),
		);
		assert_eq!(
			Force::new(0.0, 1.0) - Force::new(0.0, 0.0),
			Force::new(0.0, 1.0),
		);
		assert_eq!(
			Force::new(1.0, 1.0) - Force::new(1.0, 1.0),
			Force::new(0.0, 0.0),
		);
		assert_eq!(
			Force::new(1.0, 1.0) - Force::new(-1.0, -1.0),
			Force::new(2.0, 2.0),
		);
		assert_eq!(
			Force::new(-5.5, 2.5) - Force::new(10.0, 20.0),
			Force::new(-15.5, -17.5),
		);
	}

	/********************* Ticks ********************/

	#[test]
	fn ticks_supports_partialEq() {
		assert_eq!(Ticks(0), Ticks(0));
		assert_eq!(Ticks(1), Ticks(1));

		assert_ne!(Ticks(1), Ticks(0));
		assert_ne!(Ticks(0), Ticks(1));
	}
}

pub mod physical_quantities {
use std::ops;

	// Using a tuple struct to wrap an f64 so the compiler treats Time as a
	//	distinct type. This is the "newtype pattern."
	// The PartialEq trait is automatically implemented using "derive" here. The
	//	derived implementation will report equality between two structs if all
	//	fields are equal, and non-equality otherwise.
	/// Represents a length of time. Could be though of as milliseconds, seconds,
	/// minutes, etc.
	#[derive(PartialEq, PartialOrd)]
	#[derive(Debug)]
	#[derive(Clone, Copy)]
	pub struct Time(f64);

	// Implement multiplication of time by a coefficient.
	impl ops::Mul<f64> for Time {
		type Output = Self;

		fn mul(self, rhs: f64) -> Self::Output {
			Self(self.0 * rhs)
		}
	}

	impl ops::Add for Time {
		type Output = Self;

		fn add(self, rhs: Self) -> Self::Output {
			Self(self.0 + rhs.0)
		}
	}

	impl ops::Sub for Time {
		type Output = Self;

		fn sub(self, rhs: Self) -> Self::Output {
			Self(self.0 - rhs.0)
		}
	}

	/// A two-dimensional vector (not to be confused with `Vec<T>`).
	/// Supports basic vector math.
	#[derive(PartialEq)]
	#[derive(Debug)]
	#[derive(Clone, Copy)]
	pub struct Vector2 {
		x: f64,
		y: f64,
	}

	impl Vector2 {
		pub fn new(x: f64, y:f64) -> Self {
			Self {
				x: x,
				y: y,
			}
		}
	}

	// Scalar multiplication of a vector.
	impl ops::Mul<f64> for Vector2 {
		type Output = Self;

		fn mul(self, rhs: f64) -> Self::Output {
			Self {
				x: self.x * rhs,
				y: self.y * rhs,
			}
		}
	}
	impl ops::Mul<Vector2> for f64 {
		type Output = Vector2;

		fn mul(self, rhs: Vector2) -> Self::Output {
			Vector2 {
				x: rhs.x * self,
				y: rhs.y * self,
			}
		}
	}

	// Scalar division of a vector.
	impl ops::Div<f64> for Vector2 {
		type Output = Self;

		fn div(self, rhs: f64) -> Self::Output {
			Self {
				x: self.x / rhs,
				y: self.y / rhs,
			}
		}
	}

	// Vector addition.
	impl ops::Add for Vector2 {
		type Output = Self;

		fn add(self, rhs: Self) -> Self::Output {
			Self {
				x: self.x + rhs.x,
				y: self.y + rhs.y,
			}
		}
	}

	// Vector subtraction.
	impl ops::Sub for Vector2 {
		type Output = Self;

		fn sub(self, rhs: Self) -> Self::Output {
			Self {
				x: self.x - rhs.x,
				y: self.y - rhs.y,
			}
		}
	}

	/// Mass.
	#[derive(PartialEq)]
	#[derive(Debug)]
	#[derive(Clone, Copy)]
	pub struct Mass(f64);

	impl Mass {
		pub fn new(m: f64) -> Self {
			if m <= 0.0 {
				panic!("Mass must be positive.");
			}

			Self(m)
		}
	}

	impl ops::Mul<Acceleration> for Mass {
		type Output = Force;

		fn mul(self, rhs: Acceleration) -> Self::Output {
			Self::Output::new(
				rhs.x() * self.0,
				rhs.y() * self.0,
			)
		}
	}

	/// Position in space (displacement from the origin), displacement relative to
	/// some starting location, or distance from some arbitrary position.
	/// Wraps `Vector2` and provides functionality specific to displacement.
	#[derive(PartialEq)]
	#[derive(Debug)]
	#[derive(Clone, Copy)]
	pub struct Displacement(Vector2);

	impl Displacement {
		pub fn new(x: f64, y: f64) -> Self {
			Self(Vector2::new(x, y))
		}

		pub fn x(&self) -> f64 {
			self.0.x
		}

		pub fn y(&self) -> f64 {
			self.0.y
		}
	}

	impl ops::Add for Displacement {
		type Output = Self;

		fn add(self, rhs: Self) -> Self::Output {
			Self(self.0 + rhs.0)
		}
	}

	impl ops::AddAssign for Displacement {
		fn add_assign(&mut self, other: Self) {
			*self = *self + other;
		}
	}

	impl ops::Sub for Displacement {
		type Output = Self;

		fn sub(self, rhs: Self) -> Self::Output {
			Self(self.0 - rhs.0)
		}
	}

	impl ops::SubAssign for Displacement {
		fn sub_assign(&mut self, other: Self) {
			*self = *self + other;
		}
	}

	/// Velocity.
	/// Wraps `Vector2` and provides functionality specific to velocity.
	#[derive(PartialEq)]
	#[derive(Debug)]
	#[derive(Clone, Copy)]
	pub struct Velocity(Vector2);

	impl Velocity {
		pub fn new(x: f64, y: f64) -> Self {
			Self(Vector2::new(x, y))
		}

		pub fn x(&self) -> f64 {
			self.0.x
		}

		pub fn y(&self) -> f64 {
			self.0.y
		}
	}

	// Multiplication of velocity by time.
	impl ops::Mul<Time> for Velocity {
		type Output = Displacement;

		fn mul(self, rhs: Time) -> Self::Output {
			Displacement(self.0 * rhs.0)
		}
	}
	impl ops::Mul<Velocity> for Time {
		type Output = Displacement;

		fn mul(self, rhs: Velocity) -> Self::Output {
			Displacement(self.0 * rhs.0)
		}
	}

	impl ops::Add for Velocity {
		type Output = Velocity;

		fn add(self, rhs: Self) -> Self::Output {
			Self(self.0 + rhs.0)
		}
	}

	impl ops::AddAssign for Velocity {
		fn add_assign(&mut self, other: Self) {
			*self = *self + other;
		}
	}

	impl ops::Sub for Velocity {
		type Output = Self;

		fn sub(self, rhs: Self) -> Self::Output {
			Self(self.0 - rhs.0)
		}
	}

	impl ops::SubAssign for Velocity {
		fn sub_assign(&mut self, other: Self) {
			*self = *self - other;
		}
	}

	/// Acceleration.
	/// Wraps `Vector2` and provides functionality specific to acceleration.
	#[derive(PartialEq)]
	#[derive(Debug)]
	#[derive(Clone, Copy)]
	pub struct Acceleration(Vector2);

	impl Acceleration {
		pub fn new(x: f64, y: f64) -> Self {
			Self(Vector2::new(x, y))
		}

		pub fn x(&self) -> f64 {
			self.0.x
		}

		pub fn y(&self) -> f64 {
			self.0.y
		}
	}

	// Scalar multiplication of acceleration.
	impl ops::Mul<f64> for Acceleration {
		type Output = Self;

		fn mul(self, rhs: f64) -> Self::Output {
			Acceleration(self.0 * rhs)
		}
	}
	impl ops::Mul<Acceleration> for f64 {
		type Output = Acceleration;

		fn mul(self, rhs: Acceleration) -> Self::Output {
			Acceleration(rhs.0 * self)
		}
	}

	// Multiplication of acceleration by time.
	impl ops::Mul<Time> for Acceleration {
		type Output = Velocity;

		fn mul(self, rhs: Time) -> Self::Output {
			Velocity(self.0 * rhs.0)
		}
	}
	impl ops::Mul<Acceleration> for Time {
		type Output = Velocity;

		fn mul(self, rhs: Acceleration) -> Self::Output {
			Velocity(self.0 * rhs.0)
		}
	}

	impl ops::Add for Acceleration {
		type Output = Acceleration;

		fn add(self, rhs: Self) -> Self::Output {
			Self(self.0 + rhs.0)
		}
	}

	impl ops::Sub for Acceleration {
		type Output = Self;

		fn sub(self, rhs: Self) -> Self::Output {
			Self(self.0 - rhs.0)
		}
	}

	/// Force.
	/// Wraps `Vector2` and provides functionality specific to forces.
	#[derive(PartialEq)]
	#[derive(Debug)]
	#[derive(Clone, Copy)]
	pub struct Force(Vector2);

	impl Force {
		pub fn new(x: f64, y: f64) -> Self {
			Self(Vector2::new(x, y))
		}

		pub fn x(&self) -> f64 {
			self.0.x
		}

		pub fn y(&self) -> f64 {
			self.0.y
		}
	}

	// Force divided by mass.
	impl ops::Div<Mass> for Force {
		type Output = Acceleration;

		fn div(self, rhs: Mass) -> Self::Output {
			Acceleration::new(
				self.x() / rhs.0,
				self.y() / rhs.0,
			)
		}
	}

	impl ops::Add for Force {
		type Output = Force;

		fn add(self, rhs: Self) -> Self::Output {
			Self(self.0 + rhs.0)
		}
	}

	impl ops::Sub for Force {
		type Output = Self;

		fn sub(self, rhs: Self) -> Self::Output {
			Self(self.0 - rhs.0)
		}
	}

	/// A type representing a number of ticks.
	#[derive(PartialEq)]
	#[derive(Debug)]
	#[derive(Clone, Copy)]
	pub struct Ticks(u64);

}
