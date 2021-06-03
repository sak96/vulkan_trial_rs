use std::sync::Arc;

use vulkano::{
    buffer::CpuAccessibleBuffer,
    command_buffer::{DynamicState, PrimaryAutoCommandBuffer},
    device::{Device, Queue},
    image::SwapchainImage,
    pipeline::viewport::Viewport,
    render_pass::{FramebufferAbstract, RenderPass},
    swapchain::{Surface, Swapchain, SwapchainCreationError},
};
use winit::window::Window;

use crate::{pipeline::ConcreteGraphicsPipeline, vertex::Vertex};

pub struct ResizeHelper(DynamicState);

impl ResizeHelper {
    pub fn new() -> Self {
        Self(DynamicState::none())
    }

    pub fn resize(
        &mut self,
        pipeline: &Arc<ConcreteGraphicsPipeline>,
        surface: &Arc<Surface<Window>>,
        device: &Arc<Device>,
        vertex_buffer: &Arc<CpuAccessibleBuffer<[Vertex]>>,
        graphical_queue: &Arc<Queue>,
        renderpass: &Arc<RenderPass>,
        swapchain: &mut Arc<Swapchain<Window>>,
        framebuffers: &mut Vec<Arc<dyn FramebufferAbstract + Send + Sync>>,
        cmd_buffer: &mut Option<Vec<Arc<PrimaryAutoCommandBuffer>>>,
        images: &mut Vec<Arc<SwapchainImage<Window>>>,
    ) -> bool {
        let dim: [u32; 2] = surface.window().inner_size().into();
        let (new_swapchain, new_images) = match swapchain.recreate().dimensions(dim).build() {
            Ok(r) => r,
            Err(SwapchainCreationError::UnsupportedDimensions) => return false,
            Err(e) => panic!("Failed to recreate swapchain: {:?}", e),
        };
        *swapchain = new_swapchain;
        *images = new_images;
        *framebuffers = crate::framebuffers::get_frame_buffer(&images, &renderpass);
        self.resize_using_dynamic_state(&swapchain);
        *cmd_buffer = Some(crate::commandbuffers::get_command_buffers(
            &pipeline,
            &graphical_queue,
            &device,
            &framebuffers,
            &vertex_buffer,
            self,
        ));
        true
    }

    fn resize_using_dynamic_state(&mut self, swapchain: &Arc<Swapchain<Window>>) {
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
