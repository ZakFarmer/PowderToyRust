pub use crate::particle_sim::particle::Particle;
use rand::Rng;

pub struct World {
    particles: Vec<Particle>,
}

impl World {
    pub fn new() -> Self {
        let mut new_particles: Vec<Particle> = Vec::new();
        let mut rng = rand::thread_rng();

        for n in 1..100 {
            new_particles.push(Particle::new((rng.gen_range(4..crate::WIDTH - 4)) as f32, (rng.gen_range(4..crate::HEIGHT - 4)) as f32, rng.gen_range(-10..10) as f32, rng.gen_range(-10..10) as f32, crate::COLORS[rng.gen_range(0..7)]));
        }

        Self {
            particles: new_particles.to_vec(),
        }
    }

    pub fn draw(&self, frame: &mut [u8]) {

        // Draw the particles
        for particle in &self.particles {
            let x = particle.x as u32;
            let y = particle.y as u32;
            //let r = particle.radius as u32;
            let color = particle.color;
            
            for i in x ..=x {
                for j in y ..=y {
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
        for particle in &mut self.particles {
            particle.update();
        }
    }
}