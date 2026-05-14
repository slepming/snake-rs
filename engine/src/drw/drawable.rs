use std::sync::Arc;

use rapier2d::{
    math::Vec2,
    prelude::{RigidBody, RigidBodyHandle},
};
use vulkano::pipeline::GraphicsPipeline;

use crate::{
    MyVertex,
    drw::texture::Texture,
    geom::{matrix::Transform, shapes::Shapes},
    mv::{phys::movement::PhysicsContext, transform::Entity},
    res::cache::{Cache, PipelineHandle},
};

use crate::res::cache::DescriptorHandle;
use color::Rgba8;
use vulkano::descriptor_set::DescriptorSet;

pub struct Drawable {
    transform: Transform,
    color: Rgba8,
    pub(crate) cache: Arc<Cache>,
    pub(crate) render: DrawableRenderContext,
}

pub(crate) struct PipelineID {
    pub id: String,
}

pub(crate) struct DescriptorID {
    pub id: String,
}

pub(crate) struct DrawableRenderContext {
    pub(crate) descriptor_id: DescriptorID,
    pipeline_id: PipelineID,
    mesh: Mesh,
}

pub struct DrawableCreateInfo {
    pub cache: Option<Arc<Cache>>,
    pub position: Vec2,
    pub texture: Option<Texture>,
    pub radius: f32,
    pub thickness: f32,
    pub size: Vec2,
    pub id: u32,
    pub color: Rgba8,
}

impl Default for DrawableCreateInfo {
    fn default() -> Self {
        Self {
            cache: Default::default(),
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

pub struct Children<D: DrawableComponent + 'static> {
    // I think iterations through Vector with Box is very slowly operation, but I dont know how I to
    // make this faster. And I must replace Box reference.
    pub drawables: Vec<D>,
}

impl<D: DrawableComponent> Children<D> {
    pub fn new() -> Self {
        Children {
            drawables: Vec::new(),
        }
    }

    pub fn add_drawable(&mut self, item: D) {
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
    /// Get pipeline clone
    /// # Returns
    /// Pipeline clone
    fn get_pipeline(&self) -> Arc<GraphicsPipeline>;
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
    pub fn new(
        vertex: Vec<MyVertex>,
        id: u32,
        cache: Arc<Cache>,
        key: &'static str,
        position: Option<Vec2>,
    ) -> Self {
        let pos = position.unwrap_or(Vec2::new(1.0, 1.0));
        let transform = Transform {
            transform: [
                [1.0, 0.0, 0.0, 0.0],
                [0.0, 1.0, 0.0, 0.0],
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
            cache,
            render: DrawableRenderContext {
                descriptor_id: DescriptorID {
                    id: key.to_string(),
                },
                pipeline_id: PipelineID {
                    id: key.to_string(),
                },
                mesh: Mesh::new(vertex, id),
            },
        }
    }

    pub fn new_with_color(
        drawable_info: DrawableCreateInfo,
        key: &'static str,
        vertex: Vec<MyVertex>,
        descriptor_set: Option<Arc<DescriptorSet>>,
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

        let mut drawable = Drawable {
            color: drawable_info.color,
            transform,
            cache: drawable_info.cache.as_ref().unwrap().clone(),
            render: DrawableRenderContext {
                descriptor_id: DescriptorID {
                    id: key.to_string(),
                },
                pipeline_id: PipelineID {
                    id: key.to_string(),
                },
                mesh: Mesh::new(vertex, drawable_info.id),
            },
        };

        if let Some(desc) = descriptor_set {
            let desc_key = format!("{}_{}", key, drawable_info.id);
            drawable_info
                .cache
                .as_ref()
                .unwrap()
                .insert_descriptor_set(desc_key.clone(), desc);
            drawable.render.descriptor_id.id = desc_key;
        }

        drawable
    }

    pub fn from_shape(shape: Shapes, drw: DrawableCreateInfo) -> Self {
        let pipeline: &'static str = shape.clone().into();
        let key = Box::leak(pipeline.to_lowercase().into_boxed_str()); // Potential memory leak
        let (vertex, desc) = shape.get_vertex_and_descriptor(drw.cache.as_ref().unwrap());
        Drawable::new_with_color(drw, key, vertex, desc)
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

    fn get_pipeline(&self) -> Arc<GraphicsPipeline> {
        self.cache
            .get_pipeline(&self.render.pipeline_id.id)
            .unwrap()
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

    fn get_pipeline(&self) -> Arc<GraphicsPipeline> {
        self.get_drawable().get_pipeline()
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
