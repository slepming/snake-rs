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
    &mut Storage,
    Arc<Window>,
    &mut CommandQueue,
    Arc<SchedulerContext>,
)
{
}

impl<T> StartFn for T where
    T: FnMut(
        &ActiveEventLoop,
        Arc<Children>,
        &mut Storage,
        Arc<Window>,
        &mut CommandQueue,
        Arc<SchedulerContext>,
    )
{
}

pub trait RedrawFn:
    FnMut(Arc<Children>, &mut Storage, &WindowEvent, &mut CommandQueue, Arc<SchedulerContext>)
{
}

impl<T> RedrawFn for T where
    T: FnMut(Arc<Children>, &mut Storage, &WindowEvent, &mut CommandQueue, Arc<SchedulerContext>)
{
}
