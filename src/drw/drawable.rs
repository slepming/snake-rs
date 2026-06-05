//! Managing Drawable states

use std::sync::Arc;

use rapier2d::{
    math::Vec2,
    prelude::{RigidBody, RigidBodyHandle},
};
use vulkano::{
    descriptor_set::allocator::DescriptorSetAllocator, image::sampler::Sampler,
    memory::allocator::MemoryAllocator,
};

use crate::{
    MyVertex,
    drw::texture::Texture,
    geom::{matrix::Transform, shapes::Shapes},
    mv::{phys::movement::PhysicsContext, transform::{Entity, Positioned}},
    res::cache::{DescriptorSetCache, PipelineCache},
};

use color::Rgba8;
use vulkano::descriptor_set::DescriptorSet;

/// The main element that is rendered by the Vulkan
pub struct Drawable {
    transform: Transform,
    color: Rgba8,
    pub(crate) render: DrawableRenderContext,
}

/// Pipeline id structure
#[derive(Clone)]
pub struct PipelineID {
    /// String ID to search pipelines
    pub id: String,
}

/// Descriptor set id structure
#[derive(Clone)]
pub struct DescriptorID {
    /// String ID to search descriptor sets
    pub id: String,
}

pub(crate) struct DrawableRenderContext {
    /// Memory descriptor key(id). Drawable doesn't know anything about the descriptor
    pub(crate) descriptor_id: DescriptorID,
    /// Pipeline key(id). Drawable doesn't know anything about the pipeline
    pub(crate) pipeline_id: PipelineID,
    pub mesh: Mesh,
}

/// Information about the object to be drawn
pub struct DrawableCreateInfo {
    /// Positional coordinates
    pub position: Vec2,
    /// Texture for drawable object
    pub texture: Option<Texture>,
    /// Set radius to circle object
    pub radius: f32,
    /// TODO
    pub thickness: f32,
    /// Drawable object size
    pub size: Vec2,
    pub(crate) id: u32,
    /// Object color
    pub color: Rgba8,
}

impl DrawableCreateInfo {
    /// Sets the position of the object
    ///
    /// # Returns
    /// [`Drawable`]
    pub fn with_position(mut self, position: Vec2) -> Self {
        self.position = position;
        self
    }

    /// Sets the texture of the object(if supported)
    ///
    /// # Returns
    /// [`Drawable`]
    pub fn with_texture(mut self, texture: Texture) -> Self {
        self.texture = Some(texture);
        self
    }

    /// Sets the radius of the object(if supported)
    ///
    /// # Returns
    /// [`Drawable`]
    pub fn with_radius(mut self, radius: f32) -> Self {
        self.radius = radius;
        self
    }

    /// Sets the thickness of the object(if supported)
    ///
    /// # Returns
    /// [`Drawable`]
    pub fn with_thickness(mut self, thickness: f32) -> Self {
        self.thickness = thickness;
        self
    }

    /// Sets the size of the object
    ///
    /// # Returns
    /// [`Drawable`]
    pub fn with_size(mut self, size: Vec2) -> Self {
        self.size = size;
        self
    }

    /// Sets id of the object
    ///
    /// # Returns
    /// [`Drawable`]
    pub(crate) fn with_id(mut self, id: u32) -> Self {
        self.id = id;
        self
    }

    /// Sets the color for the object
    ///
    /// # Returns
    /// [`Drawable`]
    pub fn with_color(mut self, color: Rgba8) -> Self {
        self.color = color;
        self
    }
}

impl Default for DrawableCreateInfo {
    fn default() -> Self {
        Self {
            texture: Default::default(),
            position: Default::default(),
            radius: Default::default(),
            thickness: Default::default(),
            size: Default::default(),
            id: Default::default(),
            color: Rgba8 {
                r: 255,
                g: 255,
                b: 255,
                a: 1,
            },
        }
    }
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
    #[allow(dead_code)]
    fn set_vertex(&mut self, vertex: Vec<MyVertex>);
    #[allow(dead_code)]
    fn get_vertex_clone(&self) -> Vec<MyVertex>;
    fn get_vertex(&self) -> &Vec<MyVertex>;
    /// # Returns
    /// Colour for shader
    fn get_colour(&self) -> &Rgba8;
}

