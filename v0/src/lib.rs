use std::collections::HashMap;
use uuid::Uuid;

pub mod units_of_measure;

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
			Time(1.0),
			None,
			None,
		);
		assert_eq!(
			simulation.tick_duration,
			Time(1.0),
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
			Time(1.0),
			Some(1.0),
			Some(dummy_function),
		);
		assert_eq!(
			simulation.tick_duration,
			Time(1.0),
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
		let simulation = Simulation::new(Time(-1.0), None, None);
	}

	#[test]
	#[should_panic(expected = "simulation_speed must be positive")]
	fn simulation_new_panics_on_negative_simulation_speed() {
		let simulation = Simulation::new(Time(1.0), Some(-1.0), None);
	}

	// Verifies that create_particle() creates a particle with the correct
	//	parameters and that it is added to the particles collection.
	#[test]
	fn simulation_creates_particle() {
		let simulation = Simulation::new(Time(1.0), None, None);
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
		let simulation = Simulation::new(Time(1.0), None, None);
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
		let simulation = Simulation::new(Time(1.0), None, None);
		simulation.delete_particle(Uuid::new_v4());
	}

	// Verifies that the Simulation.apply_force() method adds a force to the
	//	collection of forces to simulate on the next tick.
	#[test]
	fn simulation_applies_force() {
		let simulation = Simulation::new(Time(1.0), None, None);
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
		let simulation = Simulation::new(Time(1.0), None, None);
		simulation.apply_force(Uuid::new_v4(), Force::new(1.0, 1.0));
	}

	#[test]
	fn simulation_gets_mass() {
		let simulation = Simulation::new(Time(1.0), None, None);
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
		let simulation = Simulation::new(Time(1.0), None, None);
		simulation.get_mass(Uuid::new_v4());
	}
	
	#[test]
	fn simulation_gets_position() {
		let simulation = Simulation::new(Time(1.0), None, None);
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
		let simulation = Simulation::new(Time(1.0), None, None);
		simulation.get_position(Uuid::new_v4());
	}

	#[test]
	fn simulation_gets_velocity() {
		let mut simulation = Simulation::new(Time(1.0), None, None);
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
		let simulation = Simulation::new(Time(1.0), None, None);
		simulation.get_velocity(Uuid::new_v4());
	}

	#[test]
	fn simulation_gets_field_info() {
		let mut simulation = Simulation::new(Time(1.0), None, None);
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
		let simulation = Simulation::new(Time(1.0), None, None);
		simulation.get_field_info(Uuid::new_v4());
	}

	#[test]
	fn simulation_starts() {
		let mut simulation = Simulation::new(Time(1.0), None, None);

		// Force the simulation to be paused, in case the constructor is broken.
		simulation.is_paused = true;
		
		simulation.start();

		assert!(!simulation.is_paused, "The simulation should have unpaused.");
	}

	#[test]
	fn simulation_pauses() {
		let mut simulation = Simulation::new(Time(1.0), None, None);

		// Force the simulation to be unpaused.
		simulation.is_paused = false;
		
		simulation.pause();

		assert!(simulation.is_paused, "The simulation should have paused.");
	}

	// Simulation.step()...

	#[test]
	fn simulation_step_increments_elapsed_ticks() {
		let mut simulation = Simulation::new(Time(1.0), None, None);

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
			Time(1.0),
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
		let mut simulation = Simulation::new(Time(1.0), None, None);
		simulation.is_paused = false;
		simulation.step();
	}

	#[test]
	fn simulation_gets_elapsed_ticks() {
		let mut simulation = Simulation::new(Time(1.0), None, None);
		simulation.elapsed_ticks = Ticks(1);

		let elapsed_ticks = simulation.get_elapsed_ticks();

		assert_eq!(Ticks(1), elapsed_ticks);
	}

	#[test]
	fn simulation_gets_elapsed_time() {
		let mut simulation = Simulation::new(Time(0.001), None, None);
		simulation.elapsed_ticks = Ticks(1000);

		let elapsed_time = simulation.get_elapsed_time();

		// elapsed time = (elapsed ticks) * (ticks duration)
		assert_eq!(Time(1.0), elapsed_time);
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
		let simulation = Simulation::new(Time(1.0), None, None);
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
		let simulation = Simulation::new(Time(1.0), None, None);
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
		let tick_duration = Time(1.0);
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
		let tick_duration = Time(1.0);
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
		let tick_duration = Time(0.005);
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
		let tick_0 = Time(0.001);
		let tick_1 = Time(0.0002);
		let tick_2 = Time(0.00003);
		let tick_3 = Time(0.000004);
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

	// TODO: Should probably treat `error` as a percent.
	fn displacements_are_almost_equal(
		d1: Displacement,
		d2: Displacement,
		error: f64
	) -> bool {
		let diff = d1 - d2;
		diff.x().abs() <= error && diff.y().abs() <= error
	}

	// TODO: Should probably treat `error` as a percent.
	fn velocities_are_almost_equal(
		v1: Velocity,
		v2: Velocity,
		error: f64
	) -> bool {
		let diff = v1 - v2;
		diff.x().abs() <= error && diff.y().abs() <= error
	}

	// TODO: Should probably treat `error` as a percent.
	fn times_are_almost_equal(
		t1: Time,
		t2: Time,
		error: f64
	) -> bool {
		let diff = t1 - t2;
		diff.0.abs() <= error
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
		let tick_duration = Time(0.001);
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
		let mut elapsed_time = Time(0.0);
		let mut time_since_last_round = Time(0.0);
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
        let tick_duration = Time(0.001);
        let simulation = Simulation::new(tick_duration, None, None);
        let force = Force::new(250.0, 500.0);
        let force_duration = Time(3.0);
        let mass = Mass::new(5.0);
        let mut expected_position = Displacement::new(0.0, 0.0);
        let mut actual_position = Displacement::new(0.0, 0.0);
		let g = -9.81;
		let gravitational_acceleration = Acceleration::new(0.0, g);
        let gravity_field = SimpleSelfGravityField::new(
			gravitational_acceleration,
            None,
        );
        let particle_id = simulation.create_particle(
            expected_position,
            mass,
            vec!(Box::new(gravity_field)),
        );

		// Forcing phase.
        // Apply the force for a set duration.
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
            "After force applied, position error greater than permissible error of {:?}.\n\
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
		let t_f = actual_force_duration.0;
        let v_yf = ((force / mass) * actual_force_duration).y();
        let t_1 = (-v_yf / g) + t_f;
        // Replace the math-friendly name with a programmer-friendly name and
		//	convert it to the Time type.
        let expected_time_to_peak = Time(t_1);

        // Calculate the expected maximum height.
        // y_f = 0.5 * ((f_0y / m) + g) * t_f^2
        // y_max = y_f + v_yf * (t_1 - t_f) + 0.5 * g * (t_1 - t_f)^2
        // Where...
        // y_f is the height achieved while the force was acting.
        // f_0y is the y-component of the force.
        // m is the mass of the particle.
        // y_max is the maximum height.
		let y_f = 0.5 * ((force.y() / mass.0) + g) * t_f * t_f;
		let y_max = y_f + v_yf * (t_1 - t_f) + 0.5 * g * (t_1 - t_f).powf(2.0);

		// Calculate the x position when the particle reaches y_max.
        // v_xf is x-velocity immediately after the force is done acting.
		// x_ymax is the x-position when the particle reaches its max height.
        let v_xf = ((force / mass) * actual_force_duration).x();
		let x_ymax = v_xf * t_1;
		let expected_peak = Displacement::new(x_ymax, y_max);

        // Calculate how long the particle should take to fall from its maximum
        //  height.
        // t_2 = sqrt((-2 * y_max) / g)
        // Where t_2 is the time taken for the particle to fall from its maximum
        //  height.
		let t_2 = ((-2.0 * y_max) / g).sqrt();

        // Calculate the particle's total flight time (from when the force is
        //  first applied to when the particle falls to y = 0).
        // t_omega = t_1 + t_2
        // Where t_omega is total flight time.
		let t_omega = t_1 + t_2;
        // Replace the math-friendly name with a programmer-friendly name and
		//	change the type.
		let expected_total_flight_time = Time(t_omega);

        // Calculate total distance the particle should travel (on the x-axis)
		//	before falling past y = 0.
        // d = 0.5 * (f_0x / m) * t_f^2 + ((f_0x / m) * t_f) * (t_omega - t_f)
        // Where...
        // d is the total distance.
        // f_0x is the x-component of the force.
        let x_distance = 0.5 * (force.x() / mass.0) * t_f * t_f +
			((force.x() / mass.0) * t_f) * (t_omega - t_f);
		let expected_final_position = Displacement::new(x_distance, 0.0);

		// Coasting phase.
        // Step until the particle returns to y = 0, or until enough time has
        //  passed that we know it should have returned to y = 0.
		let mut actual_peak = Displacement::new(-1.0, -1.0);
		let mut actual_time_to_peak = Time(0.0);
		let mut actual_velocity;
        while actual_position.y() > 0.0
			&& expected_total_flight_time >
				simulation.get_elapsed_time() + Time(10.0) {

            // As the particle coasts, we will assert that its position is
			//	correct from one tick to the next, given its previous actual
			//	position.
			// Need to consider previous/current velocity and gravitational
			//	acceleration.
			// The expectation is that the velocity reported by the simulation
			//	is the velocity used to calculate the particle's updated
			//	position in the previous tick. We will use actual velocity to
			//	calculate the expected position for the next tick.
			// We use the formula
			//	d = d_0 + v_0 * t + (1/2) * a * t^2
			//	to achieve this.
			actual_velocity = simulation.get_velocity(particle_id);
			expected_position =
				actual_position
				+ (actual_velocity * tick_duration)
				+ (0.5 * gravitational_acceleration
					* tick_duration * tick_duration);

			// Run the next tick.
			simulation.step();

			actual_position = simulation.get_position(particle_id);

			assert!(
				displacements_are_almost_equal(
					expected_position,
					actual_position,
					permissible_error
				),
				"In-flight position error greater than permissible error of {:?}.\n\
				expected_position = {:?}\n\
				actual_position = {:?}\n\
				actual - expected = {:?}\n\
				elapsed ticks: {:?}",
				permissible_error,
				expected_position,
				actual_position,
				actual_position - expected_position,
				simulation.get_elapsed_ticks(),
			);

            // Save the time and position of the highest point in the trajectory.
			if (actual_position.y() > actual_peak.y()) {
				actual_peak = actual_position;
				actual_time_to_peak = simulation.get_elapsed_time();
			}
        }

        // Assert that the actual time and position of the peak were correct
		//	when compared to the expected (calculated) time and position, within
		//	permissible error.
		assert!(
			times_are_almost_equal(
				expected_time_to_peak,
				actual_time_to_peak,
				permissible_error,
			),
			"Time-to-peak error greater than permissible error of {:?}.\n\
			expected_time_to_peak = {:?}\n\
			actual_time_to_peak = {:?}\n\
			actual - expected = {:?}",
			permissible_error,
			expected_time_to_peak,
			actual_time_to_peak,
			actual_time_to_peak - expected_time_to_peak,
		);

		assert!(
			displacements_are_almost_equal(
				expected_peak,
				actual_peak,
				permissible_error,
			),
			"Peak position error greater than permissible error of {:?}.\n\
			expected_peak = {:?}\n\
			actual_peak = {:?}\n\
			actual - expected = {:?}",
			permissible_error,
			expected_peak,
			actual_peak,
			actual_peak - expected_peak,
		);

        // Assert that the actual time and position of the zero-crossing (i.e.,
		//	current time and position) were correct when compared to the
		//	expected/calculated time and position, within permissible error.
		let actual_total_flight_time = simulation.get_elapsed_time();
		assert!(
			times_are_almost_equal(
				expected_total_flight_time,
				actual_total_flight_time,
				permissible_error,
			),
			"Total flight time error greater than permissible error of {:?}.\n\
			expected_total_flight_time = {:?}\n\
			actual_total_flight_time = {:?}\n\
			actual - expected = {:?}",
			permissible_error,
			expected_total_flight_time,
			actual_total_flight_time,
			actual_total_flight_time - expected_total_flight_time,
		);

		let actual_final_position = simulation.get_position(particle_id);
		assert!(
			displacements_are_almost_equal(
				expected_final_position,
				actual_final_position,
				permissible_error,
			),
			"Final position error greater than permissible error of {:?}.\n\
			expected_final_position = {:?}\n\
			actual_final_position = {:?}\n\
			actual - expected = {:?}",
			permissible_error,
			expected_final_position,
			actual_final_position,
			actual_peak - expected_peak,
		);
	}


	// Creates two particles with rigid body fields. Launches one of those
	//	particles at the other and verifies that the resulting velocities (speed
	//	and *direction*) and kinetic energy of each particle are as expected of
	//	an elastic collision.
	// Resulting values may not be precisely the same as calculated values
	//	due to the tick-based nature of the simulation and floating point error.
	//	Need to decide what level of error is acceptable for a given tick length
	//	and number of ticks.
	// This might not be worth doing. The design of the physics engine didn't
	//	really intend for individual particles to act as rigid bodies. Similar
	//	interactions to rigid body collisions are expected to emerge from
	//	particles pushing and pulling each other via fields.
//	#[test]
//	fn functional_collision() {
//	}


	// TODO: When done writing tests, re-read the Rust book's chapter on project
	//	organization and break this project into multiple files. Probably give
	//	each struct its own file.
}


