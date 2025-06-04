use std::collections::HashMap;
use uuid::Uuid;

pub fn add(left: u64, right: u64) -> u64 {
    left + right
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}

// Using a tuple struct to wrap an f64 so the compiler treats Seconds as a
//	distinct type. This is the "newtype pattern."
/// Time, in seconds.
pub struct Seconds(f64);

/// A two-dimensional vector (not to be confused with `Vec<T>`).
/// Supports basic vector math.
pub struct Vector2 {
	x: f64,
	y: f64,
}

/// Mass.
pub struct Mass(f64);

/// Position in space.
/// Wraps `Vector2` and provides functionality specific to position.
pub struct Position(Vector2);
// TODO: Implement getters for the underlying x and y values? This would allow
//	me to do my_position.x() and my_position.y() instead of my_position.0.x and
//	my_position.0.y.

/// Velocity.
/// Wraps `Vector2` and provides functionality specific to velocity.
pub struct Velocity(Vector2);

/// Force.
/// Wraps `Vector2` and provides functionality specific to forces.
pub struct Force(Vector2);

/// A type representing a number of ticks.
pub struct Ticks(u64);

/// Defines a field. A field is a struct implementing a method that is called by
/// the physics engine on each tick in which a particle is within a radius
/// specified by the field, centered on a particle to which the field is
/// attached.
pub trait Field {
	// TODO: Define a function signature that takes a Vec<Particle>, which will
	//	hold all particles within the field's radius, as determined by the
	//	physics engine.
}

pub struct Particle {
	mass: Mass,
	position: Position,
	velocity: Velocity,
	// Vec<Box<dyn Field>> is a "trait object". This is apparently necessary to
	//	make a Vec store an unknown type that implements a trait.
	fields: Vec<Box<dyn Field>>,
	id: Uuid,
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
	elapsed_time: Seconds,
	// Speed at which the simulation will run, resources permitting. Units are
	//	(simulated seconds) / (real world second).
	simulation_speed: f64,
	// A function called on each tick. Allows user-defined logic to be driven
	//	by the simulation.
	on_tick: Option<fn()>,
}

impl Simulation {
	/// Creates an instance of `Simulation`.
	///
	/// # Arguments
	/// * `tick_duration` - The number of simulated seconds that elapse in a
	///		single tick. Effectively the resolution of the simulation.
	/// * `simulation_speed` - The speed at which the simulation will run,
	///		resources permitting.
	///		Units are (simulated seconds) / (real world second).
	/// * `on_tick` - A function that will be called by the simulation on each
	///		tick.
	pub fn new(
		tick_duration: Seconds,
		simulation_speed: f64,
		on_tick: Option<fn()>,
	) -> Self {
		Self {
			tick_duration: tick_duration,
			particles: HashMap::new(),
			elapsed_ticks: Ticks(0),
			elapsed_time: Seconds(0.0),
			simulation_speed: simulation_speed,
			on_tick: on_tick,
		}
	}

	fn tick(&self) {

	}

	/// Creates a new particle and adds it to the simulation. Returns that
	/// particle's unique ID.
	///
	/// # Arguments
	/// * `position` - The particle's coordinates.
	/// * `mass` - The particle's mass.
	/// * `fields` - Fields to attach to the particle.
	pub fn create_particle(
		&self,
		position: Position,
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
		// TODO: If not paused, panic.
	}

	/// Returns the number of elapsed ticks since the start of the simulation.
	pub fn get_elapsted_ticks(&self) -> Ticks {
		Ticks(0)
	}

	/// Returns the number of elapsed simulated seconds since the start of the
	/// simulation.
	pub fn get_elapsed_time(&self) -> Seconds {
		Seconds(0)
	}
}
