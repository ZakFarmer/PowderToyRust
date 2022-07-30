#[derive(Clone, Debug)]
pub enum ParticleVariant {
    WOOD,
    STNE,
    URAN,
    PLUT,
    DEUT,
    C4,
}

#[derive(Clone, Debug)]
pub struct Particle {
    pub x: f32,
    pub y: f32,
    vx: f32,
    vy: f32,
    ax: f32,
    ay: f32,
    radius: f32,
    variant: ParticleVariant,
    pub color: [u8; 4],
}

impl Particle {
    pub fn new(x: f32, y: f32, vx: f32, vy: f32, variant: ParticleVariant, color: [u8; 4]) -> Self {
        Particle {
            x,
            y,
            vx,
            vy,
            ax: 0.0,
            ay: 0.0,
            radius: 4.0,
            variant,
            color,
        }
    }

    pub fn update(&mut self) {
        if self.x < self.radius || self.x > crate::WIDTH as f32 - self.radius {
            self.vx = -self.vx * crate::DAMPING;
        }

        if self.y < self.radius || self.y > crate::HEIGHT as f32 - self.radius {
            self.vy = -self.vy * crate::DAMPING;
        }

        self.x += self.vx * crate::TIMESCALE;
        self.y += self.vy * crate::TIMESCALE;
    }
}
