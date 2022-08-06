pub use crate::particle_sim::particle::{Particle, ParticleVariant};
use rand::{rngs::ThreadRng, Rng};
use rapier2d::prelude::*;

use super::{
    geometry::Point,
    graphics::{blit, Sprite},
    loader::load_assets,
};

pub struct World {
    y: i32,
    particle_sprite: Sprite,
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
    pub fn add_particle(&mut self, x: f32, y: f32, variant: ParticleVariant) -> bool {
        let mut rng: ThreadRng = rand::thread_rng();

        self.particles.push(Particle::new(
            x,
            y,
            variant,
            crate::COLORS[rng.gen_range(0..7)],
            &mut self.collider_set,
            &mut self.rigid_body_set,
        ));

        true
    }

    pub fn clear_particles(&mut self) -> bool {
        self.particles.clear();

        true
    }

    pub fn new() -> Self {
        let assets = load_assets();

        let mut new_particles: Vec<Particle> = Vec::new();
        let mut rng = rand::thread_rng();

        let mut broad_phase: BroadPhase = BroadPhase::new();
        let mut ccd_solver: CCDSolver = CCDSolver::new();
        let mut collider_set: ColliderSet = ColliderSet::new();
        let mut impulse_joint_set: ImpulseJointSet = ImpulseJointSet::new();
        let mut island_manager: IslandManager = IslandManager::new();
        let mut multibody_joint_set: MultibodyJointSet = MultibodyJointSet::new();
        let mut narrow_phase: NarrowPhase = NarrowPhase::new();
        let mut physics_pipeline: PhysicsPipeline = PhysicsPipeline::new();
        let mut rigid_body_set: RigidBodySet = RigidBodySet::new();

        let ground_collider = ColliderBuilder::cuboid(crate::WIDTH as f32, 0.1)
            .translation(vector![0.0, crate::HEIGHT as f32])
            .build();

        let left_wall_collider: Collider = ColliderBuilder::cuboid(0.1, crate::HEIGHT as f32)
            .translation(vector![0.0, 0.0])
            .build();

        let right_wall_collider: Collider = ColliderBuilder::cuboid(0.1, crate::HEIGHT as f32)
            .translation(vector![crate::WIDTH as f32, 0.0])
            .build();

        collider_set.insert(ground_collider);
        collider_set.insert(left_wall_collider);
        collider_set.insert(right_wall_collider);

        let particle_sprite = Sprite::new(&assets, crate::particle_sim::graphics::Frame::Particle);

        let mut integration_parameters = IntegrationParameters::default();

        integration_parameters.dt = 1.0 / 45.0;

        /*let (collision_send, collision_recv) = crossbeam::channel::unbounded();
        let (contact_force_send, contact_force_recv) = crossbeam::channel::unbounded();
        let event_handler = ChannelEventCollector::new(collision_send, contact_force_send);*/

        for _n in 1..20 {
            new_particles.push(Particle::new(
                (rng.gen_range(4..crate::WIDTH - 4)) as f32,
                (rng.gen_range(4..crate::HEIGHT - 4)) as f32,
                ParticleVariant::URAN,
                crate::COLORS[rng.gen_range(0..7)],
                &mut collider_set,
                &mut rigid_body_set,
            ));
        }
        Self {
            y: 0,
            particle_sprite,
            particles: new_particles.to_vec(),
            rigid_body_set,
            collider_set,
            integration_parameters,
            physics_pipeline,
            island_manager,
            broad_phase,
            narrow_phase,
            impulse_joint_set,
            multibody_joint_set,
            ccd_solver,
            physics_hooks: (),
            event_handler: (),
        }
    }

    pub fn draw(&self, frame: &mut [u8]) {
        // Draw the particles
        for particle in &self.particles {
            let x = particle.x as usize;
            let y = particle.y as usize;
            //let r = particle.radius as u32;

            let color = particle.color;
            let pos = Point::new(x, y);

            blit(frame, &pos, &self.particle_sprite, color);

            /*for i in x..=x {
                for j in y..=y {
                    if i < crate::WIDTH as u32 && j < crate::HEIGHT as u32 {
                        let offset = (j * crate::WIDTH as u32 + i) as usize * 4;

                        frame[offset] = (color[0]) as u8;
                        frame[offset + 1] = (color[1]) as u8;
                        frame[offset + 2] = (color[2]) as u8;
                        frame[offset + 3] = (color[3]) as u8;
                    }
                }
            }*/
        }
    }

    pub fn update(&mut self) {
        if self.y == 200 {
            self.y = 0;
        } else {
            self.y = self.y + 1;
        }

        /*if (self.y % 2 == 0) {
            self.add_particle(
                self.y as f32,
                (self.y as f32 / 4.0) + (self.y % 3) as f32,
                ParticleVariant::URAN,
            );
        }*/

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
