use rapier2d::{
    math::Vector,
    prelude::{
        CCDSolver, ColliderBuilder, ColliderSet, DefaultBroadPhase, ImpulseJointSet,
        IntegrationParameters, IslandManager, MultibodyJointSet, NarrowPhase, PhysicsPipeline,
        RigidBody, RigidBodyBuilder, RigidBodyHandle, RigidBodySet,
    },
};
use vulkano::buffer::BufferContents;

use crate::{
    MyVertex,
    geometry::shapes::{Shapes, get_vertex_from_shapes},
};

const GRAVITY: Vector = Vector::new(0.0, -9.81);

pub struct Children {
    pub physics_drawables: Vec<PhysicsDrawable>,
    pub drawables: Vec<Drawable>,
}

impl Children {
    pub fn new() -> Self {
        Children {
            physics_drawables: Vec::new(),
            drawables: Vec::new(),
        }
    }
}

/// Basic context for use physics.
/// # Example
/// ```
/// // We need to create rigidbody and collider sets for move their to the structure
/// let rbs = RigidBodySet::new();
/// let cds = ColliderSet::new();
///
/// // For physics we need to create space
/// let space = PhysicsSpace::new();
///
/// // We need to create Physics Context
/// let ph_context = PhysicsContext::new(rbs, cds, space);
///
/// ph.step();
/// ```
pub struct PhysicsContext {
    pub rigid_body_set: RigidBodySet,
    pub collider_set: ColliderSet,
    pub space: PhysicsSpace,
}

pub struct PhysicsSpace {
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

pub struct Drawable {
    vertex: Vec<MyVertex>,
    transform: Transform,
}

pub struct PhysicsDrawable {
    rb_h: RigidBodyHandle,
    drawable: Drawable,
}

/// Бесполезный трейт. По идее должен создавать объекты на экран, но это можно организовать в чем-либо другом.
pub trait Objects {
    fn create_phys_object_from_shape(
        &mut self,
        position: Option<Vector>,
        shape: Shapes,
    ) -> PhysicsDrawable;
    fn create_phys_object(
        &mut self,
        position: Option<Vector>,
        shape: Vec<MyVertex>,
    ) -> PhysicsDrawable;
}

pub trait DrawableGPU {
    fn set_vertex(&mut self, vertex: Vec<MyVertex>);
}

impl Drawable {
    pub fn new(vertex: Vec<MyVertex>) -> Self {
        let transform = Transform {
            transform: [
                [1.0, 0.0, 0.0, 0.0],
                [0.0, 1.0, 0.0, 0.0],
                [0.0, 0.0, 1.0, 0.0],
                [0.0, 0.0, 0.0, 1.0],
            ],
        };

        Drawable { vertex, transform }
    }

    pub fn get_transform_copy(&self) -> Transform {
        self.transform.clone() // TODO: This method not the best, but idk what function I need instead of this 
    }

    pub fn get_transform(&self) -> &Transform {
        &self.transform
    }

    pub fn set_trasnform(&mut self, transform: Transform) {
        self.transform = transform;
    }
}

impl DrawableGPU for Drawable {
    fn set_vertex(&mut self, vertex: Vec<MyVertex>) {
        self.vertex = vertex;
    }
}

impl DrawableGPU for PhysicsDrawable {
    fn set_vertex(&mut self, vertex: Vec<MyVertex>) {
        self.drawable.set_vertex(vertex);
    }
}

impl PhysicsDrawable {
    pub fn new(rb_h: RigidBodyHandle, vertex: Vec<MyVertex>) -> Self {
        PhysicsDrawable {
            drawable: Drawable::new(vertex),
            rb_h,
        }
    }
    pub fn get_rb<'a>(&self, ctx: &'a mut PhysicsContext) -> &'a mut RigidBody {
        ctx.rigid_body_set.get_mut(self.rb_h).unwrap()
    }

    pub fn get_rb_handle(&self) -> RigidBodyHandle {
        self.rb_h
    }
    pub fn get_drawable(&self) -> &Drawable {
        &self.drawable
    }
    pub fn get_mut_drawable(&mut self) -> &mut Drawable {
        &mut self.drawable
    }
}

/// Используется для перемещения объекта в пространстве
pub trait DynamicObject {
    /// Teleports this rigid body to a new position (world coordinates).
    ///
    /// ⚠️ **Warning**: This instantly moves the body, ignoring physics! The body will "teleport"
    /// without checking for collisions in between. Use this for:
    /// - Respawning objects
    /// - Level transitions
    /// - Resetting positions
    ///
    /// For smooth physics-based movement, use velocities or forces instead.
    ///
    fn teleport(&mut self, ctx: &mut PhysicsContext, vec: Vector);
}

impl DynamicObject for PhysicsDrawable {
    fn teleport(&mut self, ctx: &mut PhysicsContext, vec: Vector) {
        let rb = self.get_rb(ctx);
        rb.set_translation(vec, false);
    }
}

impl PhysicsContext {
    pub fn new(
        rigid_body_set: RigidBodySet,
        collider_set: ColliderSet,
        space: PhysicsSpace,
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
    fn create_phys_object(
        &mut self,
        position: Option<Vector>,
        vertex: Vec<MyVertex>,
    ) -> PhysicsDrawable {
        let mut rigid_body_builder = RigidBodyBuilder::dynamic();
        if let Some(pos) = position {
            rigid_body_builder = rigid_body_builder.translation(pos);
        }
        let rigid_body = rigid_body_builder.build();
        let collider = ColliderBuilder::ball(0.5).restitution(0.7).build();
        let rb_h = self.rigid_body_set.insert(rigid_body);
        self.collider_set
            .insert_with_parent(collider, rb_h.clone(), &mut self.rigid_body_set);
        PhysicsDrawable::new(rb_h, vertex)
    }
    fn create_phys_object_from_shape(
        &mut self,
        position: Option<Vector>,
        shape: Shapes,
    ) -> PhysicsDrawable {
        let vertex: Vec<MyVertex> = get_vertex_from_shapes(shape);
        let mut rigid_body_builder = RigidBodyBuilder::dynamic();
        if let Some(pos) = position {
            rigid_body_builder = rigid_body_builder.translation(pos);
        }
        let rigid_body = rigid_body_builder.build();
        let collider = ColliderBuilder::ball(0.5).restitution(0.7).build();
        let rb_h = self.rigid_body_set.insert(rigid_body);
        self.collider_set
            .insert_with_parent(collider, rb_h.clone(), &mut self.rigid_body_set);
        PhysicsDrawable::new(rb_h, vertex)
    }
}

impl PhysicsSpace {
    pub fn new() -> Self {
        PhysicsSpace {
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
#[derive(BufferContents, Clone, Copy, Debug)]
pub struct Transform {
    transform: [[f32; 4]; 4],
}

impl Position for Transform {
    fn get_matrix_mut(&mut self) -> &mut [[f32; 4]; 4] {
        &mut self.transform
    }
    fn get_matrix(&self) -> &[[f32; 4]; 4] {
        &self.transform
    }
}
