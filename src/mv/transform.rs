use rapier2d::{
    math::Vector,
    prelude::{
        CCDSolver, Collider, ColliderBuilder, ColliderHandle, ColliderSet, DefaultBroadPhase,
        ImpulseJointSet, IntegrationParameters, IslandManager, MultibodyJointSet, NarrowPhase,
        PhysicsPipeline, RigidBody, RigidBodyBuilder, RigidBodyHandle, RigidBodySet,
    },
};
use vulkano::buffer::BufferContents;

const GRAVITY: Vector = Vector::new(0.0, -9.81);

/// Структура не относится к объекту, относится только к базовой среде,
/// где ведется разработка самого приложения(в структуре App)
pub struct PhysicsContext {
    rigid_body_set: RigidBodySet,
    collider_set: ColliderSet,
    pub space: PhysicSpace,
}

pub struct PhysicSpace {
    integration_parameters: IntegrationParameters,
    pipeline: PhysicsPipeline,
    island_manager: IslandManager,
    broad_phase: DefaultBroadPhase,
    narrow_phase: NarrowPhase,
    impulse_joint_set: ImpulseJointSet,
    multibody_joint_set: MultibodyJointSet,
    ccd_solver: CCDSolver,
    physics_hooks: (),
    event_handler: (),
}

// TODO: Дальше надо создать структуру PhysicsDrawable, которая является оберткой над Drawable(имеет Drawable как переменную)
// Далее переносим все RigidBody и другие физ. свойства в PhysicsDrawable и делаем реализацию DynamicObject только для нее.
/// Базовая структура для объектов, может иметь RigidBody, Collider и их обработчики, но их значение опционально.
pub struct Drawable {}

pub struct PhysicsDrawable {
    rb: Option<RigidBody>,
    collider: Option<Collider>,
    rb_h: Option<RigidBodyHandle>,
    collider_h: Option<ColliderHandle>,
    drawable: Drawable,
}

/// Реализация этого трейта доступна для всех объектов, но использовать ее могут только если RigidBody или Collider имеются.
trait DynamicObject {
    fn change_position(&self, pos: Vector) -> &Self;
    fn set_translation(&self, vec: Vector) -> &Self;
}

/// Бесполезный трейт. По идее должен создавать объекты на экран, но это можно организовать в чем-либо другом.
trait Objects {
    fn create_phys_object(&mut self, position: Option<Vector>) -> PhysicsDrawable;
}

impl Drawable {
    pub fn new() -> Self {
        Drawable {}
    }
}

impl PhysicsDrawable {
    pub fn new(
        rb: Option<RigidBody>,
        col: Option<Collider>,
        rb_h: Option<RigidBodyHandle>,
        collider_h: Option<ColliderHandle>,
    ) -> Self {
        PhysicsDrawable {
            drawable: Drawable::new(),
            rb,
            collider: col,
            rb_h,
            collider_h,
        }
    }
}

impl DynamicObject for PhysicsDrawable {
    fn change_position(&self, pos: Vector) -> &Self {
        todo!()
    }
    fn set_translation(&self, vec: Vector) -> &Self {
        self.rb.clone().unwrap().set_translation(vec, false);
        self
    }
}

impl PhysicsContext {
    pub fn new(
        rigid_body_set: RigidBodySet,
        collider_set: ColliderSet,
        space: PhysicSpace,
    ) -> Self {
        PhysicsContext {
            rigid_body_set,
            collider_set,
            space,
        }
    }

    pub fn step(&mut self) {
        self.space.pipeline.step(
            GRAVITY,
            &self.space.integration_parameters,
            &mut self.space.island_manager,
            &mut self.space.broad_phase,
            &mut self.space.narrow_phase,
            &mut self.rigid_body_set,
            &mut self.collider_set,
            &mut self.space.impulse_joint_set,
            &mut self.space.multibody_joint_set,
            &mut self.space.ccd_solver,
            &self.space.physics_hooks,
            &self.space.event_handler,
        );
    }
}

impl Objects for PhysicsContext {
    fn create_phys_object(&mut self, position: Option<Vector>) -> PhysicsDrawable {
        let mut rigid_body_builder = RigidBodyBuilder::dynamic();
        if let Some(pos) = position {
            rigid_body_builder = rigid_body_builder.translation(pos);
        }
        let rigid_body = rigid_body_builder.build();
        let collider = ColliderBuilder::ball(0.5).restitution(0.7).build();
        let rb_h = self.rigid_body_set.insert(rigid_body.clone());
        self.collider_set.insert_with_parent(
            collider.clone(),
            rb_h.clone(),
            &mut self.rigid_body_set,
        );
        PhysicsDrawable::new(Some(rigid_body), Some(collider), Some(rb_h), None)
    }
}

impl PhysicSpace {
    pub fn new() -> Self {
        PhysicSpace {
            integration_parameters: IntegrationParameters::default(),
            pipeline: PhysicsPipeline::new(),
            island_manager: IslandManager::new(),
            broad_phase: DefaultBroadPhase::new(),
            narrow_phase: NarrowPhase::new(),
            impulse_joint_set: ImpulseJointSet::new(),
            multibody_joint_set: MultibodyJointSet::new(),
            ccd_solver: CCDSolver::new(),
            physics_hooks: (),
            event_handler: (),
        }
    }
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
