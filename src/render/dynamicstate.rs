use std::sync::Arc;

use vulkano::{
    command_buffer::DynamicState,
    device::Device,
    image::SwapchainImage,
    pipeline::viewport::Viewport,
    render_pass::{FramebufferAbstract, RenderPass},
    swapchain::{Surface, Swapchain, SwapchainCreationError},
};
use winit::window::Window;

pub struct ResizeHelper(DynamicState);

impl ResizeHelper {
    pub fn new(swapchain: &Arc<Swapchain<Window>>) -> Self {
        let mut resizer = Self(DynamicState::none());
        resizer.resize_using_dynamic_state(&swapchain);
        resizer
    }

    pub fn resize(
        &mut self,
        device: &Arc<Device>,
        surface: &Arc<Surface<Window>>,
        renderpass: &Arc<RenderPass>,
        swapchain: &mut Arc<Swapchain<Window>>,
        framebuffers: &mut Vec<Arc<dyn FramebufferAbstract + Send + Sync>>,
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
        *framebuffers =
            super::framebuffers::get_frame_buffer(&swapchain, &device, &images, &renderpass);
        self.resize_using_dynamic_state(&swapchain);
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
