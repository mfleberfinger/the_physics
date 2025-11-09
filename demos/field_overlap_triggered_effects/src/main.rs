use macroquad::prelude::*;
use std::collections::HashMap;
use uuid::Uuid;
use v0::physical_quantities::*;
use v0::simulation::Simulation;
use v0::simulation_objects::*;

#[macroquad::main("Physics Demo")]
async fn main() {
	let sim_speed = 1.0;
	let sim = Simulation::new(
		Time::new(0.0166666),
		Some(sim_speed),
		None,
	);

	let mut particles = Vec::new();

	for i in 1..6 {
		for j in 1..2 {
			let p_id = sim.create_particle(
				Mass::new(3.5),
				Displacement::new(
					500.0 + ((i * 15) as f64),
					-300.0 - (((j * 15) + (i * 0)) as f64)
				),
				vec! [
					Box::new(SimpleSelfGravityField::new(
						Acceleration::new(0.0, -9.81),
						None,
					)),
					Box::new(StickyField::new(
						15.0,
						"Faller".to_string(),
					)),
				],
			);

			particles.push(p_id);
		}
	}

	// Make a floor.
	for i in 1..21 {
		let p_id = sim.create_particle(
			Mass::new(3.5),
			Displacement::new(300.0 + ((i * 10) as f64), -700.0),
			vec! [
				Box::new(FloorAndWallField::new(
					Force::new(500.0, 500.0),
					10.0,
					"Floor".to_string(),
				)),
			],
		);

		particles.push(p_id);
	}

	// Step once to get the simulation to actually add the new particles.
	sim.step();

	// Set the window size.
	request_new_screen_size(1500.0, 1000.0);
	//set_fullscreen(true);
	let window_width = screen_width();
	let window_height = screen_height();

	let mut elapsed_sim_time;
	let mut position;
	let mut mass;
	let mut force;
	let mut prev_frame_time = 0.0;
	let return_multiplier = 0.0;//10.0;
	loop {
		elapsed_sim_time = sim.get_elapsed_time();

		// Apply a force for a while.
		if elapsed_sim_time.get_number() <= 0.5 {
			//sim.apply_force(particles[0], Force::new(1.0e12, -1.0e12));
		}

		sim.step_synchronized();

		clear_background(BLACK);

		for p_id in &particles {
			position = sim.get_position(*p_id);
			mass = sim.get_mass(*p_id);

			// If a particle reaches the edge of the window, try to knock it
			//	back towards the center.
			if position.x() < 0.0 {
				force = Force::new(mass.get_number() * return_multiplier, 0.0);
				sim.apply_force(*p_id, force);
			}
			if (position.y() as f32) < -window_height {
				force = Force::new(0.0, mass.get_number() * return_multiplier);
				sim.apply_force(*p_id, force);
			}
			if (position.x() as f32) > window_width {
				force = Force::new(-mass.get_number() * return_multiplier, 0.0);
				sim.apply_force(*p_id, force);
			}
			if position.y() > 0.0 {
				force = Force::new(0.0, -mass.get_number() * return_multiplier);
				sim.apply_force(*p_id, force);
			}

			// Draw the particles.
			let mut radius = 5.0;
			let mut color = PINK;
			// Set radius based on "collider" radius. Also set color.
			for info in sim.get_field_info(*p_id) {
				let name = info.get_name();
				if name == "Floor" || name == "Faller" {
					radius = info.get_radius() as f32;
					if name == "Floor" {
						color = GRAY;
					}
					else {
						color = GREEN;
					}
				}
			}
			draw_circle(position.x() as f32, -position.y() as f32, radius, color);
		}


		// Don't await the next animation frame until at least 1/60 of a second
		//	(more or less to respect the simulation speed setting) has elapsed
		//	in the simulation. Otherwise, we're limiting the number of times we
		//	can call step_synchronized based on framerate, which seems to max
		//	out at around 60fps.
		if elapsed_sim_time.get_number() >= prev_frame_time + (0.01666667 * sim_speed) {
			prev_frame_time = elapsed_sim_time.get_number();
			next_frame().await
		}
	}
}