// TODO: Implement a "rigid body" (or "basic collider") collider Field as part of the library. It
//	could expose parameters (e.g. coefficient of friction, coefficient of
//	restitution) as fields of the struct.
// 2025-08-05: Implementing a rigid body collider might be hacky without changing
//	my design at least a little. The intent was never to treat individual
//	particles as rigid bodies. A "rigid body" might emerge as a result of
//	interactions between multiple particles, but probably won't be its own field.
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
	tick_duration: Time,
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
	/// * `tick_duration` - The amount of simulated time that elapses in a
	///		single tick. Effectively the resolution of the simulation.
	/// * `simulation_speed` - The speed at which the simulation will run,
	///		resources permitting.
	///		Units can be though of as, for eaxmple,
	///		(simulated seconds) / (real world second).
	///		If None is specified, the simulation will run as fast as possible.
	/// * `on_tick` - A function that will be called by the simulation on each
	///		tick. Takes the simulation itself as a parameter to allow the user
	///		to write code that influences the simulation.
	///
	///	# Panics
	/// Panics if `tick_duration` or `simulation_speed` is less than or equal to
	/// zero.
	pub fn new(
		tick_duration: Time,
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
			tick_duration: Time(-1.0),
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
	///		velocity.
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

	/// Returns the returns the amount of simulated time (e.g., seconds) since
	/// the start of the simulation.
	pub fn get_elapsed_time(&self) -> Time {
		Time(0.0)
	}
}
