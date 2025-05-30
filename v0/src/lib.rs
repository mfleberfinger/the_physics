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

// Using a tuple struct to wrap an i64 so the compiler treats Seconds as a
//	distinct type. This is the "newtype pattern."
/// Represents time, in seconds, within the simulation.
pub struct Seconds(i64);

pub struct Particle {
	// TODO: add fields
}

/// Owns all of the data that constitutes a physics simulation. Provides methods
/// for interacting with and running the simulation.
pub struct Simulation {
	tick_duration: Seconds,
	particles: HashMap<Particle>,
	// TODO: Continue adding fields.
}
