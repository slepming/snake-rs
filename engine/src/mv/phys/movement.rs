use rapier2d::{math::Vec2, prelude::{CCDSolver, ColliderBuilder, ColliderSet, DefaultBroadPhase, ImpulseJointSet, IntegrationParameters, IslandManager, MultibodyJointSet, NarrowPhase, PhysicsPipeline, RigidBodyBuilder, RigidBodySet}};
use tracing::debug;

use crate::{MyVertex, drw::drawable::{Drawable, PhysicsDrawable}, geom::shapes::Shapes};

const GRAVITY: Vec2 = Vec2::new(0.0, -9.81 * 60.0); // * 60 is magick value. I will fix that in the future

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
    fn teleport(&mut self, ctx: &mut PhysicsContext, vec: Vec2);
}

impl DynamicObject for PhysicsDrawable {
    fn teleport(&mut self, ctx: &mut PhysicsContext, vec: Vec2) {
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
        position: Option<Vec2>,
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
        let drawable = Drawable::new(vertex, id, None);
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
        mut rigid_body_builder: RigidBodyBuilder,
        size: [f32; 2],
        id: u32,
        position: Option<Vec2>,
    ) -> PhysicsDrawable {
        #[cfg(feature = "tracing")]
        let _span = tracy_client::span!("PhysicsContext::create_phys_square");
        if let Some(pos) = position {
            rigid_body_builder = rigid_body_builder.translation(pos);
        }
        let rigid_body = rigid_body_builder.build();
        let collider = ColliderBuilder::cuboid(size[0], size[1]).build();
        let rb_h = self.rigid_body_set.insert(rigid_body.clone());
        self.collider_set.insert_with_parent(
            collider.clone(),
            rb_h.clone(),
            &mut self.rigid_body_set,
        );
        let drawable = Drawable::from_shape(Shapes::Square(size), id, None);
        debug!(
            id = id,
            "created new object:\n\
             position: {:?}\n\
             size: {:?}\n\n
             rigid_body: {:?}\n\n
             collider: {:?}\n\n
             rb_handle: {:?}",
            &position,
            &size,
            &rigid_body,
            &collider,
            &rb_h
        );
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
