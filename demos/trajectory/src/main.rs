use macroquad::prelude::*;
use v0::physical_quantities::*;
use v0::simulation::Simulation;
use v0::simulation_objects::SimpleSelfGravityField;

#[macroquad::main("Physics Demo")]
async fn main() {
	let sim = Simulation::new(
		Time::new(0.01),
		Some(1.0),
		None,
	);
	let particle_id = sim.create_particle(
		Mass::new(1.0),
		Displacement::new(100.0, -400.0),
		vec! [Box::new(SimpleSelfGravityField::new(
			Acceleration::new(0.0, -9.81),
			None,
		))],
	);
	// Step once to get the simulation to actually add the new particle.
	sim.step();

	let mut position;
	let mut elapsed_sim_time;
	let mut segment_points = Vec::new();
	// Add a tracer segment endpoint every time this many simulated seconds pass.
	let segment_time = Time::new(0.25);
	let mut last_seg_time = Time::new(0.0);
	loop {
		elapsed_sim_time = sim.get_elapsed_time();

		// Apply a force for a few seconds.
		if elapsed_sim_time.get_number() <= 0.5 {
			sim.apply_force(particle_id, Force::new(100.0, 100.0));
		}
		sim.step_synchronized();

		clear_background(BLACK);

		position = sim.get_position(particle_id);

		if elapsed_sim_time - last_seg_time >= segment_time {
			segment_points.push(position);
			last_seg_time = elapsed_sim_time;
		}

		// Draw the particle.
		draw_circle(position.x() as f32, -position.y() as f32, 7.0, WHITE);

		// Draw a tracer.
		for i in 0..segment_points.len() {
			// Each time we encounter a point with an odd index, we can add a
			//	new line segment.
			if i % 2 != 0 {
				draw_line(
					segment_points[i - 1].x() as f32,
					-segment_points[i - 1].y() as f32,
					segment_points[i].x() as f32,
					-segment_points[i].y() as f32,
					3.0,
					GREEN,
				);
			}
		}

		next_frame().await
	}
}