pub trait DrawableComponent: DrawableGPU {
    fn transform(&self) -> &Transform;
    fn transform_mut(&mut self) -> &mut Transform;
    fn transform_clone(&self) -> Transform;
    fn set_transform(&mut self, transform: Transform);
    /// # Returns
    /// Reference to drawable
    fn drawable(&self) -> &Drawable;
    /// # Returns
    /// Mutable drawable
    fn drawable_mut(&mut self) -> &mut Drawable;
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
    pub fn new(
        drawable_info: DrawableCreateInfo,
        pipeline_id: PipelineID,
        descriptor_id: DescriptorID,
        vertex: Vec<MyVertex>,
    ) -> Self {
        let pos = drawable_info.position;
        let transform = Transform {
            transform: [
                [drawable_info.size[0], 0.0, 0.0, 0.0],
                [0.0, drawable_info.size[1], 0.0, 0.0],
                [0.0, 0.0, 1.0, 0.0],
                [pos[0], pos[1], 0.0, 1.0],
            ],
        };

        let drawable = Drawable {
            color: drawable_info.color,
            transform,
            render: DrawableRenderContext {
                descriptor_id,
                pipeline_id,
                mesh: Mesh::new(vertex, drawable_info.id),
            },
        };

        drawable
    }

    /// Creates allocations, pipeline descriptors for drawable and calls [`Self::new_with_color`]
    ///
    /// # Returns
    /// ([`Drawable`], [`vulkano::descriptor_set::DescriptorSet`])
    pub fn from_shape(
        shape: Shapes,
        drw: DrawableCreateInfo,
        mem_alloc: Arc<dyn MemoryAllocator>,
        desc_alloc: Arc<dyn DescriptorSetAllocator>,
        pipeline_cache: Arc<PipelineCache>,
        desc_cache: Arc<DescriptorSetCache>,
        sampler: Option<Arc<Sampler>>,
    ) -> (Self, Option<Arc<DescriptorSet>>) {
        let key_raw: &'static str = shape.clone().into();
        let key = key_raw.to_string().to_lowercase();
        let pipeline_id = PipelineID { id: key.clone() };

        let descriptor_id = DescriptorID { id: key.clone() };

        let (vertex, desc) = shape.get_vertex_and_descriptor(
            pipeline_id.clone(),
            descriptor_id.clone(),
            mem_alloc,
            desc_alloc,
            desc_cache,
            pipeline_cache,
            sampler,
        );
        (Drawable::new(drw, pipeline_id, descriptor_id, vertex), desc)
    }
}

impl DrawableGPU for Drawable {
    fn set_vertex(&mut self, vertex: Vec<MyVertex>) {
        self.render.mesh.vertex = vertex;
    }

    fn get_vertex_clone(&self) -> Vec<MyVertex> {
        self.render.mesh.vertex.clone()
    }

    fn get_vertex(&self) -> &Vec<MyVertex> {
        &self.render.mesh.vertex
    }

    fn get_colour(&self) -> &Rgba8 {
        &self.color
    }
}

impl DrawableComponent for Drawable {
    fn transform(&self) -> &Transform {
        &self.transform
    }

    fn transform_clone(&self) -> Transform {
        self.transform.clone() // TODO: This method not the best, but idk what function I need instead of this 
    }

    fn set_transform(&mut self, transform: Transform) {
        self.transform = transform;
    }

    fn drawable(&self) -> &Drawable {
        &self
    }

    fn drawable_mut(&mut self) -> &mut Drawable {
        self
    }

    fn transform_mut(&mut self) -> &mut Transform {
        &mut self.transform
    }
}

impl DrawableGPU for PhysicsDrawable {
    fn set_vertex(&mut self, vertex: Vec<MyVertex>) {
        self.drawable.set_vertex(vertex);
    }

    fn get_vertex_clone(&self) -> Vec<MyVertex> {
        self.drawable.get_vertex_clone()
    }

    fn get_vertex(&self) -> &Vec<MyVertex> {
        self.drawable.get_vertex()
    }

    fn get_colour(&self) -> &Rgba8 {
        &self.drawable.color
    }
}

impl DrawableComponent for PhysicsDrawable {
    fn transform(&self) -> &Transform {
        self.drawable.transform()
    }

    fn transform_clone(&self) -> Transform {
        self.drawable.transform_clone()
    }

    fn set_transform(&mut self, transform: Transform) {
        self.drawable.set_transform(transform);
    }

    fn drawable(&self) -> &Drawable {
        self.get_drawable()
    }

    fn drawable_mut(&mut self) -> &mut Drawable {
        self.get_mut_drawable()
    }

    fn transform_mut(&mut self) -> &mut Transform {
        &mut self.drawable.transform
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

impl Entity for PhysicsDrawable {
    fn rigid_body<'a>(&self, ctx: &'a mut PhysicsContext) -> &'a mut RigidBody {
        self.get_rb(ctx)
    }

    fn rb_handle(&self) -> RigidBodyHandle {
        self.get_rb_handle()
    }
}

impl Positioned for Drawable {
    fn position(&self) -> Vec2 {
        let transform = self.transform.transform.as_ref();

        Vec2::new(transform[0][0], transform[1][1])
    }

    fn set_position(&mut self, vec: Vec2) {
        let current_transform = self.transform_mut();

        current_transform.transform[0][0] = vec.x;
        current_transform.transform[1][1] = vec.y;
    }
}
