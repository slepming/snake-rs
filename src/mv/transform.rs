use rapier2d::{
    math::Vector,
    prelude::{
        CCDSolver, Collider, ColliderSet, DefaultBroadPhase, ImpulseJointSet, IslandManager,
        MultibodyJointSet, NarrowPhase, PhysicsPipeline, RigidBody, RigidBodySet,
    },
};
use vulkano::buffer::BufferContents;

pub struct Physics {
    rigid_body: RigidBody,
    collider: Collider,
    collider_set: ColliderSet,
    rigid_body_set: RigidBodySet,
    physics_pipeline: PhysicsPipeline,
    island_manager: IslandManager,
    broad_phase: DefaultBroadPhase,
    narrow_phase: NarrowPhase,
    impulse_joint_set: ImpulseJointSet,
    multibody_joint_set: MultibodyJointSet,
    ccd_solver: CCDSolver,
    gravity: Vector,
}

pub trait Phys {
    fn set_force(&mut self, f: i32);
}

pub trait Position {
    fn get_matrix_mut(&mut self) -> &mut [[f32; 4]; 4];
    fn get_matrix(&self) -> &[[f32; 4]; 4];
}

#[repr(C)]
#[derive(BufferContents, Clone, Copy)]
pub struct Transform {
    transform: [[f32; 4]; 4],
}