// A janky pseudo-collider.
struct FloorAndWallField {
	force: Force,
	radius: f64,
	name: String,
}

impl FloorAndWallField {
	fn new(force: Force, radius: f64, name: String) -> Self {
		Self {
			force: force,
			radius: radius,
			name: name,
		}
	}
}

impl Field for FloorAndWallField {
	fn effect(
		&self,
		simulation: &Simulation,
		_position: Displacement,
		triggered_by: HashMap<Uuid, Vec<Option<FieldInfo>>>,
		_field_owner_id: Uuid,
	) {
		for id in triggered_by.keys() {
			// Only apply a force if the detected particle isn't part of the
			//	wall.
			let info = simulation.get_field_info(*id);
			let mut is_wall = false;
			for i in info {
				if i.get_name() == self.get_name() {
					is_wall = true;
				}
			}
			if !is_wall {
				simulation.apply_force(*id, self.force);
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
		true
	}

	fn triggers_on_particles(&self) -> bool {
		false
	}

	fn get_name(&self) -> &String {
		&self.name
	}
}

// Just here to provide something to set rendered circle size and trigger the
//	floor's effect.
struct MarkerField {
	radius: f64,
	name: String,
}

impl MarkerField {
	fn new(radius: f64, name: String) -> Self {
		Self {
			radius: radius,
			name: name,
		}
	}
}

impl Field for MarkerField {
	fn effect(
		&self,
		simulation: &Simulation,
		_position: Displacement,
		triggered_by: HashMap<Uuid, Vec<Option<FieldInfo>>>,
		_field_owner_id: Uuid,
	) {
		// No-op
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

struct StickyField {
	radius: f64,
	name: String,
}

impl StickyField {
	fn new(radius: f64, name: String) -> Self {
		Self {
			radius: radius,
			name: name,
		}
	}
}

impl Field for StickyField {
	fn effect(
		&self,
		simulation: &Simulation,
		position: Displacement,
		triggered_by: HashMap<Uuid, Vec<Option<FieldInfo>>>,
		field_owner_id: Uuid,
	) {
		// Attempt to get particles with this field to stick to each other.
		// Don't bother trying to calculate or reason too much about how to
		//	make this work properly. Just do something naive and see what
		//	happens. It should be entertaining.
		for (particle_id, field_info_vec) in triggered_by.iter() {
			for field_info in field_info_vec {
				if let Some(info) = field_info {
					if info.get_name() == self.get_name() {
						// Stick.
						let my_velocity = simulation.get_velocity(field_owner_id);
						let other_velocity =
							simulation.get_velocity(*particle_id);
						// Naively assume that each particle can just apply half of
						//	the required velocity change to set their relative
						//	velocity to 0. What happens if three particles act on
						//	each other? Doesn't matter.
						let double_velocity =
							 Velocity::new(
								(my_velocity.x() - other_velocity.x()),
								(my_velocity.y() - other_velocity.y()),
							);

						let half_magnitude =
							0.5 * (
								double_velocity.x().powf(2.0) 
								+ double_velocity.y().powf(2.0)
							).sqrt();

						let other_delta_velocity =
							Velocity::new(
								(double_velocity.x() * 0.5),
								(double_velocity.y() * 0.5),
							);
						// a = m/s/s => a = delta_v / t
						let one_over_time =
							(1.0 / simulation.get_tick_duration().get_number());
						let other_acceleration =
							Acceleration::new(
								other_delta_velocity.x() * one_over_time,
								other_delta_velocity.y() * one_over_time);
						// f = ma
						let force =
							 simulation.get_mass(*particle_id) * other_acceleration;
						simulation.apply_force(*particle_id, force);
					}
				}
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
		true
	}

	fn triggers_on_particles(&self) -> bool {
		false
	}

	fn get_name(&self) -> &String {
		&self.name
	}
}
