// engine.rs
use rayon::prelude::*;

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct Vec3 {
	pub x: f32,
	pub y: f32,
	pub z: f32,
}

impl std::ops::Add for Vec3 {
    type Output = Self;
    fn add(self, other: Self) -> Self {
        Vec3 { x: self.x + other.x, y: self.y + other.y, z: self.z + other.z }
    }
}

impl std::ops::Sub for Vec3 {
    type Output = Self;
    fn sub(self, other: Self) -> Self {
        Vec3 { x: self.x - other.x, y: self.y - other.y, z: self.z - other.z }
    }
}

impl std::ops::AddAssign for Vec3 {
    fn add_assign(&mut self, other: Self) {
        self.x += other.x; self.y += other.y; self.z += other.z;
    }
}

impl std::ops::Mul<f32> for Vec3 {
    type Output = Self;
    fn mul(self, scalar: f32) -> Self {
        Vec3 { x: self.x * scalar, y: self.y * scalar, z: self.z * scalar }
    }
}

impl std::ops::Div<f32> for Vec3 {
    type Output = Self;
    fn div(self, scalar: f32) -> Self {
        Vec3 { x: self.x / scalar, y: self.y / scalar, z: self.z / scalar }
    }
}

impl Vec3 {
	pub fn zero() -> Self {
		Vec3 { x: 0.0, y: 0.0, z: 0.0 }
	}
	pub fn squared_dist(self, other: Self) -> f32 {
		let dx = self.x - other.x;
		let dy = self.y - other.y;
		let dz = self.z - other.z;

		(dx * dx) + (dy * dy) + (dz * dz)
	}
}

#[derive(Debug)]
pub struct SimulationState {
	pub positions: Vec<Vec3>,
	pub velocities: Vec<Vec3>,
	pub forces: Vec<Vec3>,
}

#[repr(C)]
#[derive(Debug)]
pub struct ChemicalProperties {
	pub mass: f32,
	pub diameter_sigma: f32,
	pub depth_epsilon: f32
}

#[derive(Debug)]
pub struct DoubleBufferedEngine {
	pub total_atoms: usize,
	pub primary_buffer: SimulationState,
	pub secondary_buffer: SimulationState,
	pub atom_registry: Vec<ChemicalProperties>,
}

impl DoubleBufferedEngine {
	pub fn new(capacity: usize) -> Self {
		let mut registry = Vec::with_capacity(capacity);

		for _ in 0..capacity {
			registry.push(ChemicalProperties { mass: 1.0, diameter_sigma: 1.0, depth_epsilon: 1.0 });
		}

		DoubleBufferedEngine {
			total_atoms: capacity,

			primary_buffer: SimulationState {
				positions: vec![Vec3::zero(); capacity],
				velocities: vec![Vec3::zero(); capacity],
				forces: vec![Vec3::zero(); capacity],
			},

			secondary_buffer: SimulationState {
				positions: vec![Vec3::zero(); capacity],
				velocities: vec![Vec3::zero(); capacity],
				forces: vec![Vec3::zero(); capacity],
			},

			atom_registry: registry,
		}
	}

	pub fn execute_runtime_step(&mut self, delta_time: f32) {
		let dt_half = delta_time * 0.5;
		let cutoff_sq = 100.0;

		self.secondary_buffer.positions.par_iter_mut()
			.zip(&mut self.secondary_buffer.velocities)
			.zip(&self.primary_buffer.positions)
            .zip(&self.primary_buffer.velocities)
            .zip(&self.primary_buffer.forces)
            .zip(&self.atom_registry)
			.for_each(|(((((next_pos, next_vel), curr_pos), curr_vel), curr_force), chem)| {
				let acc = *curr_force / chem.mass;

				*next_pos = *curr_pos + (*curr_vel * delta_time) + (acc * dt_half * delta_time);

				*next_vel = *curr_vel + (acc * dt_half);
			});

		self.secondary_buffer.forces.fill(Vec3::zero());

		self.secondary_buffer.forces.par_iter_mut().enumerate().for_each(|(i, total_force_vector)| {
			let pos_i = self.secondary_buffer.positions[i];
			let chem_i = &self.atom_registry[i];
			let mut accumulator = Vec3::zero();

			for j in 0..self.total_atoms {
				if i == j { continue; }

				let pos_j = self.secondary_buffer.positions[j];
				let r2 = pos_i.squared_dist(pos_j);

				if r2 < cutoff_sq {
					let displacement = pos_j - pos_i;
					let inv_r2 = 1.0 / r2;

					let mixed_sigma = (chem_i.diameter_sigma + self.atom_registry[j].diameter_sigma) * 0.5;
					let mixed_epsilon = (chem_i.depth_epsilon * self.atom_registry[j].depth_epsilon).sqrt();

					let sig6 = mixed_sigma.powi(6);
					let term6 = sig6 * (inv_r2 * inv_r2 * inv_r2);
					let term12 = term6 * term6;

					let scalar = 24.0 * mixed_epsilon * (2.0 * term12 - term6) * inv_r2;

					accumulator += displacement * scalar;
				}
			}

			*total_force_vector = accumulator;
		});

		self.secondary_buffer.velocities.par_iter_mut()
			.zip(&self.secondary_buffer.forces)
			.zip(&self.atom_registry)
			.for_each(|((next_vel, next_force), chem)| {
				let new_acc = *next_force / chem.mass;

				*next_vel += new_acc * dt_half;
			});

		std::mem::swap(&mut self.primary_buffer, &mut self.secondary_buffer);
	}
}
