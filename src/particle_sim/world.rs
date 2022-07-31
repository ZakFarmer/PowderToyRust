pub use crate::particle_sim::particle::{Particle, ParticleVariant};
use rand::Rng;
use rapier2d::{na::matrix, prelude::*};

pub struct World {
    particles: Vec<Particle>,
    pub rigid_body_set: RigidBodySet,
    collider_set: ColliderSet,
    integration_parameters: IntegrationParameters,
    physics_pipeline: PhysicsPipeline,
    island_manager: IslandManager,
    broad_phase: BroadPhase,
    narrow_phase: NarrowPhase,
    impulse_joint_set: ImpulseJointSet,
    multibody_joint_set: MultibodyJointSet,
    ccd_solver: CCDSolver,
    physics_hooks: (),
    event_handler: (),
}

impl World {
    pub fn new() -> Self {
        let mut new_particles: Vec<Particle> = Vec::new();
        let mut rng = rand::thread_rng();

        let mut collider_set: ColliderSet = ColliderSet::new();
        let mut rigid_body_set: RigidBodySet = RigidBodySet::new();

        let ground_collider = ColliderBuilder::cuboid(crate::WIDTH as f32, 0.1)
            .translation(vector![0.0, crate::HEIGHT as f32])
            .build();
        collider_set.insert(ground_collider);

        for n in 1..20 {
            new_particles.push(Particle::new(
                (rng.gen_range(4..crate::WIDTH - 4)) as f32,
                (rng.gen_range(4..crate::HEIGHT - 4)) as f32,
                rng.gen_range(-10..10) as f32,
                rng.gen_range(-10..10) as f32,
                ParticleVariant::WOOD,
                crate::COLORS[rng.gen_range(0..7)],
                &mut collider_set,
                &mut rigid_body_set,
            ));
        }

        Self {
            particles: new_particles.to_vec(),
            rigid_body_set,
            collider_set,
            integration_parameters: IntegrationParameters::default(),
            physics_pipeline: PhysicsPipeline::new(),
            island_manager: IslandManager::new(),
            broad_phase: BroadPhase::new(),
            narrow_phase: NarrowPhase::new(),
            impulse_joint_set: ImpulseJointSet::new(),
            multibody_joint_set: MultibodyJointSet::new(),
            ccd_solver: CCDSolver::new(),
            physics_hooks: (),
            event_handler: (),
        }
    }

    pub fn draw(&self, frame: &mut [u8]) {
        // Draw the particles
        for particle in &self.particles {
            let x = particle.x as u32;
            let y = particle.y as u32;
            //let r = particle.radius as u32;
            let color = particle.color;

            for i in x..=x {
                for j in y..=y {
                    if i < crate::WIDTH as u32 && j < crate::HEIGHT as u32 {
                        let offset = (j * crate::WIDTH as u32 + i) as usize * 4;

                        frame[offset] = (color[0]) as u8;
                        frame[offset + 1] = (color[1]) as u8;
                        frame[offset + 2] = (color[2]) as u8;
                        frame[offset + 3] = (color[3]) as u8;
                    }
                }
            }
        }
    }

    pub fn update(&mut self) {
        self.physics_pipeline.step(
            &vector![0.0, 9.81],
            &self.integration_parameters,
            &mut self.island_manager,
            &mut self.broad_phase,
            &mut self.narrow_phase,
            &mut self.rigid_body_set,
            &mut self.collider_set,
            &mut self.impulse_joint_set,
            &mut self.multibody_joint_set,
            &mut self.ccd_solver,
            &self.physics_hooks,
            &self.event_handler,
        );

        for particle in &mut self.particles {
            particle.update(&mut self.rigid_body_set);
        }
    }
}
