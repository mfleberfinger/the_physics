use crate::{physical_quantities, simulation};
use uuid::Uuid;


#[cfg(test)]
mod tests {
	extern crate test_utilities;
    use super::*;

	/********************* Particle ********************/

	#[test]
	fn new_creates_particle() {
		let particle = Particle::new(
			physical_quantities::Mass::new(1.0),
			physical_quantities::Displacement::new(0.0, 0.0),
			physical_quantities::Velocity::new(0.0, 0.0),
			vec!(Box::new(
				test_utilities::DummyField {
					radius: 1.0,
					affects_self: false,
					affects_others: false,
					name: String::from("dummy"),
				}
			)),
		);
		assert_eq!(particle.mass, physical_quantities::Mass::new(1.0));
		assert_eq!(particle.position, physical_quantities::Displacement::new(0.0, 0.0));
		assert_eq!(particle.velocity, physical_quantities::Velocity::new(0.0, 0.0));
		assert_eq!(particle.fields.len(), 1);
	}

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
		simulation: &simulation::Simulation,
		position: physical_quantities::Displacement,
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
	acceleration: physical_quantities::Acceleration,
	name: String,
}

impl SimpleSelfGravityField {
	/// Creates an instance of `SimpleSelfGravityField `.
	///
	/// # Arguments
	/// * `acceleration` - The acceleration due to gravity, "little 'g'."
	/// * `name` - The field name. Defaults to "SimpleSelfGravityField" if None.
	fn new(acceleration: physical_quantities::Acceleration, name: Option<String>)
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
		simulation: &simulation::Simulation,
		position: physical_quantities::Displacement,
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

impl FieldInfo {
	pub fn new(
		radius: f64,
		affects_self: bool,
		affects_others: bool,
		name: String,
	) -> Self {
		Self {
			radius: radius,
			affects_self: affects_self,
			affects_others: affects_others,
			name: name,
		}
	}
}

// TODO: Should this (and probably other structs) actually be public? The
//	Simulation's interface is written in a way that assumes none of this
//	struct's fields will be directly accessible by the user.
/// Represents an infinitesimal massive particle. Stores the particle's mass,
/// position, velocity, and attached `Field`s.
pub struct Particle {
	mass: physical_quantities::Mass,
	position: physical_quantities::Displacement,
	velocity: physical_quantities::Velocity,
	// Vec<Box<dyn Field>> is a "trait object". This is apparently necessary to
	//	make a Vec store an unknown type that implements a trait.
	fields: Vec<Box<dyn Field>>,
	id: Uuid,
}

impl Particle {
	pub fn new(
		mass: physical_quantities::Mass,
		position: physical_quantities::Displacement,
		velocity: physical_quantities::Velocity,
		fields: Vec<Box<dyn Field>>,
	) -> Self {
		// TODO: Intentionally incorrect placeholder code. Write tests, then
		//	replace.
		Self {
			mass: physical_quantities::Mass::new(2384928.0),
			position: physical_quantities::Displacement::new(45345.0, 43434.0),
			velocity: physical_quantities::Velocity::new(45345.0, 43434.0),
			fields: Vec::new(),
			id: Uuid::new_v4(),
		}
	}
}
