use crate::{physical_quantities, simulation};
use uuid::Uuid;


#[cfg(test)]
mod tests {
    use super::*;

	/********************* Particle ********************/

	#[test]
	fn new_creates_particle() {
		let particle = Particle::new(
			physical_quantities::Mass::new(1.0),
			physical_quantities::Displacement::new(0.0, 0.0),
			physical_quantities::Velocity::new(0.0, 0.0),
			vec!(Box::new(
				DummyField {
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

	/// Called by the simulation to get the field's radius.
	fn get_radius(&self) -> f64;

	// TODO: It would probably be better to give effect() a parameter to
	//	accept the ID of the particle to which it's attached. Then effect() could decide
	//	whether or not to affect that particle and would be able to take
	//	different actions for the particle to which it's attached than for any
	//	other particles it might effect. For example, maybe effect() could
	//	delete any other particles that enter the field and increase the mass
	//	of the particle to which the field is attached for each particle deleted
	//	to simulate it "absorbing" other particles. Maybe save this change for
	//	a later version. It doesn't seem important.
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

#[cfg(test)]
pub struct DummyField {
	pub radius: f64,
	pub affects_self: bool,
	pub affects_others: bool,
	pub name: String,
}

#[cfg(test)]
impl Field for DummyField {
	fn effect(
		&self,
		simulation: &simulation::Simulation,
		position: physical_quantities::Displacement,
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
	pub fn new(acceleration: physical_quantities::Acceleration, name: Option<String>)
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
		_position: physical_quantities::Displacement,
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

	pub fn get_radius(&self) -> f64 {
		self.radius
	}

	pub fn get_affects_self(&self) -> bool {
		self.affects_self
	}

	pub fn get_affects_others(&self) -> bool {
		self.affects_others
	}

	pub fn get_name(&self) -> &String {
		&self.name
	}
}

// TODO: Should this (and probably other structs) actually be public? The
//	Simulation's interface is written in a way that assumes none of this
//	struct's fields will be directly accessible by the user. It does need to be
//	public to be accessible from other modules (e.g., the simulation module).
//	Maybe there's a way to make it accessible from modules within the library,
//	but inaccessible to a user of the library, if desirable.
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
		Self {
			mass: mass,
			position: position,
			velocity: velocity,
			fields: fields,
			id: Uuid::new_v4(),
		}
	}

	pub fn get_mass(&self) -> physical_quantities::Mass {
		self.mass
	}

	pub fn get_position(&self) -> physical_quantities::Displacement {
		self.position
	}

	pub fn get_velocity(&self) -> physical_quantities::Velocity {
		self.velocity
	}

	pub fn get_id(&self) -> Uuid {
		self.id
	}

	pub fn get_field_info(&self) -> Vec<FieldInfo> {
		let mut field_info_vec: Vec<FieldInfo> = Vec::new();

		for field in self.fields.iter() {
			let field_info = FieldInfo::new(
				field.get_radius(),
				field.affects_self(),
				field.affects_others(),
				field.get_name().to_string(),
			);
			field_info_vec.push(field_info);
		}

		field_info_vec
	}

	pub(crate) fn get_fields(&self) -> &Vec<Box<dyn Field>> {
		&self.fields
	}

	// TODO: Directly setting physical quantities could be fun, but might cause
	//	issues. Reconsider later.
	//pub fn set_mass(&mut self, mass: physical_quantities::Mass) {
	//	self.mass = mass;
	//}

	//pub fn set_position(&mut self, position: physical_quantities::Displacement) {
	//	self.position = position;
	//}

	//pub fn set_velocity(&mut self, velocity: physical_quantities::Velocity) {
	//	self.velocity = velocity;
	//}

	// Given a list of forces and an amount of time; add up the forces,
	//	calculate acceleration by dividing the sum by this particle's mass,
	//	calculate the change in velocity, and add it to this particle's current
	//	velocity.
	pub fn accelerate(
		&mut self,
		forces: &Vec<physical_quantities::Force>,
		time: physical_quantities::Time
	) {
		let mut x = 0.0;
		let mut y = 0.0;
		for force in forces {
			x += force.x();
			y += force.y();
		}
		let total_force = physical_quantities::Force::new(x, y);
		let acceleration = total_force / self.mass;
		self.velocity += acceleration * time;
	}

	// Given an amount of time, set the particle's new position based on its
	//	velocity.
	pub fn coast(&mut self, time: physical_quantities::Time) {
		self.position += self.velocity * time;
	}
}
