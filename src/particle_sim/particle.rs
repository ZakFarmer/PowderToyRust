use rapier2d::prelude::*;

#[derive(Clone, Debug)]
pub enum PhysicsType {
    DYNAMIC,
    STATIC,
}

#[derive(Clone, Debug, Copy)]
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
    physics_type: PhysicsType,
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
        variant: ParticleVariant,
        color: [u8; 4],
        collider_set: &mut ColliderSet,
        rigid_body_set: &mut RigidBodySet,
    ) -> Self {
        let physics_type: PhysicsType = match variant {
            ParticleVariant::C4 => PhysicsType::STATIC,
            ParticleVariant::DEUT => PhysicsType::DYNAMIC,
            ParticleVariant::PLUT => PhysicsType::DYNAMIC,
            ParticleVariant::STNE => PhysicsType::STATIC,
            ParticleVariant::URAN => PhysicsType::DYNAMIC,
            ParticleVariant::WOOD => PhysicsType::STATIC,
        };

        let collider: Collider = ColliderBuilder::cuboid(0.5, 0.5)
            //.active_events(ActiveEvents::COLLISION_EVENTS)
            .mass(1.0)
            .restitution(0.0)
            .build();

        let rigid_body: RigidBody = match physics_type {
            PhysicsType::DYNAMIC => RigidBodyBuilder::dynamic()
                .lock_rotations()
                .translation(vector![x as f32, y as f32])
                .build(),
            PhysicsType::STATIC => RigidBodyBuilder::fixed()
                .lock_rotations()
                .translation(vector![x as f32, y as f32])
                .build(),
        };

        let body_handle: RigidBodyHandle = rigid_body_set.insert(rigid_body.clone());
        collider_set.insert_with_parent(collider.clone(), body_handle, rigid_body_set);

        Particle {
            x,
            y,
            physics_type,
            variant,
            color,
            rigid_body,
            collider,
            body_handle,
        }
    }

    pub fn update(&mut self, rigid_body_set: &mut RigidBodySet) {
        let particle_body = &mut rigid_body_set[self.body_handle];

        self.x = particle_body.translation().x;
        self.y = particle_body.translation().y;

        if (particle_body.linvel().x.abs() < 0.01) && (particle_body.linvel().y.abs() == 0.01) {
            /*rigid_body_set.remove(
                self.body_handle,
                self.island_manager,
                self.collider_set,
                self.impulse_joint_set,
                self.multibody_joint_set,
                false,
            );*/
            particle_body.reset_forces(false);
            particle_body.sleep();
        }
    }
}
