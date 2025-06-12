use std::collections::HashMap;
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

	/********************* Mass ********************/

	#[test]
	fn mass_supports_partialEq() {
		assert_eq!(Mass(0.0), Mass(0.0));
		assert_eq!(Mass(1.0), Mass(1.0));
		assert_eq!(Mass(-1.0), Mass(-1.0));

		assert_ne!(Mass(-1.0), Mass(0.0));
		assert_ne!(Mass(1.0), Mass(0.0));
		assert_ne!(Mass(-1.0), Mass(1.0));
		assert_ne!(Mass(0.0), Mass(-1.0));
		assert_ne!(Mass(0.0), Mass(1.0));
		assert_ne!(Mass(1.0), Mass(-1.0));
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
			Mass(1.0),
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
		assert_eq!(particle.mass, Mass(1.0));
		assert_eq!(particle.position, Displacement::new(0.0, 0.0));
		assert_eq!(particle.velocity, Velocity::new(0.0, 0.0));
		assert_eq!(particle.fields.len(), 1);
	}


	/********************* Simulation ********************/

	fn dummy_function() {
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
    }

	// Verifies that create_particle() creates a particle with the correct
	//	parameters and that it is added to the particles collection.
	#[test]
	fn simulation_creates_particle() {
		let simulation = Simulation::new(Seconds(1.0), None, None);
		let particle_id_1 = simulation.create_particle(
			Displacement::new(0.0, 0.0),
			Mass(1.0),
			Vec::new(),
		);
		let particle_id_2 = simulation.create_particle(
			Displacement::new(0.0, 0.0),
			Mass(1.0),
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
		assert_eq!(particle_1.mass, Mass(1.0));
		assert!(
			particle_1.fields.is_empty(),
			"particle_1 should have no fields"
		);

		assert_eq!(particle_2.position, Displacement::new(0.0, 0.0));
		assert_eq!(particle_2.mass, Mass(1.0));
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
			Mass(1.0),
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

	// Verifies that the Simulation.apply_force() method adds a force to the
	//	collection of forces to apply on the next tick.
	#[test]
	fn simulation_applies_force() {
		let simulation = Simulation::new(Seconds(1.0), None, None);
		let particle_id = simulation.create_particle(
			Displacement::new(0.0, 0.0),
			Mass(1.0),
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
	fn simulation_gets_mass() {
		let simulation = Simulation::new(Seconds(1.0), None, None);
		let particle_id = simulation.create_particle(
			Displacement::new(0.0, 0.0),
			Mass(1.0),
			Vec::new(),
		);

		let mass = simulation.get_mass(particle_id);

		assert_eq!(Mass(1.0), mass);
	}
	
	#[test]
	fn simulation_gets_position() {
		let simulation = Simulation::new(Seconds(1.0), None, None);
		let particle_id = simulation.create_particle(
			Displacement::new(-1.23, 123.0),
			Mass(1.0),
			Vec::new(),
		);

		let position = simulation.get_position(particle_id);

		assert_eq!(Displacement::new(-1.23, 123.0), position);
	}

	#[test]
	fn simulation_gets_velocity() {
		let mut simulation = Simulation::new(Seconds(1.0), None, None);
		let particle_id = simulation.create_particle(
			Displacement::new(0.0, 0.0),
			Mass(1.0),
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
	fn simulation_gets_field_info() {
		let mut simulation = Simulation::new(Seconds(1.0), None, None);
		let particle_id = simulation.create_particle(
			Displacement::new(0.0, 0.0),
			Mass(1.0),
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

	// Test start().
	// TODO: Decide what start() and pause() should actually do. Maybe just add
	//	a "paused" bool to Simulation. Set it to true when instantiating
	//	Simulation, have start() set it to false, and have pause() set it to
	//	true. The tick() method (or the code that calls tick (some timer event?)
	//	could just not do anything when paused is true).

	// Test pause().

	// Test step().

	// Test get_elapsed_ticks().

	// Test get_elapsed_time().


	/************** Simulation: functional tests ********************/

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
pub struct Seconds(f64);

/// A two-dimensional vector (not to be confused with `Vec<T>`).
/// Supports basic vector math.
#[derive(PartialEq)]
#[derive(Debug)]
pub struct Vector2 {
	x: f64,
	y: f64,
}

impl Vector2 {
	pub fn new(x: f64, y:f64) -> Self {
		// TODO: Intentionally incorrect. Write test, then replace.
		Self {
			x: 999.0,
			y: 999.0,
		}
	}
}

/// Mass.
#[derive(PartialEq)]
#[derive(Debug)]
pub struct Mass(f64);

/// Position in space (displacement from the origin), displacement relative to
/// some starting location, or distance from some arbitrary position.
/// Wraps `Vector2` and provides functionality specific to displacement.
#[derive(PartialEq)]
#[derive(Debug)]
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

/// Velocity.
/// Wraps `Vector2` and provides functionality specific to velocity.
#[derive(PartialEq)]
#[derive(Debug)]
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

/// Force.
/// Wraps `Vector2` and provides functionality specific to forces.
#[derive(PartialEq)]
#[derive(Debug)]
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

/// A type representing a number of ticks.
#[derive(PartialEq)]
#[derive(Debug)]
pub struct Ticks(u64);

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

// TODO: Shoud this (and probably other structs) actually be public? The
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
            mass: Mass(2384928.0),
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
	// The number of simulated seconds that have passed so far.
	// TODO: This should be calculated, not stored in a field.
	// elapsed_time: Seconds,
	// Speed at which the simulation will run, resources permitting. Units are
	//	(simulated seconds) / (real world second). If None, run as fast as
	//	possible.
	simulation_speed: Option<f64>,
	// A function called on each tick. Allows user-defined logic to be driven
	//	by the simulation.
	on_tick: Option<fn()>,
	// Holds forces, keyed by particle_id, to calculate on the next tick.
	applied_forces: HashMap<Uuid, Vec<Force>>,
}

impl Simulation {
	fn tick(&self) {

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
	///		tick.
	///
	///	# Panics
	/// Panics if `tick_duration` or `simulation_speed` is less than or equal to
	/// zero.
	pub fn new(
		tick_duration: Seconds,
		simulation_speed: Option<f64>,
		on_tick: Option<fn()>,
	) -> Self {
	/* TODO: Uncomment this and delete the incorrect code below this.
		Self {
			tick_duration: tick_duration,
			particles: HashMap::new(),
			elapsed_ticks: Ticks(0),
			simulation_speed: simulation_speed,
			on_tick: on_tick,
			applied_forces: HashMap::new(),
		}
	*/
		Self {
			tick_duration: Seconds(-1.0),
			particles: HashMap::new(),
			elapsed_ticks: Ticks(11234124),
			simulation_speed: Some(-1.0),
			on_tick: None,
			applied_forces: HashMap::new(),
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
		// TODO: Remember to panic with a helpful message if particle_id doesn't
		//	exist. Don't just let the HashMap panic. Write "should panic" tests
		//	before implementing.
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
		// TODO: Remember to panic with a helpful message if particle_id doesn't
		//	exist. Don't just let the HashMap panic. Write "should panic" tests
		//	before implementing.
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
		// TODO: Remember to panic with a helpful message if particle_id doesn't
		//	exist. Don't just let the HashMap panic. Write "should panic" tests
		//	before implementing.
		Mass(234234.0)
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
		// TODO: Remember to panic with a helpful message if particle_id doesn't
		//	exist. Don't just let the HashMap panic. Write "should panic" tests
		//	before implementing.
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
		// TODO: Remember to panic with a helpful message if particle_id doesn't
		//	exist. Don't just let the HashMap panic. Write "should panic" tests
		//	before implementing.
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
		// TODO: Remember to panic with a helpful message if particle_id doesn't
		//	exist. Don't just let the HashMap panic. Write "should panic" tests
		//	before implementing.

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
		// TODO: If not paused, panic. Write "should panic" tests before
		//	implementing.
	}

	/// Returns the number of elapsed ticks since the start of the simulation.
	pub fn get_elapsted_ticks(&self) -> Ticks {
		Ticks(0)
	}

	/// Returns the number of elapsed simulated seconds since the start of the
	/// simulation.
	pub fn get_elapsed_time(&self) -> Seconds {
		Seconds(0.0)
	}
}
