use std::collections::HashMap;
use std::ops;
use uuid::Uuid;

#[cfg(test)]
mod tests {
    use super::*;

	struct DummyField {
		radius: f64,
		affects_self: bool,
		affects_others: bool,
		name: String,
	}

	impl Field for DummyField {
		fn effect(
			&self,
			simulation: &Simulation,
			position: Displacement,
			particle_ids: Vec<Uuid>
		) {
			// Does nothing.
		}

		fn get_radius(&self) -> f64 {
			self.radius
		}

		fn affects_self(&self) -> bool {
			self.affects_self
		}

		fn affects_others(&self) -> bool {
			self.affects_others
		}

		fn get_name(&self) -> &String {
			&self.name
		}
	}

	/********************* Seconds ********************/

	#[test]
	fn seconds_supports_partialEq() {
		assert!(
			Seconds(-1.0) == Seconds(-1.0),
			"Seconds(-1.0) did not equal Seconds(-1.0)."
		);
		assert!(
			Seconds(0.0) == Seconds(0.0),
			"Seconds(0.0) did not equal Seconds(0.0)."
		);
		assert!(
			Seconds(1.0) == Seconds(1.0),
			"Seconds(1.0) did not equal Seconds(1.0)."
		);

		assert!(
			Seconds(-1.0) != Seconds(0.0),
			"Seconds(-1.0) was equal Seconds(0.0)."
		);
		assert!(
			Seconds(0.0) != Seconds(1.0),
			"Seconds(0.0) was equal to Seconds(1.0)."
		);
		assert!(
			Seconds(1.0) != Seconds(-1.0),
			"Seconds(1.0) was equal to Seconds(-1.0)."
		);
	}

	#[test]
	fn seconds_supports_multiplication_by_a_coefficient() {
		assert_eq!(Seconds(2.0) * 5.0, Seconds(10.0));
		assert_eq!(Seconds(-2.0) * 5.0, Seconds(-10.0));
	}

