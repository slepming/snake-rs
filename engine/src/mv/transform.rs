use std::fmt::Display;

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

const GRAVITY: Vector = Vector::new(0.0, -9.81 * 60.0); // * 60 is magick value. I will fix that in the future

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

/// Contains all files for using physics.
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
    transform: Transform,
    mesh: Mesh,
}

pub struct Mesh {
    vertex: Vec<MyVertex>,
    /// ID need for find matrix in buffer
    id: u32,
}

pub struct PhysicsDrawable {
    rb_h: RigidBodyHandle,
    drawable: Drawable,
}

pub trait DrawableGPU {
    fn set_vertex(&mut self, vertex: Vec<MyVertex>);
    fn get_transform(&self) -> &Transform;
    fn get_transform_clone(&self) -> Transform;
    fn get_vertex_clone(&self) -> Vec<MyVertex>;
    fn get_vertex(&self) -> &Vec<MyVertex>;
    fn set_transform(&mut self, transform: Transform);
}

impl Mesh {
    pub fn new(ver: Vec<MyVertex>, id: u32) -> Self {
        Mesh { vertex: ver, id }
    }

    pub fn get_id(&self) -> &u32 {
        &self.id
    }
}

impl Drawable {
    pub fn new(vertex: Vec<MyVertex>, id: u32) -> Self {
        let transform = Transform {
            transform: [
                [1.0, 0.0, 0.0, 0.0],
                [0.0, 1.0, 0.0, 0.0],
                [0.0, 0.0, 1.0, 0.0],
                [0.0, 0.0, 0.0, 1.0],
            ],
        };

        Drawable {
            mesh: Mesh::new(vertex, id),
            transform,
        }
    }

    pub fn from_shape(shape: Shapes, id: u32) -> Self {
        Drawable::new(get_vertex_from_shapes(shape), id)
    }
}

impl DrawableGPU for Drawable {
    fn get_transform_clone(&self) -> Transform {
        self.transform.clone() // TODO: This method not the best, but idk what function I need instead of this 
    }

    fn get_vertex_clone(&self) -> Vec<MyVertex> {
        self.mesh.vertex.clone()
    }

    fn get_vertex(&self) -> &Vec<MyVertex> {
        &self.mesh.vertex
    }

    fn get_transform(&self) -> &Transform {
        &self.transform
    }

    fn set_transform(&mut self, transform: Transform) {
        self.transform = transform;
    }
    fn set_vertex(&mut self, vertex: Vec<MyVertex>) {
        self.mesh.vertex = vertex;
    }
}

impl DrawableGPU for PhysicsDrawable {
    fn set_vertex(&mut self, vertex: Vec<MyVertex>) {
        self.drawable.set_vertex(vertex);
    }

    fn get_transform(&self) -> &Transform {
        self.drawable.get_transform()
    }

    fn get_transform_clone(&self) -> Transform {
        self.drawable.get_transform_clone()
    }

    fn get_vertex_clone(&self) -> Vec<MyVertex> {
        self.drawable.get_vertex_clone()
    }

    fn get_vertex(&self) -> &Vec<MyVertex> {
        self.drawable.get_vertex()
    }

    fn set_transform(&mut self, transform: Transform) {
        self.drawable.set_transform(transform);
    }
}

impl PhysicsDrawable {
    pub fn new(rb_h: RigidBodyHandle, drawable: Drawable) -> Self {
        PhysicsDrawable { drawable, rb_h }
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

    /// Step for physics calculates
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

    /// Create physical drawable object with physical size which will be converted to graphical size. WIP
    /// # Parameters
    /// `position` -> position where drawable object must spawn
    /// `vertex` -> custom vertices for draw
    /// `id` -> object id in engine array.
    // TODO: Currently the id is not finished and WIP
    pub fn create_phys_object(
        &mut self,
        position: Option<Vector>,
        vertex: Vec<MyVertex>,
        id: u32,
    ) -> PhysicsDrawable {
        let mut rigid_body_builder = RigidBodyBuilder::dynamic();
        if let Some(pos) = position {
            rigid_body_builder = rigid_body_builder.translation(pos);
        }
        let rigid_body = rigid_body_builder.build();
        let collider = ColliderBuilder::cuboid(0.3, 0.3).build();
        let rb_h = self.rigid_body_set.insert(rigid_body);
        self.collider_set
            .insert_with_parent(collider, rb_h.clone(), &mut self.rigid_body_set);
        let drawable = Drawable::new(vertex, id);
        PhysicsDrawable::new(rb_h, drawable)
    }

    /// Create physical drawable object with physical size which will be converted to graphical size
    /// # Parameters
    /// `size` -> drawable object size
    /// `position` -> position where drawable object must spawn
    /// `id` -> object id in engine array.
    // TODO: Currently the id is not finished and WIP
    pub fn create_phys_square(
        &mut self,
        position: Option<Vector>,
        mut rigid_body_builder: RigidBodyBuilder,
        size: [f32; 2],
        id: u32,
    ) -> PhysicsDrawable {
        if let Some(pos) = position {
            rigid_body_builder = rigid_body_builder.translation(pos);
        }
        let rigid_body = rigid_body_builder.build();
        let collider = ColliderBuilder::cuboid(size[0], size[1]).build();
        let rb_h = self.rigid_body_set.insert(rigid_body);
        self.collider_set
            .insert_with_parent(collider, rb_h.clone(), &mut self.rigid_body_set);
        let drawable = Drawable::from_shape(Shapes::Square(size), id);
        PhysicsDrawable::new(rb_h, drawable)
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

impl Display for Transform {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let fmt = format!(
            "\n{:?}\n{:?}\n{:?}\n{:?}",
            self.transform[0], self.transform[1], self.transform[2], self.transform[3]
        );
        write!(f, "{}", fmt)
    }
}

impl Position for Transform {
    fn get_matrix_mut(&mut self) -> &mut [[f32; 4]; 4] {
        &mut self.transform
    }
    fn get_matrix(&self) -> &[[f32; 4]; 4] {
        &self.transform
    }
}
