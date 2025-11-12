use crate::{physical_quantities, simulation, utilities};
use std::collections::HashMap;
use uuid::Uuid;


#[cfg(test)]
mod tests {
    use super::*;

	
	/********************* Collider ********************/

	#[test]
	fn new_creates_collider() {
		let collider1 = Collider::new(
			100.0,
			50.0,
			Some(String::from("SomeName")),
		);
		let collider2 = Collider::new(
			50.0,
			100.0,
			None,
		);

		assert_eq!(100.0, collider1.get_radius());
		assert_eq!(50.0, collider1.coefficient_of_restitution);
		assert_eq!(&String::from("SomeName"), collider1.get_name());

		assert_eq!(50.0, collider2.get_radius());
		assert_eq!(100.0, collider2.coefficient_of_restitution);
		assert_eq!(&String::from("Collider"), collider2.get_name());
	}


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

/// Defines a field. A field is a struct implementing a method that is called by
/// the physics engine on each tick in which a particle is within a radius
/// specified by the field, centered on a particle to which the field is
/// attached.
pub trait Field {
	/// Determines what happens when the field is triggered.
	/// # Arguments
	/// * `simulation` - The Simulation that called the effect function.
	/// * `position` - The position of the particle to which this field is
	///		attached. The center of the field.
	/// * `triggered_by` - A HashMap containing the IDs of all particles that
	///		should be affected by the field, as well as information about any of
	///		those particles' fields that overlap with this field, if this
	///		field's effect triggers on fields. The HashMap is keyed by particle
	///		ID and contains vectors of `Option<FieldInfo>` as its values.
	///		If a vector contains a `None` value, there are two possibilities:
	///		Either the field is set to affect itself and the particle is in the
	///		HashMap because it is the field owner (in this case, the vector
	///		will only have one element) OR the field triggers on particles and
	///		the particle is inside the field (in this case, the collection may
	///		also contain Some(FieldInfo) values if the field is triggered by
	///		field overlap).
	/// * `field_owner_id` - The ID of the particle to which this field is
	///		attached.
	fn effect(
		&self,
		simulation: &simulation::Simulation,
		position: physical_quantities::Displacement,
		triggered_by: HashMap<Uuid, Vec<Option<FieldInfo>>>,
		field_owner_id: Uuid,
	);

	/// Called by the simulation to get the field's radius.
	fn get_radius(&self) -> f64;

	/// Called by the simulation to determine whether this field affects the
	///	particle to which it's attached.
	fn affects_self(&self) -> bool;

	/// Called by the simulation to determine whether this field affects
	/// particles other than the particle to which it's attached.
	fn affects_others(&self) -> bool;

	/// Called by the simulation to determine whether this field should affect
	/// other particles when it overlaps with fields attached to those
	/// particles.
	fn triggers_on_fields(&self) -> bool;

	/// Called by the simulation to determine whether this field should affect
	/// other particles when those particles are contained within this field.
	fn triggers_on_particles(&self) -> bool;

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
#[derive(Clone)]
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
		triggered_by: HashMap<Uuid, Vec<Option<FieldInfo>>>,
		field_owner_id: Uuid,
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

	fn triggers_on_fields(&self) -> bool {
		false
	}

