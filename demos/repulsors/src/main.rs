use macroquad::prelude::*;
use v0::physical_quantities::*;
use v0::simulation::Simulation;
use v0::simulation_objects::*;

#[macroquad::main("Physics Demo")]
async fn main() {
	let sim_speed = 1.0;
	let sim = Simulation::new(
		Time::new(0.03333),
		Some(sim_speed),
		None,
	);

	let mut particles = Vec::new();

	for i in 1..11 {
		for j in 1..11 {
			let p_id = sim.create_particle(
				Mass::new(3.5),
				Displacement::new(300.0 + ((j * 50) as f64), -50.0 * (i as f64)),
				vec! [
					Box::new(UniversalGravitationField::new(
						10000.0,
						Some(-0e3),
						None,
					)),
					Box::new(SimpleSelfGravityField::new(
						Acceleration::new(0.0, -9.81),
						None,
					)),
				],
			);

			particles.push(p_id);
		}
	}

	// Make a floor.
	for i in 1..50 {
		let p_id = sim.create_particle(
			Mass::new(3.5e16),
			Displacement::new(300.0 + ((i * 10) as f64), -900.0),
			vec! [
				Box::new(UniversalGravitationField::new(
					10.0,
					Some(-1e-12),
					None,
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
			//let radius = (mass.get_number() * 3.5e-14) as f32;
			let radius = 5.0;
			draw_circle(position.x() as f32, -position.y() as f32, radius, BLUE);
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
