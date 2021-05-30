use std::sync::Arc;

use vulkano::{command_buffer::DynamicState, pipeline::viewport::Viewport, swapchain::Swapchain};
use winit::window::Window;

pub struct ResizeHelper(DynamicState);

impl ResizeHelper {
    pub fn new() -> Self {
        Self(DynamicState::none())
    }

    pub fn resize_using_dynamic_state(&mut self, swapchain: &Arc<Swapchain<Window>>) {
        let dim = swapchain.dimensions();
        let viewport = Viewport {
            origin: [0.0, 0.0],
            dimensions: [dim[0] as f32, dim[1] as f32],
            depth_range: 0.0..1.0,
        };
        self.0.viewports = Some(vec![viewport]);
    }

    pub fn inner(&self) -> &DynamicState {
        &self.0
    }
}