	#[test]
	fn seconds_supports_subtraction() {
        assert_eq!(Seconds(0.0), Seconds(0.0) - Seconds(0.0));
        assert_eq!(Seconds(-1.0), Seconds(0.0) - Seconds(1.0));
        assert_eq!(Seconds(1.0), Seconds(0.0) - Seconds(-1.0));
        assert_eq!(Seconds(1.0), Seconds(1.0) - Seconds(0.0));
        assert_eq!(Seconds(0.0), Seconds(1.0) - Seconds(1.0));
        assert_eq!(Seconds(2.0), Seconds(1.0) - Seconds(-1.0));
        assert_eq!(Seconds(-1.0), Seconds(-1.0) - Seconds(0.0));
        assert_eq!(Seconds(-2.0), Seconds(-1.0) - Seconds(1.0));
        assert_eq!(Seconds(0.0), Seconds(-1.0) - Seconds(-1.0));
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
	fn velocity_supports_multiplication_by_seconds() {
		assert_eq!(
			Velocity::new(1.0, 2.0) * Seconds(5.0),
			Displacement::new(5.0, 10.0)
		);
		assert_eq!(
			Seconds(5.0) * Velocity::new(1.0, 2.0),
			Displacement::new(5.0, 10.0)
		);
		assert_eq!(
			Velocity::new(1.0, 2.0) * Seconds(-5.0),
			Displacement::new(-5.0, -10.0)
		);
		assert_eq!(
			Seconds(-5.0) * Velocity::new(1.0, 2.0),
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
	fn acceleration_supports_multiplication_by_seconds() {
		assert_eq!(
			Acceleration::new(1.0, 2.0) * Seconds(5.0),
			Velocity::new(5.0, 10.0)
		);
		assert_eq!(
			Seconds(5.0) * Acceleration::new(1.0, 2.0),
			Velocity::new(5.0, 10.0)
		);
		assert_eq!(
			Acceleration::new(1.0, 2.0) * Seconds(-5.0),
			Velocity::new(-5.0, -10.0)
		);
		assert_eq!(
			Seconds(-5.0) * Acceleration::new(1.0, 2.0),
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

	/********************* Particle ********************/

	#[test]
	fn new_creates_particle() {
		let particle = Particle::new(
			Mass::new(1.0),
			Displacement::new(0.0, 0.0),
			Velocity::new(0.0, 0.0),
			vec!(Box::new(
				DummyField {
					radius: 1.0,
					affects_self: false,
					affects_others: false,
					name: String::from("dummy"),
				}
			)),
		);
		assert_eq!(particle.mass, Mass::new(1.0));
		assert_eq!(particle.position, Displacement::new(0.0, 0.0));
		assert_eq!(particle.velocity, Velocity::new(0.0, 0.0));
		assert_eq!(particle.fields.len(), 1);
	}


	/********************* Simulation ********************/

	fn dummy_function(simulation: Simulation) {
		// Will just test that on_tick is Some. Not testing it any further.:
        //
        //  Mutating a static variable is unsafe because multiple
        //  threads may attempt to mutate it at the same time. This issue is
        //  probably relevant to this use case because each test will run on its
        //  own thread and I may want to run the on_tick callback in multiple
        //  tests. I either need to find some other way to verify that the
        //  on_tick callback ran or ensure that I will only mutate test_bool in
        //  one thread (in one test?) at a time (or ever).
        //  
        //  Apparently, to correctly and safely test a function pointer, with no
        //  return type and no parameters, I would need to provide a global
        //  variable and mutate it using Rust's thread synchronization features
        //  (ChatGPT mentions AtomicBool, for example). This appears non-trivial
        //  to understand and would probably require learning to work with
        //  Rust concurrency to do without just mindlessly copying ChatGPT's
        //  recommendation. It's not worth doing just to make sure the function
        //  pointer is set in the Simulation struct in v0.
        //  
		//test_bool = true;
        println!("This is a dummy function for testing Simulation.on_tick.");
	}

	// Verifies that the constructor creates a simulation with the correct
	//	parameters.
    #[test]
    fn new_creates_simulation() {
        let simulation = Simulation::new(
			Seconds(1.0),
			None,
			None,
		);
		assert_eq!(
			simulation.tick_duration,
			Seconds(1.0),
			"Incorrect tick_duration.",
		);
		assert_eq!(
			simulation.elapsed_ticks,
			Ticks(0),
			"Incorrect elapsed_ticks."
		);
		assert!(
			simulation.simulation_speed.is_none(),
			"Incorrect simulation_speed."
		);
		assert!(
			simulation.on_tick.is_none(),
			"Incorrect on_tick."
		);
		assert!(
			simulation.is_paused,
			"The simulation should be paused when instantiated."
		);

		let simulation = Simulation::new(
			Seconds(1.0),
			Some(1.0),
			Some(dummy_function),
		);
		assert_eq!(
			simulation.tick_duration,
			Seconds(1.0),
			"Incorrect tick_duration."
		);
		assert_eq!(
			simulation.elapsed_ticks,
			Ticks(0),
			"Incorrect elapsed_ticks."
		);
		assert_eq!(
			simulation.simulation_speed.expect("Should have simulation speed."),
			1.0,
			"Incorrect simulation_speed."
		);
        assert!(
            simulation.on_tick.is_some(),
            "The on_tick function pointer should be set."
        );
		assert!(
			simulation.is_paused,
			"The simulation should be paused when instantiated."
		);
    }

	#[test]
	#[should_panic(expected = "tick_duration must be positive")]
	fn simulation_new_panics_on_negative_tick_duration() {
		let simulation = Simulation::new(Seconds(-1.0), None, None);
	}

	#[test]
	#[should_panic(expected = "simulation_speed must be positive")]
	fn simulation_new_panics_on_negative_simulation_speed() {
		let simulation = Simulation::new(Seconds(1.0), Some(-1.0), None);
	}

	// Verifies that create_particle() creates a particle with the correct
	//	parameters and that it is added to the particles collection.
	#[test]
	fn simulation_creates_particle() {
		let simulation = Simulation::new(Seconds(1.0), None, None);
		let particle_id_1 = simulation.create_particle(
			Displacement::new(0.0, 0.0),
			Mass::new(1.0),
			Vec::new(),
		);
		let particle_id_2 = simulation.create_particle(
			Displacement::new(0.0, 0.0),
			Mass::new(1.0),
			vec!(Box::new(
				DummyField {
					radius: 1.0,
					affects_self: false,
					affects_others: false,
					name: String::from("dummy"),
				}
			)),
		);

		assert!(
			simulation.particles.contains_key(&particle_id_1),
			"particle_id_1 is not in the particles collection."
		);
		assert!(
			simulation.particles.contains_key(&particle_id_2),
			"particle_id_2 is not in the particles collection."
		);

		let particle_1 = simulation.particles.get(&particle_id_1)
			.expect("simulation.particles should contain particle_id_1");
		let particle_2 = simulation.particles.get(&particle_id_2)
			.expect("simulation.particles should contain particle_id_2");

		assert_eq!(particle_1.position, Displacement::new(0.0, 0.0));
		assert_eq!(particle_1.mass, Mass::new(1.0));
		assert!(
			particle_1.fields.is_empty(),
			"particle_1 should have no fields"
		);

		assert_eq!(particle_2.position, Displacement::new(0.0, 0.0));
		assert_eq!(particle_2.mass, Mass::new(1.0));
		assert_eq!(
			particle_2.fields.len(),
			1,
			"particle_2 should have a field"
		);
	}

	#[test]
	fn simulation_deletes_particle() {
		let simulation = Simulation::new(Seconds(1.0), None, None);
		let particle_id = simulation.create_particle(
			Displacement::new(0.0, 0.0),
			Mass::new(1.0),
			Vec::new(),
		);

		assert!(
			simulation.particles.contains_key(&particle_id),
			"Cannot test particle deletion if the particle was not created.",
		);

		simulation.delete_particle(particle_id);

		assert!(
			!simulation.particles.contains_key(&particle_id),
			"The particles collection should not contain a deleted particle.",
		);
	}

	#[test]
	#[should_panic(expected = "the provided particle ID was not found: ")]
	fn simulation_delete_particle_panics_on_missing_id() {
		let simulation = Simulation::new(Seconds(1.0), None, None);
		simulation.delete_particle(Uuid::new_v4());
	}

	// Verifies that the Simulation.apply_force() method adds a force to the
	//	collection of forces to simulate on the next tick.
	#[test]
	fn simulation_applies_force() {
		let simulation = Simulation::new(Seconds(1.0), None, None);
		let particle_id = simulation.create_particle(
			Displacement::new(0.0, 0.0),
			Mass::new(1.0),
			Vec::new(),
		);

		simulation.apply_force(particle_id, Force::new(1.0, 1.0));
		assert!(
			simulation.applied_forces.contains_key(&particle_id),
			"Applied forces should appear in the applied_forces collection.",
		);

		assert_eq!(
			simulation.applied_forces[&particle_id][0],
			Force::new(1.0, 1.0),
			"The force in applied_forces is incorrect.",
		);
	}

	#[test]
	#[should_panic(expected = "the provided particle ID was not found: ")]
	fn simulation_apply_foce_panics_on_missing_id() {
		let simulation = Simulation::new(Seconds(1.0), None, None);
		simulation.apply_force(Uuid::new_v4(), Force::new(1.0, 1.0));
	}

	#[test]
	fn simulation_gets_mass() {
		let simulation = Simulation::new(Seconds(1.0), None, None);
		let particle_id = simulation.create_particle(
			Displacement::new(0.0, 0.0),
			Mass::new(1.0),
			Vec::new(),
		);

		let mass = simulation.get_mass(particle_id);

		assert_eq!(Mass::new(1.0), mass);
	}

	#[test]
	#[should_panic(expected = "the provided particle ID was not found: ")]
	fn simulation_get_mass_panics_on_missing_id() {
		let simulation = Simulation::new(Seconds(1.0), None, None);
		simulation.get_mass(Uuid::new_v4());
	}
	
	#[test]
	fn simulation_gets_position() {
		let simulation = Simulation::new(Seconds(1.0), None, None);
		let particle_id = simulation.create_particle(
			Displacement::new(-1.23, 123.0),
			Mass::new(1.0),
			Vec::new(),
		);

		let position = simulation.get_position(particle_id);

		assert_eq!(Displacement::new(-1.23, 123.0), position);
	}

	#[test]
	#[should_panic(expected = "the provided particle ID was not found: ")]
	fn simulation_get_position_panics_on_missing_id() {
		let simulation = Simulation::new(Seconds(1.0), None, None);
		simulation.get_position(Uuid::new_v4());
	}

	#[test]
	fn simulation_gets_velocity() {
		let mut simulation = Simulation::new(Seconds(1.0), None, None);
		let particle_id = simulation.create_particle(
			Displacement::new(0.0, 0.0),
			Mass::new(1.0),
			Vec::new(),
		);

		// We have access to the simulation here, so just set the velocity. The
		//	get_velocity method should just return Particle.velocity.
		match simulation.particles.get_mut(&particle_id) {
			Some(p) => p.velocity = Velocity::new(1.0, 1.0),
			None => panic!("The created particle was not found!"),
		};
		let velocity = simulation.get_velocity(particle_id);

		assert_eq!(Velocity::new(1.0, 1.0), velocity);
	}

	#[test]
	#[should_panic(expected = "the provided particle ID was not found: ")]
	fn simulation_get_velocity_panics_on_missing_id() {
		let simulation = Simulation::new(Seconds(1.0), None, None);
		simulation.get_velocity(Uuid::new_v4());
	}

	#[test]
	fn simulation_gets_field_info() {
		let mut simulation = Simulation::new(Seconds(1.0), None, None);
		let particle_id = simulation.create_particle(
			Displacement::new(0.0, 0.0),
			Mass::new(1.0),
			vec!(Box::new(
				DummyField {
					radius: 1.0,
					affects_self: true,
					affects_others: true,
					name: String::from("dummy"),
				}
			)),
		);

		let field_info = simulation.get_field_info(particle_id);

		assert_eq!(field_info[0].radius, 1.0);
		assert!(field_info[0].affects_self);
		assert!(field_info[0].affects_others);
		assert_eq!(field_info[0].name, String::from("dummy"));
	}

	#[test]
	#[should_panic(expected = "the provided particle ID was not found: ")]
	fn simulation_get_field_info_panics_on_missing_id() {
		let simulation = Simulation::new(Seconds(1.0), None, None);
		simulation.get_field_info(Uuid::new_v4());
	}

	#[test]
	fn simulation_starts() {
		let mut simulation = Simulation::new(Seconds(1.0), None, None);

		// Force the simulation to be paused, in case the constructor is broken.
		simulation.is_paused = true;
		
		simulation.start();

		assert!(!simulation.is_paused, "The simulation should have unpaused.");
	}

	#[test]
	fn simulation_pauses() {
		let mut simulation = Simulation::new(Seconds(1.0), None, None);

		// Force the simulation to be unpaused.
		simulation.is_paused = false;
		
		simulation.pause();

		assert!(simulation.is_paused, "The simulation should have paused.");
	}

	// Simulation.step()...

	#[test]
	fn simulation_step_increments_elapsed_ticks() {
		let mut simulation = Simulation::new(Seconds(1.0), None, None);

		// Verify that the count of elapsed ticks increases by one (without
		//	calling get_elapsed_ticks()).
		assert_eq!(Ticks(0), simulation.elapsed_ticks);
		simulation.step();
		assert_eq!(Ticks(1), simulation.elapsed_ticks);
		simulation.step();
		assert_eq!(Ticks(2), simulation.elapsed_ticks);
	}

	// A trivial on_tick function for testing.
	fn create_particle(simulation: Simulation) {
		simulation.create_particle(
			Displacement::new(0.0, 0.0),
			Mass::new(1.0),
			Vec::new(),
		);
	}

	#[test]
	fn simulation_step_calls_on_tick_callback() {
		// Verifies that the on_tick function pointer gets called. This is done
		//	by having on_tick create a particle and verifying that the particle
		//	count changed as expected.
		let simulation = Simulation::new(
			Seconds(1.0),
			None,
			Some(create_particle)
		);

		simulation.step();
		assert_eq!(simulation.particles.len(), 1);
		simulation.step();
		assert_eq!(simulation.particles.len(), 2);
	}

	#[test]
	#[should_panic(expected = "The simulation must be paused to call step().")]
	fn simulation_step_panics_if_not_paused() {
		let mut simulation = Simulation::new(Seconds(1.0), None, None);
		simulation.is_paused = false;
		simulation.step();
	}

	#[test]
	fn simulation_gets_elapsed_ticks() {
		let mut simulation = Simulation::new(Seconds(1.0), None, None);
		simulation.elapsed_ticks = Ticks(1);

		let elapsed_ticks = simulation.get_elapsed_ticks();

		assert_eq!(Ticks(1), elapsed_ticks);
	}

	#[test]
	fn simulation_gets_elapsed_time() {
		let mut simulation = Simulation::new(Seconds(0.001), None, None);
		simulation.elapsed_ticks = Ticks(1000);

		let elapsed_time = simulation.get_elapsed_time();

		// elapsed time = (elapsed ticks) * (ticks duration)
		assert_eq!(Seconds(1.0), elapsed_time);
	}

	struct DeletionField {
		radius: f64,
		affects_self: bool,
		affects_others: bool,
		name: String,
	}

	impl Field for DeletionField {
		fn effect(
			&self,
			simulation: &Simulation,
			position: Displacement,
			particle_ids: Vec<Uuid>
		) {
			for p in particle_ids {
				simulation.delete_particle(p);
			}
		}

		fn get_radius(&self) -> f64 {
			self.radius
		}

		fn affects_self(&self) -> bool {
			self.affects_self
		}

		fn affects_others(&self) -> bool {
			self.affects_others
		}

		fn get_name(&self) -> &String {
			&self.name
		}
	}

	// Verifies that a field attached to a particle will be called and passed a
	//	list of all particles within its radius when a tick (step()) occurs.
	#[test]
	fn simulation_field_affects_others() {
		let simulation = Simulation::new(Seconds(1.0), None, None);
		let field = DeletionField {
			radius: 10.0,
			affects_self: false,
			affects_others: true,
			name: String::from("The Destructor"),
		};
		let destroyer = simulation.create_particle(
			Displacement::new(0.0, 0.0),
			Mass::new(1.0),
			vec!(Box::new(field)),
		);
		let victim_1 = simulation.create_particle(
			Displacement::new(1.0, 0.0),
			Mass::new(1.0),
			Vec::new(),
		);
		let victim_2 = simulation.create_particle(
			Displacement::new(0.0, 1.0),
			Mass::new(1.0),
			Vec::new(),
		);
		let survivor = simulation.create_particle(
			Displacement::new(10.1, 0.0),
			Mass::new(1.0),
			Vec::new(),
		);

		simulation.step();

		assert!(
			!simulation.particles.contains_key(&victim_1),
			"The victim_1 particle should have been deleted.",
		);
		assert!(
			!simulation.particles.contains_key(&victim_2),
			"The victim_2 particle should have been deleted.",
		);
		assert!(
			simulation.particles.contains_key(&destroyer),
			"The destroyer particle should still exist.",
		);
		assert!(
			simulation.particles.contains_key(&survivor),
			"The survivor particle should still exist.",
		);
	}

	// Verifies that a field meant to affect itself will be passed its own
	//	particle's ID.
	#[test]
	fn simulation_field_affects_self() {
		let simulation = Simulation::new(Seconds(1.0), None, None);
		let field = DeletionField {
			radius: 10.0,
			affects_self: true,
			affects_others: false,
			name: String::from("Self Destructor"),
		};
		let suicide_particle = simulation.create_particle(
			Displacement::new(0.0, 0.0),
			Mass::new(1.0),
			vec!(Box::new(field)),
		);
		let survivor = simulation.create_particle(
			Displacement::new(1.0, 0.0),
			Mass::new(1.0),
			Vec::new(),
		);

		assert!(simulation.particles.contains_key(&suicide_particle));

		simulation.step();

		assert!(
			!simulation.particles.contains_key(&suicide_particle),
			"The suicide_particle should have been deleted.",
		);
		assert!(
			!simulation.particles.contains_key(&suicide_particle),
			"The survivor particle should still exist.",
		);
	}


	/************** Simulation: functional tests ********************/

	// Verifies that an applied force causes the expected increase in velocity
	//	and that a velocity actually causes the expected displacement.
	#[test]
	fn functional_single_force() {
		let force = Force::new(1.0, 0.0);
		let mass = Mass::new(1.0);
		let tick_duration = Seconds(1.0);
		let expected_velocity;
		let mut expected_displacement;
		let simulation = Simulation::new(tick_duration, None, None);
		let particle_id = simulation.create_particle(
			Displacement::new(0.0, 0.0),
			mass,
			vec!(),
		);
		let particle = simulation.particles
			.get(&particle_id)
			.expect("The particle that was just created should exist.");

		// Apply a force.
		simulation.apply_force(particle_id, force);
		// During this step, the particle should accelerate as the force is
		//	simulated.
		simulation.step();
		// Verify that the particle moved the distance expected during its
		//	acceleration, based on the particle's mass, force vector, and force
		//	duration. The actual displacement should be exactly as calculated by
		//	the equations of motion here because we're only applying a force for
		//	a single tick.
		// a = f / m
		// d = (1 / 2) * a * t^2 (when initial position and velocity are 0)
		// Therefore, d = (1 / 2) * (f / m) * t^2
		expected_displacement =
			0.5 * (force / mass) * tick_duration * tick_duration;
		assert_eq!(expected_displacement, particle.position);
		// During this step, the particle should coast at a known velocity.
		simulation.step();
		// Verify that the particle moved the distance expected, given its
		//	expected velocity.
		// a = f / m
		// v = a * t (when initial velocity is 0)
		// Therefore, v = (f / m) * t
		expected_velocity = (force / mass) * tick_duration;
		expected_displacement += expected_velocity * tick_duration;
		assert_eq!(expected_displacement, particle.position);
	}

	// Apply several forces, then call step() and verify that the particle
	//	reacts appropriately to the vector sum of the forces.
	#[test]
	fn functional_sum_of_several_forces() {
		let f0 = Force::new(1.0, 1.0);
		let f1 = Force::new(10.0, 10.0);
		let f2 = Force::new(-2.0, -1.0);
		let f3 = Force::new(-10.0, -4.0);
		let f4 = Force::new(20.0, -1.0);
		let net_force = f0 + f1 + f2 + f3 + f4;
		let mass = Mass::new(1.0);
		let tick_duration = Seconds(1.0);
		let expected_velocity;
		let mut expected_displacement;
		let simulation = Simulation::new(tick_duration, None, None);
		let particle_id = simulation.create_particle(
			Displacement::new(0.0, 0.0),
			mass,
			vec!(),
		);
		let particle = simulation.particles
			.get(&particle_id)
			.expect("The particle that was just created should exist.");

		// Apply several forces.
		simulation.apply_force(particle_id, f0);
		simulation.apply_force(particle_id, f1);
		simulation.apply_force(particle_id, f2);
		simulation.apply_force(particle_id, f3);
		simulation.apply_force(particle_id, f4);
		// During this step, the particle should accelerate as the force is
		//	simulated.
		simulation.step();
		// Verify that the particle moved the distance expected during its
		//	acceleration, based on the particle's mass, force vector, and force
		//	duration. The actual displacement should be exactly as calculated by
		//	the equations of motion here because we're only applying a force for
		//	a single tick.
		// a = f / m
		// d = (1 / 2) * a * t^2 (when initial position and velocity are 0)
		// Therefore, d = (1 / 2) * (f / m) * t^2
		expected_displacement =
			0.5 * (net_force / mass) * tick_duration * tick_duration;
		assert_eq!(expected_displacement, particle.position);
		// During this step, the particle should coast at a known velocity.
		simulation.step();
		// Verify that the particle moved the distance expected, given its
		//	expected velocity.
		// a = f / m
		// v = a * t (when initial velocity is 0)
		// Therefore, v = (f / m) * t
		expected_velocity = (net_force / mass) * tick_duration;
		expected_displacement += expected_velocity * tick_duration;
		assert_eq!(expected_displacement, particle.position);
	}

	// Apply a force to particles with several different masses. Verify that
	//	the particles move appropriately.
	#[test]
	fn functional_force_applied_to_several_masses() {
		let force = Force::new(25.123, 50.5);
		let m0 = Mass::new(10.1);
		let m1 = Mass::new(100.01);
		let m2 = Mass::new(20.0);
		let m3 = Mass::new(200.0);
		let tick_duration = Seconds(0.005);
		let mut expected_velocity;
		let mut d0;
		let mut d1;
		let mut d2;
		let mut d3;
		let simulation = Simulation::new(tick_duration, None, None);
		let p_id_0 = simulation.create_particle(
			Displacement::new(0.0, 0.0),
			m0,
			vec!(),
		);
		let p_id_1 = simulation.create_particle(
			Displacement::new(0.0, 0.0),
			m1,
			vec!(),
		);
		let p_id_2 = simulation.create_particle(
			Displacement::new(0.0, 0.0),
			m2,
			vec!(),
		);
		let p_id_3 = simulation.create_particle(
			Displacement::new(0.0, 0.0),
			m3,
			vec!(),
		);
		let p0 = simulation.particles
			.get(&p_id_0)
			.expect("The particle (p_id_0) that was just created should exist.");
		let p1 = simulation.particles
			.get(&p_id_1)
			.expect("The particle (p_id_1) that was just created should exist.");
		let p2 = simulation.particles
			.get(&p_id_2)
			.expect("The particle (p_id_2) that was just created should exist.");
		let p3 = simulation.particles
			.get(&p_id_3)
			.expect("The particle (p_id_3) that was just created should exist.");

		// Apply a force.
		simulation.apply_force(p_id_0, force);
		simulation.apply_force(p_id_1, force);
		simulation.apply_force(p_id_2, force);
		simulation.apply_force(p_id_3, force);

		// During this step, the particles should accelerate as the force is
		//	simulated.
		simulation.step();

		// Verify that the particles moved the distance expected during their
		//	acceleration, based on the particles' masses, force vector, and
		//	force duration. The actual displacement should be exactly as
		//	calculated by the equations of motion here because we're only
		//	applying a force for a single tick.
		// a = f / m
		// d = (1 / 2) * a * t^2 (when initial position and velocity are 0)
		// Therefore, d = (1 / 2) * (f / m) * t^2
		d0 = 0.5 * (force / m0) * tick_duration * tick_duration;
		assert_eq!(
			d0,
			p0.position,
			"The displacement of particle p0 was wrong after acceleration."
		);
		d1 = 0.5 * (force / m1) * tick_duration * tick_duration;
		assert_eq!(
			d1,
			p1.position,
			"The displacement of particle p1 was wrong after acceleration."
		);
		d2 = 0.5 * (force / m2) * tick_duration * tick_duration;
		assert_eq!(
			d2,
			p2.position,
			"The displacement of particle p2 was wrong after acceleration."
		);
		d3 = 0.5 * (force / m3) * tick_duration * tick_duration;
		assert_eq!(
			d3,
			p3.position,
			"The displacement of particle p3 was wrong after acceleration."
		);

		// During this step, the particles should coast at known velocities.
		simulation.step();

		// Verify that the particles moved the distance expected, given their
		//	expected velocities.
		// a = f / m
		// v = a * t (when initial velocity is 0)
		// Therefore, v = (f / m) * t
		expected_velocity = (force / m0) * tick_duration;
		d0 += expected_velocity * tick_duration;
		assert_eq!(
			d0,
			p0.position,
			"The displacement of particle p0 was wrong after coasting."
		);
		expected_velocity = (force / m1) * tick_duration;
		d1 += expected_velocity * tick_duration;
		assert_eq!(
			d1,
			p1.position,
			"The displacement of particle p1 was wrong after coasting."
		);
		expected_velocity = (force / m2) * tick_duration;
		d2 += expected_velocity * tick_duration;
		assert_eq!(
			d2,
			p2.position,
			"The displacement of particle p2 was wrong after coasting."
		);
		expected_velocity = (force / m3) * tick_duration;
		d3 += expected_velocity * tick_duration;
		assert_eq!(
			d3,
			p3.position,
			"The displacement of particle p3 was wrong after coasting."
		);
	}

	// Apply a force to particles in simulations with several different
	//	tick_durations. Verify that the particles move appropriately.
	#[test]
	fn functional_force_applied_with_several_tick_durations() {
		let force = Force::new(5.0, 10.0);
		let mass = Mass::new(1.0);
		let tick_0 = Seconds(0.001);
		let tick_1 = Seconds(0.0002);
		let tick_2 = Seconds(0.00003);
		let tick_3 = Seconds(0.000004);
		let mut expected_velocity;
		let mut d0;
		let mut d1;
		let mut d2;
		let mut d3;
		let s0 = Simulation::new(tick_0, None, None);
		let s1 = Simulation::new(tick_1, None, None);
		let s2 = Simulation::new(tick_2, None, None);
		let s3 = Simulation::new(tick_3, None, None);
		let p_id_0 = s0.create_particle(
			Displacement::new(0.0, 0.0),
			mass,
			vec!(),
		);
		let p_id_1 = s1.create_particle(
			Displacement::new(0.0, 0.0),
			mass,
			vec!(),
		);
		let p_id_2 = s2.create_particle(
			Displacement::new(0.0, 0.0),
			mass,
			vec!(),
		);
		let p_id_3 = s3.create_particle(
			Displacement::new(0.0, 0.0),
			mass,
			vec!(),
		);
		let p0 = s0.particles
			.get(&p_id_0)
			.expect("The particle that was just created in s0 should exist.");
		let p1 = s1.particles
			.get(&p_id_1)
			.expect("The particle that was just created in s1 should exist.");
		let p2 = s2.particles
			.get(&p_id_2)
			.expect("The particle that was just created in s2 should exist.");
		let p3 = s3.particles
			.get(&p_id_3)
			.expect("The particle that was just created in s3 should exist.");

		// Apply a force.
		s0.apply_force(p_id_0, force);
		s1.apply_force(p_id_1, force);
		s2.apply_force(p_id_2, force);
		s3.apply_force(p_id_3, force);
		// During this step, the particle should accelerate as the force is
		//	simulated.
		s0.step();
		s1.step();
		s2.step();
		s3.step();
		// Verify that the particle moved the distance expected during its
		//	acceleration, based on the particle's mass, force vector, and force
		//	duration. The actual displacement should be exactly as calculated by
		//	the equations of motion here because we're only applying a force for
		//	a single tick.
		// a = f / m
		// d = (1 / 2) * a * t^2 (when initial position and velocity are 0)
		// Therefore, d = (1 / 2) * (f / m) * t^2
		d0 = 0.5 * (force / mass) * tick_0 * tick_0;
		assert_eq!(
			d0,
			p0.position,
			"The displacement of particle p0 was wrong after acceleration."
		);
		d1 = 0.5 * (force / mass) * tick_1 * tick_1;
		assert_eq!(
			d1,
			p1.position,
			"The displacement of particle p1 was wrong after acceleration."
		);
		d2 = 0.5 * (force / mass) * tick_2 * tick_2;
		assert_eq!(
			d2,
			p2.position,
			"The displacement of particle p2 was wrong after acceleration."
		);
		d3 = 0.5 * (force / mass) * tick_3 * tick_3;
		assert_eq!(
			d3,
			p3.position,
			"The displacement of particle p3 was wrong after acceleration."
		);
		// During this step, the particle should coast at a known velocity.
		s0.step();
		s1.step();
		s2.step();
		s3.step();
		// Verify that the particle moved the distance expected, given its
		//	expected velocity.
		// a = f / m
		// v = a * t (when initial velocity is 0)
		// Therefore, v = (f / m) * t
		expected_velocity = (force / mass) * tick_0;
		d0 += expected_velocity * tick_0;
		assert_eq!(
			d0,
			p0.position,
			"The displacement of particle p0 was wrong after coasting."
		);
		expected_velocity = (force / mass) * tick_1;
		d1 += expected_velocity * tick_1;
		assert_eq!(
			d1,
			p1.position,
			"The displacement of particle p1 was wrong after coasting."
		);
		expected_velocity = (force / mass) * tick_2;
		d2 += expected_velocity * tick_2;
		assert_eq!(
			d2,
			p2.position,
			"The displacement of particle p2 was wrong after coasting."
		);
		expected_velocity = (force / mass) * tick_3;
		d3 += expected_velocity * tick_3;
		assert_eq!(
			d3,
			p3.position,
			"The displacement of particle p3 was wrong after coasting."
		);
	}

	fn displacements_are_almost_equal(
		d1: Displacement,
		d2: Displacement,
		error: f64
	) -> bool {
		let diff = d1 - d2;
		diff.x().abs() <= error && diff.y().abs() <= error
	}

	fn velocities_are_almost_equal(
		v1: Velocity,
		v2: Velocity,
		error: f64
	) -> bool {
		let diff = v1 - v2;
		diff.x().abs() <= error && diff.y().abs() <= error
	}


	// Apply several forces in several directions, over a few seconds. Check the
	//	displacement after each second.
	// Resulting values may not be precisely the same as calculated values
	//	due to the tick-based nature of the simulation and floating point error.
	//	Need to decide what level of error is acceptable for a given tick length
	//	and number of ticks.
	#[test]
	fn functional_several_forces_over_several_seconds() {
		let permissible_error = 0.0;
		let tick_duration = Seconds(0.001);
		let initial_position = Displacement::new(0.0, 0.0);
		let simulation = Simulation::new(tick_duration, None, None);
		let mass = Mass::new(5.0);
		let particle_id = simulation.create_particle(
			initial_position,
			mass,
			Vec::new(),
		);

		// Nine combinations of positive, negative, and 0:
		//	(-, -), (-, 0), (-, +), (0, -), (0, 0), (0, +), (+, -), (+, 0),
		//	(+, +)
		let mut force;
		let mut expected_position = initial_position;
		let mut actual_position = initial_position;
		let mut expected_velocity = Velocity::new(0.0, 0.0);
		let mut elapsed_time = Seconds(0.0);
		let mut time_since_last_round = Seconds(0.0);
        let mut expected_acceleration;
		for i in -1..2 {
			for j in -1..2 {
				force = Force::new(10.0 * (i as f64), 5.0 * (j as f64));

				// Run a second worth of ticks, applying the current force
				//	the whole time.
				for i in 0..((1.0 / tick_duration.0) as i64) {
					simulation.apply_force(particle_id, force);
					simulation.step();
				}

				// Depending on tick duration, we may not be able to run for
				//	exactly one second. Ask the simulation how much time it has
				//	actually simulated.
				time_since_last_round =
					simulation.get_elapsed_time() - elapsed_time;
				elapsed_time = simulation.get_elapsed_time();

				// Calculate the expected position after the most recent second.
				// a = f / m
				// r = r_0 + v_0 * t + 0.5 * a * t * t
				// therefore, r = r_0 + v_0 * t + 0.5 * (f / m) * t * t
                expected_acceleration = force / mass;
				expected_position =
					actual_position + expected_velocity * time_since_last_round +
					0.5 * expected_acceleration * time_since_last_round *
					time_since_last_round;

				// Get the new position from the simulation.
				actual_position = simulation.get_position(particle_id);

                // Calculate the new expected velocity.
                expected_velocity +=
                    expected_acceleration * time_since_last_round;

				// Assert that the new position is correct. Use a failure
				//	message that includes the difference between expected and
				//	actual position.
                assert!(
                    displacements_are_almost_equal(
                        expected_position,
                        actual_position,
                        permissible_error,
                    ),
                    "Position error greater than permissible error of {:?}.\n\
                    expected_position = {:?}\n\
                    actual_position = {:?}\n\
                    actual - expected = {:?}",
                    permissible_error,
                    expected_position,
                    actual_position,
                    actual_position - expected_position,
                );
			}
		}
	}

    // TODO: Continue implementing tests from here.

	// Creates a particle with a self-affecting gravity field and launches it
	//	with a known force. Uses equations of motion to calculate the expected
	//	position, velocity, energy, etc. of the particle at different times.
	// Resulting values may not be precisely the same as calculated values
	//	due to the tick-based nature of the simulation and floating point error.
	//	Need to decide what level of error is acceptable for a given tick length
	//	and number of ticks.
	#[test]
	fn functional_trajectory() {
        let permissible_error = 0.0;
        let tick_duration = Seconds(0.001);
        let simulation = Simulation::new(tick_duration, None, None);
        let force = Force::new(250.0, 500.0);
        let force_duration = Seconds(3.0);
        let mass = Mass::new(5.0);
        let mut expected_position = Displacement::new(0.0, 0.0);
        let mut actual_position = Displacement::new(0.0, 0.0);
        let gravity_field = SimpleSelfGravityField::new(
            Acceleration::new(0.0, -9.81),
            None,
        );
        let particle_id = simulation.create_particle(
            expected_position,
            mass,
            vec!(Box::new(gravity_field)),
        );

        // Apply the force for a few seconds.
        for i in 0..((force_duration.0 / tick_duration.0) as i64) {
            simulation.apply_force(particle_id, force);
            simulation.step();
        }

        // Find out how long the force was actually applied for and where the
        //  particle was at the end of the acceleration period.
        let actual_force_duration = simulation.get_elapsed_time();

        // Calculate the expected position after the acceleration period.
        // a = f / m
        // r = r_0 + v_0 * t + 0.5 * a * t * t
        // r = r_0 + v_0 * t + 0.5 * (f / m) * t * t
        // r_0 and v_0 = 0
        // Therefore, r = 0.5 * (f / m) * t * t
        expected_position =
            0.5 * (force / mass) * actual_force_duration
            * actual_force_duration;

        // Get the new position from the simulation.
        actual_position = simulation.get_position(particle_id);
        
        // Assert that the particle is in the correct position immediately after
        //  the last tick of acceleration.
        assert!(
            displacements_are_almost_equal(
                expected_position,
                actual_position,
                permissible_error,
            ),
            "Position error greater than permissible error of {:?}.\n\
            expected_position = {:?}\n\
            actual_position = {:?}\n\
            actual - expected = {:?}",
            permissible_error,
            expected_position,
            actual_position,
            actual_position - expected_position,
        );


        // Calculate when the particle should reach its maximum height.
        // t_1 = (-v_yf / g) + t_f
        // Where...
        // t_1 is time at which max height is reached.
        // v_yf is y-velocity immediately after the force is done acting.
        // g, acceleration due to gravity, is negative.
        // t_f is the time for which the force was acting.
        let v_yf = ((force / mass) * actual_force_duration).y();
        let t_1 = (-v_yf / 
        // Replace the math-friendly name with a programmer-friendly name.
        let expected_time_to_y_max = t_1;

        // Calculate the expected maximum height.
        // y_f = 0.5 * ((f_0y / m) + g) * t_f^2
        // y_max = y_f + v_yf * (t_1 - t_f) + 0.5 * g * (t_1 - t_f)^2
        // Where...
        // y_f is the height achieved while the force was acting.
        // f_0y is the y-component of the force.
        // m is the mass of the particle.
        // y_max is the maximum height.

        // Calculate how long the particle should take to fall from its maximum
        //  height.
        // t_2 = sqrt((-2 * y_max) / g)
        // Where t_2 is the time taken for the particle to fall from its maximum
        //  height.

        // Calculate the particle's total flight time (from when the force is
        //  first applied to when the particle falls to y = 0).
        // t_omega = t_1 + t_2
        // Where t_omega is total flight time.
        
        // Calculate total distance the particle should travel (on the x-axis).
        // d = 0.5 * (f_0x / m) * t_f^2 + ((f_0x / m) * t_f) * (t_omega - t_f)
        // Where...
        // d is the total distance.
        // f_0x is the x-component of the force.
        

        // Step until the particle returns to y = 0, or until enough time has
        //  passed that we know it should have returned to y = 0.
        while actual_position.y() > 0.0 {
            panic!("Infinit lop y u no end :(");
            // As the particle coasts, repeatedly assert that its position is
            //  correct.

            // TODO: Consider adding some kind of failure condition if the particle
            //  never crosses y = 0. I need to calculate the time at which it's
            //  expected to cross 0 anyway. I might as well use it to avoid
            //  looping forever if the particle doesn't move or flies off into space.

            // Save the time and position of the highest point in the trajectory.
        }

        // Assert that the time and position of the peak were correct.

        // Assert that the time and position of the zero-crossing (i.e., current
        //  time and position) are correct.
	}


	// Creates two particles with rigid body fields. Launches one of those
	//	particles at the other and verifies that the resulting velocities (speed
	//	and *direction*) and kinetic energy of each particle are as expected of
	//	an elastic collision.
	// Resulting values may not be precisely the same as calculated values
	//	due to the tick-based nature of the simulation and floating point error.
	//	Need to decide what level of error is acceptable for a given tick length
	//	and number of ticks.
	// TODO: Define a rigid body field as part of the library.
	#[test]
	fn functional_collision() {
	}


	// TODO: When done writing tests, re-read the Rust book's chapter on project
	//	organization and break this project into multiple files. Probably give
	//	each struct its own file.
}

// Using a tuple struct to wrap an f64 so the compiler treats Seconds as a
//	distinct type. This is the "newtype pattern."
// The PartialEq trait is automatically implemented using "derive" here. The
//	derived implementation will report equality between two structs if all
//	fields are equal, and non-equality otherwise.
/// Time, in seconds.
#[derive(PartialEq)]
#[derive(Debug)]
#[derive(Clone, Copy)]
pub struct Seconds(f64);

// Implement multiplication of time by a coefficient.
impl ops::Mul<f64> for Seconds {
	type Output = Self;

	fn mul(self, rhs: f64) -> Self::Output {
		Self(self.0 * rhs)
	}
}

impl ops::Sub for Seconds {
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
impl ops::Mul<Seconds> for Velocity {
	type Output = Displacement;

	fn mul(self, rhs: Seconds) -> Self::Output {
		Displacement(self.0 * rhs.0)
	}
}
impl ops::Mul<Velocity> for Seconds {
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
impl ops::Mul<Seconds> for Acceleration {
	type Output = Velocity;

	fn mul(self, rhs: Seconds) -> Self::Output {
		Velocity(self.0 * rhs.0)
	}
}
impl ops::Mul<Acceleration> for Seconds {
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

// TODO: Implement a "rigid body" (or "basic collider") collider Field as part of the library. It
//	could expose parameters (e.g. coefficient of friction, coefficient of
//	restitution) as fields of the struct.
/// Defines a field. A field is a struct implementing a method that is called by
/// the physics engine on each tick in which a particle is within a radius
/// specified by the field, centered on a particle to which the field is
/// attached.
pub trait Field {
	/// Determines what happens when the field is triggered.
	/// # Arguments
	/// * `simulation` - The Simulation that called the effect function.
	/// # `position` - The position of the particle to which this field is
	///		attached. The center of the field.
	/// * `particle_ids` - IDs of all particles affected by the field. Determined
	///		by the simulation.
	fn effect(
		&self,
		simulation: &Simulation,
		position: Displacement,
		particle_ids: Vec<Uuid>
	);

	// This is a method instead of a field because there is no way to specify
	//	that a trait implementation must have a field.
	/// Called by the simulation to get the field's radius.
	fn get_radius(&self) -> f64;

	/// Called by the simulation to determine whether this field affects the
	///	particle to which it's attached.
	fn affects_self(&self) -> bool;

	/// Called by the simulation to determine whether this field affects
	/// particles other than the particle to which it's attached.
	fn affects_others(&self) -> bool;

	/// Called by the simulation to get a name that identifies this field, which
	/// it will then make available to user-defined code through
	/// `simulation.get_field_info()`. One application of this may be to have
	/// fields affect particles differently if those particles have a field
	/// with the same name. For example, a water particle may have a "water"
	/// field that uses certain rules to apply  a cohesion force to other
	/// particles with a water field and uses different rules to apply an
	/// adhesion force to particles without the water field.
	fn get_name(&self) -> &String;
}

/// Applies "gravity" to the particle to which the field is attached.
/// Implemented as a force that pulls the object to which it's attached in the
/// direction of the specified acceleration.
pub struct SimpleSelfGravityField {
    acceleration: Acceleration,
    name: String,
}

impl SimpleSelfGravityField {
	/// Creates an instance of `SimpleSelfGravityField `.
	///
	/// # Arguments
	/// * `acceleration` - The acceleration due to gravity, "little 'g'."
    /// * `name` - The field name. Defaults to "SimpleSelfGravityField" if None.
    fn new(acceleration: Acceleration, name: Option<String>)
        -> SimpleSelfGravityField
    {
        let field_name = match name {
            Some(s) => s,
            None => String::from("SimpleSelfGravityField"),
        };

        SimpleSelfGravityField {
            acceleration: acceleration,
            name: field_name,
        }
    }
}

impl Field for SimpleSelfGravityField {
    fn effect(
        &self,
        simulation: &Simulation,
        position: Displacement,
        particle_ids: Vec<Uuid>
    ) {
        // There should only ever be one thing in the Vec (the particle to
        //  which this field is attached).
        for id in particle_ids {
            let force = simulation.get_mass(id) * self.acceleration;
            simulation.apply_force(id, force);
        }
    }

	fn get_radius(&self) -> f64 {
        0.0
    }

	fn affects_self(&self) -> bool {
        true
    }

	fn affects_others(&self) -> bool {
        false
    }

	fn get_name(&self) -> &String {
        &self.name
    }
}

#[derive(Debug)]
pub struct FieldInfo {
	radius: f64,
	affects_self: bool,
	affects_others: bool,
	name: String,
}

// TODO: Should this (and probably other structs) actually be public? The
//	Simulation's interface is written in a way that assumes none of this
//	struct's fields will be directly accessible by the user.
/// Represents an infinitesimal massive particle. Stores the particle's mass,
/// position, velocity, and attached `Field`s.
pub struct Particle {
	mass: Mass,
	position: Displacement,
	velocity: Velocity,
	// Vec<Box<dyn Field>> is a "trait object". This is apparently necessary to
	//	make a Vec store an unknown type that implements a trait.
	fields: Vec<Box<dyn Field>>,
	id: Uuid,
}

impl Particle {
    pub fn new(
        mass: Mass,
        position: Displacement,
        velocity: Velocity,
        fields: Vec<Box<dyn Field>>,
    ) -> Self {
		// TODO: Intentionally incorrect placeholder code. Write tests, then
		//	replace.
        Self {
            mass: Mass::new(2384928.0),
            position: Displacement(Vector2::new(45345.0, 43434.0)),
            velocity: Velocity(Vector2::new(45345.0, 43434.0)),
            fields: Vec::new(),
			id: Uuid::new_v4(),
        }
    }
}

/// Owns all of the data that constitutes a physics simulation. Provides methods
/// for interacting with and running the simulation.
pub struct Simulation {
	// The number of simulated seconds that elapse in a single tick.
	//	This is effectively the resolution of the simulation.
	tick_duration: Seconds,
	// A collection that owns all particles in the simulation.
	particles: HashMap<Uuid, Particle>,
	// The number of ticks that have passed so far.
	elapsed_ticks: Ticks,
	// Speed at which the simulation will run, resources permitting. Units are
	//	(simulated seconds) / (real world second). If None, run as fast as
	//	possible.
	simulation_speed: Option<f64>,
	// A function called on each tick. Allows user-defined logic to be driven
	//	by the simulation.
	on_tick: Option<fn()>,
	// Holds forces, keyed by particle_id, to calculate on the next tick.
	applied_forces: HashMap<Uuid, Vec<Force>>,
	is_paused: bool,
}

impl Simulation {
	fn tick(&self) {
		if !self.is_paused {
			// Do stuff.
		}
	}

	/// Creates an instance of `Simulation`.
	///
	/// # Arguments
	/// * `tick_duration` - The number of simulated seconds that elapse in a
	///		single tick. Effectively the resolution of the simulation.
	/// * `simulation_speed` - The speed at which the simulation will run,
	///		resources permitting.
	///		Units are (simulated seconds) / (real world second).
	///		If None is specified, the simulation will run as fast as possible.
	/// * `on_tick` - A function that will be called by the simulation on each
	///		tick. Takes the simulation itself as a parameter to allow the user
	///		to write code that influences the simulation.
	///
	///	# Panics
	/// Panics if `tick_duration` or `simulation_speed` is less than or equal to
	/// zero.
	pub fn new(
		tick_duration: Seconds,
		simulation_speed: Option<f64>,
		on_tick: Option<fn(Simulation)>,
	) -> Self {
	/* TODO: Uncomment this and delete the incorrect code below this.
		// TODO: Remember to panic as described in the documentation comment.
		Self {
			tick_duration: tick_duration,
			particles: HashMap::new(),
			elapsed_ticks: Ticks(0),
			simulation_speed: simulation_speed,
			on_tick: on_tick,
			applied_forces: HashMap::new(),
			is_paused: true,
		}
	*/
		Self {
			tick_duration: Seconds(-1.0),
			particles: HashMap::new(),
			elapsed_ticks: Ticks(11234124),
			simulation_speed: Some(-1.0),
			on_tick: None,
			applied_forces: HashMap::new(),
			is_paused: false,
		}
	}


	/// Creates a new particle and adds it to the simulation. Returns that
	/// particle's unique ID.
	///
	/// # Arguments
	/// * `position` - The particle's coordinates in space.
	/// * `mass` - The particle's mass.
	/// * `fields` - Fields to attach to the particle.
	pub fn create_particle(
		&self,
		position: Displacement,
		mass: Mass,
		fields: Vec<Box<dyn Field>>,
	) -> Uuid {
		Uuid::new_v4()
	}

	/// Removes a particle from the simulation.
	///
	/// # Arguments
	/// * `particle_id` - The unique ID of the particle to delete.
	///
	/// # Panics
	/// This method will panic if there is no particle identified by
	/// 	`particle_id`.
	pub fn delete_particle(&self, particle_id: Uuid) {
	}

	/// Applies a force to a specific particle for the duration of the next
	/// tick.
	///
	/// # Arguments
	/// * `particle_id` - The unique ID of the particle to which to apply a
	/// 	force.
	/// * `force` - The force vector to apply to the particle.
	///
	/// # Panics
	/// This method will panic if there is no particle identified by
	/// 	`particle_id`.
	pub fn apply_force(
		&self,
		particle_id: Uuid,
		force: Force,
	) {
	}

	/// Gets the mass of a specific particle.
	///
	/// # Arguments
	/// * `particle_id` - The unique ID of the particle for which to retrieve
	///		mass.
	///
	/// # Panics
	/// This method will panic if there is no particle identified by
	/// 	`particle_id`.
	pub fn get_mass(&self, particle_id: Uuid) -> Mass {
		Mass::new(234234.0)
	}

	/// Gets the position (i.e., displacement from the origin) of a specific
	/// particle.
	///
	/// # Arguments
	/// * `particle_id` - The unique ID of the particle for which to retrieve
	///		position.
	///
	/// # Panics
	/// This method will panic if there is no particle identified by
	/// 	`particle_id`.
	pub fn get_position(&self, particle_id: Uuid) -> Displacement {
		Displacement::new(234.0, 2342.0)
	}

	/// Gets the velocity of a specific particle.
	///
	/// # Arguments
	/// * `particle_id` - The unique ID of the particle for which to retrieve
	///		mass.
	///
	/// # Panics
	/// This method will panic if there is no particle identified by
	/// 	`particle_id`.
	pub fn get_velocity(&self, particle_id: Uuid) -> Velocity {
		Velocity::new(23423.4, 234234.4)
	}

	/// Gets a collection containing information about all `Field`s attached to
	///	a specific particle.
	///
	/// # Arguments
	/// * `particle_id` - The unique ID of the particle for which to retrieve
	///		field information.
	///
	/// # Panics
	/// This method will panic if there is no particle identified by
	/// 	`particle_id`.
	pub fn get_field_info(&self, particle_id: Uuid) -> Vec<FieldInfo> {
		vec!(FieldInfo {
			radius: 0.0,
			affects_self: false,
			affects_others: false,
			name: String::from("William Beauregard Jefferschmidt IV"),
		})
	}

	/// Starts the simulation.
	pub fn start(&self) {

	}

	/// Pauses the simulation.
	pub fn pause(&self) {

	}

	/// While the simulation is paused, executes a single tick.
	///
	/// # Panics
	/// This method will panic if the simulation is not paused.
	pub fn step(&self) {
	}

	/// Returns the number of elapsed ticks since the start of the simulation.
	pub fn get_elapsed_ticks(&self) -> Ticks {
		Ticks(0)
	}

	/// Returns the number of elapsed simulated seconds since the start of the
	/// simulation.
	pub fn get_elapsed_time(&self) -> Seconds {
		Seconds(0.0)
	}
}
