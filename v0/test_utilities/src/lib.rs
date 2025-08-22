// This library contains shared code for use by tests. This is not part of the
//	physics engine and should only be compiled for tests. Notice the
//	"dev-dependencies" section of the Cargo.toml file in the parent directory.

extern crate v0;
use uuid::Uuid;

pub struct DummyField {
	pub radius: f64,
	pub affects_self: bool,
	pub affects_others: bool,
	pub name: String,
}

impl v0::simulation_objects::Field for DummyField {
	fn effect(
		&self,
		simulation: &v0::simulation::Simulation,
		position: v0::physical_quantities::Displacement,
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


