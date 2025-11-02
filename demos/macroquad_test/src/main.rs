/* Simple Macroquad example. Should display three moving dots if uncommented.
use macroquad::prelude::*;
use std::time::{Instant, Duration};

#[macroquad::main("Physics Demo")]
async fn main() {
	let mut array: [(f32, f32); 3] = [(0.0, 0.0); 3];

	array[0] = (200.0, 200.0);
	array[1] = (300.0, 300.0);
	array[2] = (400.0, 200.0);

	let d = Duration::from_secs_f64(0.0333);
	let mut previous = Instant::now();

	loop {
		clear_background(BLACK);

		if previous.elapsed() > d {
			previous = Instant::now();

			for b in &mut array {
				b.0 += 1.0;
				b.1 += 1.0;
			}
		}

		for b in &array {
			draw_circle(b.0, b.1, 10.0, WHITE);
		}

		next_frame().await
	}
}
*/

use macroquad::prelude::*;
use v0::physical_quantities::*;
use v0::simulation::Simulation;

#[macroquad::main("Physics Demo")]
async fn main() {
	let sim = Simulation::new(
		Time::new(0.01),
		Some(1.0),
		None
	);
	let particle_id = sim.create_particle(
		Mass::new(1.0),
		Displacement::new(300.0, 300.0),
		Vec::new(),
	);
	// Step once to get the simulation to actually add the new particle.
	sim.step();
	

	let mut position;
	loop {
		// ------- Physics ------- 

		// Apply a force for a few seconds.
		if sim.get_elapsed_time().get_number() <= 10.0 {
			sim.apply_force(particle_id, Force::new(10.0, 10.0));
		}
		sim.step_synchronized();


		// ------- Animation ------- 

		clear_background(BLACK);

		position = sim.get_position(particle_id);

		draw_circle(position.x() as f32, position.y() as f32, 10.0, WHITE);


		next_frame().await
	}
}
