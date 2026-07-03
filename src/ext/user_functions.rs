use std::sync::Arc;

use winit::{event::WindowEvent, event_loop::ActiveEventLoop, window::Window};

use crate::{cmd::command::CommandQueue, drw::children::Children, res::assets::Storage};

pub trait StartFn:
    FnMut(&ActiveEventLoop, Arc<Children>, &mut Storage, Arc<Window>, &mut CommandQueue)
{
}

impl<T> StartFn for T where
    T: FnMut(&ActiveEventLoop, Arc<Children>, &mut Storage, Arc<Window>, &mut CommandQueue)
{
}

pub trait RedrawFn: FnMut(Arc<Children>, &mut Storage, &WindowEvent, &mut CommandQueue) {}

impl<T> RedrawFn for T where T: FnMut(Arc<Children>, &mut Storage, &WindowEvent, &mut CommandQueue) {}
