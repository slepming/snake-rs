use rapier2d::{
    math::Vec2,
    prelude::{RigidBody, RigidBodyHandle},
};

use crate::{
    MyVertex,
    geom::{
        matrix::Transform,
        shapes::{Shapes, get_vertex_from_shapes},
    },
    mv::{phys::movement::PhysicsContext, transform::Entity},
};

use color::Rgba8;

pub struct Children {
    // I think iterations through Vector with Box is very slowly operation, but I dont know how I to
    // make this faster
    pub drawables: Vec<Box<dyn DrawableGPU>>,
    pub physics_drawables: Vec<Box<dyn Entity>>,
}

impl Children {
    pub fn new() -> Self {
        Children {
            drawables: Vec::new(),
            physics_drawables: Vec::new(),
        }
    }

    pub fn add_physics<T: Entity + 'static>(&mut self, item: T) {
        self.physics_drawables.push(Box::new(item));
    }

    pub fn add_drawable<T: DrawableGPU + 'static>(&mut self, item: T) {
        self.drawables.push(Box::new(item));
    }
}

pub struct Drawable {
    transform: Transform,
    color: Rgba8,
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
    fn drawable(&self) -> &Drawable;
    fn drawable_mut(&mut self) -> &mut Drawable;
    fn get_colour(&self) -> &Rgba8;
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
    pub fn new(vertex: Vec<MyVertex>, id: u32, position: Option<Vec2>) -> Self {
        let pos = position.unwrap_or(Vec2::new(1.0, 1.0));
        let transform = Transform {
            transform: [
                [1.0, 0.0, 0.0, pos[0]],
                [0.0, 1.0, 0.0, pos[1]],
                [0.0, 0.0, 1.0, 0.0],
                [0.0, 0.0, 0.0, 1.0],
            ],
        };

        Drawable {
            mesh: Mesh::new(vertex, id),
            color: Rgba8 { r: 0, g: 0, b: 0, a: 255 },
            transform,
        }
    }

    pub fn new_with_color(vertex: Vec<MyVertex>, color: Rgba8, id: u32, position: Option<Vec2>) -> Self {
        let pos = position.unwrap_or(Vec2::new(1.0, 1.0));
        let transform = Transform {
            transform: [
                [1.0, 0.0, 0.0, pos[0]],
                [0.0, 1.0, 0.0, pos[1]],
                [0.0, 0.0, 1.0, 0.0],
                [0.0, 0.0, 0.0, 1.0],
            ],
        };

        Drawable {
            mesh: Mesh::new(vertex, id),
            color,
            transform,
        }
    }

    pub fn from_shape(shape: Shapes, col: Rgba8, id: u32, position: Option<Vec2>) -> Self {
        Drawable::new_with_color(get_vertex_from_shapes(shape), col, id, position,)
    }
}

impl DrawableGPU for Drawable {
    fn set_vertex(&mut self, vertex: Vec<MyVertex>) {
        self.mesh.vertex = vertex;
    }

    fn get_transform(&self) -> &Transform {
        &self.transform
    }

    fn get_transform_clone(&self) -> Transform {
        self.transform.clone() // TODO: This method not the best, but idk what function I need instead of this 
    }

    fn get_vertex_clone(&self) -> Vec<MyVertex> {
        self.mesh.vertex.clone()
    }

    fn get_vertex(&self) -> &Vec<MyVertex> {
        &self.mesh.vertex
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

    fn get_colour(&self) -> &Rgba8 {
        &self.color
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

    fn drawable(&self) -> &Drawable {
        self.get_drawable()
    }

    fn drawable_mut(&mut self) -> &mut Drawable {
        self.get_mut_drawable()
    }

    fn get_colour(&self) -> &Rgba8 {
        &self.drawable.color
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
