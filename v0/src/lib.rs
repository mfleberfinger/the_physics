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

/// A two-dimensional vector (not to be confused with Vec<T>).
/// Supports basic vector math.
pub struct Vector2 {
	x: f64,
	y: f64,
}

/// Mass.
pub struct Mass(f64);

/// Position in space.
/// Wraps Vector2 and provides functionality specific to position.
pub struct Position(Vector2);
// TODO: Implement getters for the underlying x and y values? This would allow
//	me to do my_position.x() and my_position.y() instead of my_position.0.x and
//	my_position.0.y.

/// Velocity.
/// Wraps Vector2 and provides functionality specific to velocity.
pub struct Velocity(Vector2);

/// Force.
/// Wraps Vector2 and provides functionality specific to forces.
pub struct Force(Vector2);

/// Defines a field. A field is a struct implementing a method that is called by
/// the physics engine on each tick in which a particle is within a radius
/// specified by the field, centered on a particle to which the field is
/// attached.
pub trait Field {
}

pub struct Particle {
	/// Mass of the particle.
	mass: Mass,
	/// The particle's location.
	position: Position,
	/// The particle's velocity.
	velocity: Velocity,
	/// Fields attached to the particle.
	// Vec<Box<dyn Field>> is a "trait object". This is apparently necessary to
	//	make a Vec store an unknown type that implements a trait.
	fields: Vec<Box<dyn Field>>,
	/// Uniquely identifies this particle.
	id: Uuid,
}

/// Owns all of the data that constitutes a physics simulation. Provides methods
/// for interacting with and running the simulation.
pub struct Simulation {
	/// The number of simulated seconds that elapse in a single tick.
	/// This is effectively the resolution of the simulation.
	tick_duration: Seconds,
	/// A collection that owns all particles in the simulation.
	particles: HashMap<Uuid, Particle>,
	/// The number of ticks that have passed so far.
	elapsed_ticks: u64,
	/// The number of simulated seconds that have passed so far.
	elapsed_time: Seconds,
	/// Speed at which the simulation will run, resources permitting. Units are
	/// (simulated seconds) / (real world second).
	simulation_speed: f64,
}
