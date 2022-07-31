use rapier2d::prelude::*;

#[derive(Clone, Debug)]
pub enum ParticleVariant {
    WOOD,
    STNE,
    URAN,
    PLUT,
    DEUT,
    C4,
}

#[derive(Clone)]
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
    pub rigid_body: RigidBody,
    pub collider: Collider,
    pub body_handle: RigidBodyHandle,
}

impl Particle {
    pub fn new(
        x: f32,
        y: f32,
        vx: f32,
        vy: f32,
        variant: ParticleVariant,
        color: [u8; 4],
        collider_set: &mut ColliderSet,
        rigid_body_set: &mut RigidBodySet,
    ) -> Self {
        let collider: Collider = ColliderBuilder::ball(0.75).restitution(0.7).build();
        let rigid_body: RigidBody = RigidBodyBuilder::dynamic()
            .translation(vector![x as f32, y as f32])
            .build();

        let body_handle: RigidBodyHandle = rigid_body_set.insert(rigid_body.clone());
        collider_set.insert_with_parent(collider.clone(), body_handle, rigid_body_set);

        Particle {
            x,
            y,
            vx,
            vy,
            ax: 0.0,
            ay: 0.0,
            radius: 4.0,
            variant,
            rigid_body,
            collider,
            color,
            body_handle,
        }
    }

    pub fn update(&mut self, rigid_body_set: &mut RigidBodySet) {
        let particle_body = &rigid_body_set[self.body_handle];

        self.x = particle_body.translation().x;
        self.y = particle_body.translation().y;

        /*if self.x < self.radius || self.x > crate::WIDTH as f32 - self.radius {
            self.vx = -self.vx * crate::DAMPING;
        }

        if self.y < self.radius || self.y > crate::HEIGHT as f32 - self.radius {
            self.vy = -self.vy * crate::DAMPING;
        }

        self.x += self.vx * crate::TIMESCALE;
        self.y += self.vy * crate::TIMESCALE;*/
    }
}
