use std::sync::Arc;

use winit::{event::WindowEvent, event_loop::ActiveEventLoop, window::Window};

use crate::{
    cmd::command::CommandQueue, drw::children::Children, res::assets::Storage,
    threading::scheduler::SchedulerContext,
};

pub trait StartFn:
    FnMut(
    &ActiveEventLoop,
    Arc<Children>,
    Arc<Storage>,
    Arc<Window>,
    Arc<SchedulerContext>,
) -> CommandQueue
{
}

impl<T> StartFn for T where
    T: FnMut(
        &ActiveEventLoop,
        Arc<Children>,
        Arc<Storage>,
        Arc<Window>,
        Arc<SchedulerContext>,
    ) -> CommandQueue
{
}

pub trait RedrawFn:
    FnMut(Arc<Children>, Arc<Storage>, &WindowEvent, Arc<SchedulerContext>) -> CommandQueue
{
}

impl<T> RedrawFn for T where
    T: FnMut(Arc<Children>, Arc<Storage>, &WindowEvent, Arc<SchedulerContext>) -> CommandQueue
{
}
