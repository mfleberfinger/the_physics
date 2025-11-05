use macroquad::prelude::*;
use v0::physical_quantities::*;
use v0::simulation::Simulation;
use v0::simulation_objects::*;
use std::time::{Instant, Duration};

#[macroquad::main("Physics Demo")]
async fn main() {
	let sim_speed = 1.0;
	let sim = Simulation::new(
		Time::new(0.001),
		Some(sim_speed),
		None,
	);
	let p_id1 = sim.create_particle(
		Mass::new(3.5e15),
		Displacement::new(400.0, -300.0),
		vec! [Box::new(UniversalGravitationField::new(
			10000.0,
			None,
			None,
		))],
	);
	let p_id2 = sim.create_particle(
		Mass::new(5000.0),
		Displacement::new(400.0, -400.0),
		vec! [Box::new(UniversalGravitationField::new(
			10000.0,
			None,
			None,
		))],
	);
	let p_id3 = sim.create_particle(
		Mass::new(5000.0),
		Displacement::new(400.0, -450.0),
		vec! [Box::new(UniversalGravitationField::new(
			10000.0,
			None,
			None,
		))],
	);
	let p_id4 = sim.create_particle(
		Mass::new(5000.0),
		Displacement::new(400.0, -500.0),
		vec! [Box::new(UniversalGravitationField::new(
			10000.0,
			None,
			None,
		))],
	);

	// Step once to get the simulation to actually add the new particles.
	sim.step();

	let mut position1;
	let mut position2;
	let mut position3;
	let mut position4;
	let mut elapsed_sim_time;
	let mut segment_points2 = Vec::new();
	let mut segment_points3 = Vec::new();
	let mut segment_points4 = Vec::new();
	// Add a tracer segment endpoint every time this many simulated seconds pass.
	let segment_time = Time::new(0.25);
	let mut last_seg_time = Time::new(0.0);
	let mut timer = Instant::now();
	let mut prev_frame_time = 0.0;
	let mut last_tick;
	let mut last_force_tick = Ticks::new(0);
	//for i in 0..1000 {
	loop {
		elapsed_sim_time = sim.get_elapsed_time();
		last_tick = sim.get_elapsed_ticks();

		// Apply a force for a few seconds. Need to avoid doing this more than
		//	once per tick, to avoid non-determinism.
		if elapsed_sim_time.get_number() <= 0.5
			&& last_tick.get_number() > last_force_tick.get_number() {
			last_force_tick = last_tick;
			sim.apply_force(p_id2, Force::new(4.0e5, 0.0));
			sim.apply_force(p_id3, Force::new(4.0e5, 0.0));
			sim.apply_force(p_id4, Force::new(3.0e5, 0.0));
		}
		sim.step_synchronized();

		clear_background(BLACK);

		position1 = sim.get_position(p_id1);
		position2 = sim.get_position(p_id2);
		position3 = sim.get_position(p_id3);
		position4 = sim.get_position(p_id4);


		if elapsed_sim_time - last_seg_time >= segment_time {
			segment_points2.push(position2);
			segment_points3.push(position3);
			segment_points4.push(position4);
			last_seg_time = elapsed_sim_time;
		}

		// Draw the particles.
		draw_circle(position1.x() as f32, -position1.y() as f32, 10.0, BLUE);
		draw_circle(position2.x() as f32, -position2.y() as f32, 5.0, RED);
		draw_circle(position3.x() as f32, -position3.y() as f32, 5.0, VIOLET);
		draw_circle(position4.x() as f32, -position4.y() as f32, 5.0, LIGHTGRAY);

		// Draw a tracer.
		for i in 0..segment_points2.len() {
			// Each time we encounter a point with an odd index, we can add a
			//	new line segment.
			if i % 2 != 0 {
				draw_line(
					segment_points2[i - 1].x() as f32,
					-segment_points2[i - 1].y() as f32,
					segment_points2[i].x() as f32,
					-segment_points2[i].y() as f32,
					3.0,
					MAROON,
				);
			}
		}
		// Draw a tracer.
		for i in 0..segment_points3.len() {
			// Each time we encounter a point with an odd index, we can add a
			//	new line segment.
			if i % 2 != 0 {
				draw_line(
					segment_points3[i - 1].x() as f32,
					-segment_points3[i - 1].y() as f32,
					segment_points3[i].x() as f32,
					-segment_points3[i].y() as f32,
					3.0,
					PURPLE,
				);
			}
		}
		// Draw a tracer.
		for i in 0..segment_points4.len() {
			// Each time we encounter a point with an odd index, we can add a
			//	new line segment.
			if i % 2 != 0 {
				draw_line(
					segment_points4[i - 1].x() as f32,
					-segment_points4[i - 1].y() as f32,
					segment_points4[i].x() as f32,
					-segment_points4[i].y() as f32,
					3.0,
					GRAY,
				);
			}
		}

		
		// Don't await the next animation frame until at least 1/60 of a second
		//	(more or less to respect the simulation speed setting). has elapsed
		//	in the simulation. Otherwise, we're limiting the number of times we
		//	can call step_synchronized based on framerate, which seems to max
		//	out at around 60fps.
		if elapsed_sim_time.get_number() >= prev_frame_time + (0.01666667 * sim_speed) {
			prev_frame_time = elapsed_sim_time.get_number();
			next_frame().await
		}
	}
	print!("1000 loops took {0} seconds.", timer.elapsed().as_secs());
}
