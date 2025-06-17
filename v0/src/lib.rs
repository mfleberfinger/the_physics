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
			simulation: Simulation,
			position: Displacement,
			particles: Vec<Particle>
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

	/********************* Mass ********************/

	#[test]
	fn mass_supports_partialEq() {
		assert_eq!(Mass::new(0.0), Mass::new(0.0));
		assert_eq!(Mass::new(1.0), Mass::new(1.0));

		assert_ne!(Mass::new(1.0), Mass::new(0.0));
		assert_ne!(Mass::new(0.0), Mass::new(1.0));
	}

	#[test]
	#[should_panic(expected = "Mass must be positive.")]
	fn mass_new_panics_if_not_positive() {
		let m = Mass::new(0.0);
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

	// Verifies that an applied force causes the expected increase in velocity
	//	and that a velocity actually causes the expected displacement.
	#[test]
	fn simulation_step_simulates_force() {
		let force = Force::new(1.0, 0.0);
		let mass = Mass::new(1.0);
		let tick_duration = Seconds(1.0);
		let expected_velocity;
		let mut expected_displacement;
		let mut simulation = Simulation::new(tick_duration, None, None);
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
		//	duration. The actual dsiplacement should be exactly as calculated by
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
		assert_eq!(simulation.particles.len(), 1);
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

	// TODO: Write a test to verify that a field attached to a particle will
	//	be called and passed a list of all particles within its radius when a
	//	tick (step()) occurs.

	// TODO: Write a test to verify that a field configured to affect its own
	//	particle will be passed its own particle ID.

	// TODO: Write a test to verify that a field configured not to affect its
	//	own particle will not be passed its own particle ID.

	/************** Simulation: functional tests ********************/

	// TODO: Work on these and any operations and skeleton code required to
	//	implement them.

	// Verifies that the velocity of a particle is set correctly when a force
	//	is applied. Should test multiple edge cases (positive values, negative
	//	values, 0 mass (if this is allowed), multiple directions, particle with
	//	and without velocity, multiple forces, others?).
	// Resulting velocity may not be precisely the same as calculated velocity
	//	due to the tick-based nature of the simulation. Need to decide what
	//	level of error is acceptable for a given tick length and number of
	//	ticks.
	#[test]
	fn simulation_calculates_velocity() {
	}

	// Creates a particle with a self-affecting gravity field and launches it
	//	with a known force. Uses equations of motion to calculate the expected
	//	position, velocity, energy, etc. of the particle at different times.
	// Resulting values may not be precisely the same as calculated values
	//	due to the tick-based nature of the simulation. Need to decide what
	//	level of error is acceptable for a given tick length and number of
	//	ticks.
	#[test]
	fn trajectory_test() {
	}
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

	fn add(self, other: Self) -> Self::Output {
		Self::new(self.x() + other.x(), self.y() + other.y())
	}
}

impl ops::AddAssign for Displacement {
	fn add_assign(&mut self, other: Self) {
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

/// A type representing a number of ticks.
#[derive(PartialEq)]
#[derive(Debug)]
#[derive(Clone, Copy)]
pub struct Ticks(u64);

// TODO: Implement a rigid body collider Field as part of the library. It could
//	expose parameters (e.g. coefficient of friction, coefficient of restitution)
//	as fields of the struct.
/// Defines a field. A field is a struct implementing a method that is called by
/// the physics engine on each tick in which a particle is within a radius
/// specified by the field, centered on a particle to which the field is
/// attached.
pub trait Field {
	/// Determines what happens when the field is triggered.
	/// # Arguments
	/// * `simulation` - The Simulation that calls the effect function.
	/// # `position` - The position of the particle to which this field is
	///		attached. The center of the field.
	/// * `particles` - All particles affected by the field. Determined by
	///		the simulation.
	fn effect(
		&self,
		simulation: Simulation,
		position: Displacement,
		particles: Vec<Particle>
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
