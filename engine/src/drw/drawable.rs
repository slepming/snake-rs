use std::sync::Arc;

use rapier2d::{
    math::Vec2,
    prelude::{RigidBody, RigidBodyHandle},
};
use vulkano::{
    descriptor_set::allocator::DescriptorSetAllocator, image::sampler::Sampler,
    memory::allocator::MemoryAllocator, pipeline::Pipeline,
};

use crate::{
    MyVertex,
    drw::texture::Texture,
    geom::{matrix::Transform, shapes::Shapes},
    mv::{phys::movement::PhysicsContext, transform::Entity},
};

use color::Rgba8;
use vulkano::descriptor_set::DescriptorSet;

pub struct Drawable {
    transform: Transform,
    color: Rgba8,
    pub(crate) render: DrawableRenderContext,
}

pub(crate) struct PipelineID {
    pub id: String,
}

pub(crate) struct DescriptorID {
    pub id: String,
}

pub(crate) struct DrawableRenderContext {
    /// Memory descriptor key(id). Drawable doesn't know anything about the descriptor
    pub(crate) descriptor_id: DescriptorID,
    /// Pipeline key(id). Drawable doesn't know anything about the pipeline
    pub(crate) pipeline_id: PipelineID,
    mesh: Mesh,
}

pub struct DrawableCreateInfo {
    pub position: Vec2,
    pub texture: Option<Texture>,
    /// Set radius to circle object
    pub radius: f32,
    pub thickness: f32,
    pub size: Vec2,
    pub id: u32,
    pub color: Rgba8,
}

impl DrawableCreateInfo {
    pub fn set_position(mut self, position: Vec2) -> Self {
        self.position = position;
        self
    }

    pub fn set_texture(mut self, texture: Texture) -> Self {
        self.texture = Some(texture);
        self
    }

    pub fn set_radius(mut self, radius: f32) -> Self {
        self.radius = radius;
        self
    }

    pub fn set_thickness(mut self, thickness: f32) -> Self {
        self.thickness = thickness;
        self
    }

    pub fn set_size(mut self, size: Vec2) -> Self {
        self.size = size;
        self
    }

    pub fn set_id(mut self, id: u32) -> Self {
        self.id = id;
        self
    }

    pub fn set_color(mut self, color: Rgba8) -> Self {
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
                r: 0,
                g: 0,
                b: 0,
                a: 1,
            },
        }
    }
}

pub struct Children {
    // I think iterations through Vector with Box is very slowly operation, but I dont know how I to
    // make this faster. And I must replace Box reference.
    pub drawables: Vec<Drawable>,
}

impl Children {
    pub fn new() -> Self {
        Children {
            drawables: Vec::new(),
        }
    }

    pub fn add_drawable(&mut self, item: Drawable) {
        self.drawables.push(item);
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
    fn get_transform(&self) -> &Transform;
    fn get_transform_clone(&self) -> Transform;
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
    pub fn new(vertex: Vec<MyVertex>, key: &'static str, create_info: DrawableCreateInfo) -> Self {
        let pos = create_info.position;
        let transform = Transform {
            transform: [
                [create_info.size[0], 0.0, 0.0, 0.0],
                [0.0, create_info.size[1], 0.0, 0.0],
                [0.0, 0.0, 1.0, 0.0],
                [pos[0], pos[1], 0.0, 1.0],
            ],
        };

        Drawable {
            color: Rgba8 {
                r: 0,
                g: 0,
                b: 0,
                a: 255,
            },
            transform,
            render: DrawableRenderContext {
                descriptor_id: DescriptorID {
                    id: key.to_string(),
                },
                pipeline_id: PipelineID {
                    id: key.to_string(),
                },
                mesh: Mesh::new(vertex, create_info.id),
            },
        }
    }

    pub fn new_with_color(
        drawable_info: DrawableCreateInfo,
        key: &'static str,
        vertex: Vec<MyVertex>,
    ) -> Self {
        let key = key.to_string().to_lowercase();
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
                descriptor_id: DescriptorID {
                    id: key.clone(), // If descriptor id is key then few circles will have same
                                     // data
                },
                pipeline_id: PipelineID { id: key },
                mesh: Mesh::new(vertex, drawable_info.id),
            },
        };

        drawable
    }

    pub fn from_shape(
        shape: Shapes,
        drw: DrawableCreateInfo,
        pipeline: Arc<dyn Pipeline>,
        mem_alloc: Arc<dyn MemoryAllocator>,
        desc_alloc: Arc<dyn DescriptorSetAllocator>,
        sampler: Option<Arc<Sampler>>,
    ) -> (Self, Option<Arc<DescriptorSet>>) {
        let (vertex, desc) =
            shape.get_vertex_and_descriptor(pipeline, mem_alloc, desc_alloc, sampler);
        (Drawable::new_with_color(drw, shape.into(), vertex), desc)
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
    fn get_transform(&self) -> &Transform {
        &self.transform
    }

    fn get_transform_clone(&self) -> Transform {
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
    fn get_transform(&self) -> &Transform {
        self.drawable.get_transform()
    }

    fn get_transform_clone(&self) -> Transform {
        self.drawable.get_transform_clone()
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