	fn triggers_on_particles(&self) -> bool {
		false
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
	/// * `name` - The field name. Defaults to "SimpleSelfGravityField" if `None`.
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
		triggered_by: HashMap<Uuid, Vec<Option<FieldInfo>>>,
		_field_owner_id: Uuid,
	) {
		// There should only ever be one thing in the Vec (the particle to
		//  which this field is attached).
		for id in triggered_by.keys() {
			let force = simulation.get_mass(*id) * self.acceleration;
			simulation.apply_force(*id, force);
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

	fn triggers_on_fields(&self) -> bool {
		false
	}

	fn triggers_on_particles(&self) -> bool {
		false
	}

	fn get_name(&self) -> &String {
		&self.name
	}
}

/// Makes a particle apply a gravitational pull to other particles within the
/// field's radius.
/// #Notes
///	In order to apply realistic gravitational forces, all particles affected by
/// this field must have their own gravity fields with equal radius. Otherwise,
/// it will be possible for one particle to be within the gravity of another
///	particle, but not have that other particle within its own gravity field.
/// This would lead to the law of "equal and opposite reactions" being ignored
/// (i.e. one particle would experience a force while the other would not).
// TODO: This Field's effect should probably only apply forces to particles
//	containing a field of the same name as this field. Otherwise, all particles
//	in a simulation with a UniversalGravitationField would be affected by it.
pub struct UniversalGravitationField {
	radius: f64,
	gravitational_constant: f64,
	name: String,
}

impl UniversalGravitationField {

	/// Creates an instance of `UniversalGravitationField`.
	///
	/// # Arguments
	/// * `gravitational_constant` - The gravitational constant, G. If this is
	///		`None`, the real world value of 6.6743e−11 will be used.
	/// * `name` - The field name. Defaults to "UniversalGravitationField" if
	///		`None`.
	pub fn new(
		radius: f64,
		gravitational_constant: Option<f64>,
		name: Option<String>)
		-> UniversalGravitationField
	{
		let field_name = match name {
			Some(s) => s,
			None => String::from("UniversalGravitationField"),
		};

		let big_g = match gravitational_constant {
			Some(g) => g,
			None => 6.6743e-11,
		};

		UniversalGravitationField {
			radius: radius,
			gravitational_constant: big_g,
			name: field_name,
		}
	}
}

impl Field for UniversalGravitationField {
	fn effect(
		&self,
		simulation: &simulation::Simulation,
		position: physical_quantities::Displacement,
		triggered_by: HashMap<Uuid, Vec<Option<FieldInfo>>>,
		field_owner_id: Uuid,
	) {
		for id in triggered_by.keys() {
			// We want to calculate
			// F = G * ((m_1 * m_2) / |r_12|^2) * ru_12
			// Where r_12 is the vector from the other particle to this field's
			//	owner particle and ru_12 is the unit vector derived from r_12.
			let other_position = simulation.get_position(*id);
			let displacement_vector = utilities::get_displacement_vector(
				other_position,
				position,
			);
			let unit_vector = displacement_vector.get_vector().get_unit_vector();
			// Magnitude = sqrt(x^2 + y^2) => Magnitude^2 = x^2 + y^2
			let magnitude_squared = displacement_vector.x().powf(2.0) + displacement_vector.y().powf(2.0);

			// Don't divide by 0. If we ever encounter this situation, it seems
			//	extremely unlikely that it would last for more than one tick if
			//	things are allowed to move (i.e., the user isn't intentionally
			//	pinning two particles to the same location).
			if magnitude_squared > 0.0 {
				let force_vector =
					self.gravitational_constant
					* (
					(simulation.get_mass(field_owner_id).get_number() * simulation.get_mass(*id).get_number())
					/ magnitude_squared) * unit_vector;

				let force =
					physical_quantities::Force::new(force_vector.x(), force_vector.y());

				simulation.apply_force(*id, force);
			}
		}
	}

	fn get_radius(&self) -> f64 {
		self.radius
	}

	fn affects_self(&self) -> bool {
		false
	}

	fn affects_others(&self) -> bool {
		true
	}

	fn triggers_on_fields(&self) -> bool {
		false
	}

	fn triggers_on_particles(&self) -> bool {
		true
	}

	fn get_name(&self) -> &String {
		&self.name
	}
}

/// Allows collisions between two or more particles to be simulated, instead of
///		letting particles pass through each other. Each particle involved in a
///		collision must have colliders with the same name in order to be affected.
pub struct Collider {
	radius: f64,
	coefficient_of_restitution: f64,
	name: String,
}

impl Collider {

	/// Creates an instance of `Collider`.
	///
	/// # Arguments
	/// * `coefficient_of_restitution` - How "bouncy" the particle will be. In
	///		a collision between two objects, the coefficient of restitution of
	///		the collision will be the product of those two objects'
	///		`coefficient_of_restitution` values. A collision where this product
	///		is 1 will be perfectly elastic. A collision where this product is
	///		0 will be perfectly inelastic. Values outside of this range are
	///		allowed and will result in unrealistic behavior.
	/// * `name` - The field name. Defaults to "Collider" if `None`. Only
	///		particles with colliders having the same `name` will collide.
	pub fn new(
		radius: f64,
		coefficient_of_restitution: f64,
		name: Option<String>)
		-> Collider
	{
		let field_name = match name {
			Some(s) => s,
			None => String::from("Collider"),
		};

		Collider {
			radius: radius,
			coefficient_of_restitution: coefficient_of_restitution,
			name: field_name,
		}
	}
}

impl Field for Collider {
	// NOTE: It may be necessary to add some (user configurable) threshold
	//	value that a velocity must be greater than to allow a collision to
	//	occur. Otherwise, floating point errors will probably make
	//	collisions between particles that should be stationary happen
	//	constantly. If this "vibration" is small enough, it may not matter,
	//	but it's also possible that it could cause random motion or somehow
	//	increase in magnitude if not damped out or ignored in some way.
	fn effect(
		&self,
		simulation: &simulation::Simulation,
		position: physical_quantities::Displacement,
		triggered_by: HashMap<Uuid, Vec<Option<FieldInfo>>>,
		field_owner_id: Uuid,
	) {

		// TODO: Implement collisions.

		// For each triggering field with the same name as this field, find the
		//	secant defined by the overlapping circles. If it is not possible to
		//	find the secant because the circles have identical radius and share
		//	a center, do nothing and skip all remaining logic. If it is not
		//	possible to find the secant because one of the two circles is
		//	completely within the other, calculate where the two circles would
		//	have first overlapped based on the particles' relative velocities.

		// Find the component of this particle's velocity and the component of
		//	the triggering particle's velocity normal to the secant. Be careful
		//	to be consistent with which direction positive and negative velocity
		//	represent. Is there a convention I should follow (look it up)?

		// If the relative velocities of the two particles along the normal are
		//	already (approximately?) zero or away from each other, skip all
		//	remaining logic. No forces should be applied.

		// Using the components of velocity along the normal, calculate the
		//	other particle's new velocity along the normal. If we call this
		//	particle v_a and the other particle v_b, the new velocity along the
		//	normal will be calculated as follows:
		//		v_b = (C_R * m_a * (u_a - u_b) + m_b * u_b + m_a * u_b) / (m_b + m_a)
		//	Where
		//		v_a is the final velocity of the first object after impact
		//		v_b is the final velocity of the second object after impact
		//		u_a is the initial velocity of the first object before impact
		//		u_b is the initial velocity of the second object before impact
		//		m_a is the mass of the first object
		//		m_b is the mass of the second object
		//		C_R is the coefficient of restitution

		// Calculate a force that will set the other particle's velocity along
		//	the normal to the new velocity in a single tick. Apply that force to
		//	the other particle. Do nothing to this particle, assuming that the
		//	field attached to the other particle will handle that.
	}

	fn get_radius(&self) -> f64 {
		self.radius
	}

	fn affects_self(&self) -> bool {
		false
	}

	fn affects_others(&self) -> bool {
		true
	}

	fn triggers_on_fields(&self) -> bool {
		true
	}

	fn triggers_on_particles(&self) -> bool {
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
	triggers_on_fields: bool,
	triggers_on_particles: bool,
	name: String,
}

impl FieldInfo {
	pub fn new(
		radius: f64,
		affects_self: bool,
		affects_others: bool,
		triggers_on_fields: bool,
		triggers_on_particles: bool,
		name: String,
	) -> Self {
		Self {
			radius: radius,
			affects_self: affects_self,
			affects_others: affects_others,
			triggers_on_fields: triggers_on_fields,
			triggers_on_particles: triggers_on_particles,
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

	pub fn get_triggers_on_fields(&self) -> bool {
		self.triggers_on_fields
	}

	pub fn get_triggers_on_particles(&self) -> bool {
		self.triggers_on_particles
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
				field.triggers_on_fields(),
				field.triggers_on_particles(),
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
	//	velocity. Also calculate the new position, based on acceleration and
	//	starting velocity and set position.
	pub fn accelerate(
		&mut self,
		forces: &Vec<physical_quantities::Force>,
		time: physical_quantities::Time
	) {
		let v_0 = self.velocity;
		let mut x = 0.0;
		let mut y = 0.0;

		for force in forces {
			x += force.x();
			y += force.y();
		}

		let total_force = physical_quantities::Force::new(x, y);
		let acceleration = total_force / self.mass;

		self.velocity = v_0 + acceleration * time;
		self.position =
			self.position + v_0 * time + 0.5 * acceleration * time * time;
	}

	// Given an amount of time, set the particle's new position based on its
	//	velocity.
	pub fn coast(&mut self, time: physical_quantities::Time) {
		self.position += self.velocity * time;
	}
}
