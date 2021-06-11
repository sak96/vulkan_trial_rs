use std::sync::Arc;

use vulkano::{
    device::Device,
    format::Format,
    image::{view::ImageView, AttachmentImage, SwapchainImage},
    render_pass::{Framebuffer, FramebufferAbstract, RenderPass},
    swapchain::Swapchain,
};

use winit::window::Window;

pub fn get_frame_buffer(
    swapchain: &Swapchain<Window>,
    device: &Arc<Device>,
    images: &[Arc<SwapchainImage<Window>>],
    render_pass: &Arc<RenderPass>,
) -> Vec<Arc<dyn FramebufferAbstract + Send + Sync>> {
    let depth_buffer = ImageView::new(
        AttachmentImage::transient(device.clone(), swapchain.dimensions(), Format::D16Unorm)
            .unwrap(),
    )
    .unwrap();
    images
        .iter()
        .map(|image| {
            let view = ImageView::new(image.clone()).unwrap();
            Arc::new(
                Framebuffer::start(render_pass.clone())
                    .add(view)
                    .unwrap()
                    .add(depth_buffer.clone())
                    .unwrap()
                    .build()
                    .unwrap(),
            ) as Arc<dyn FramebufferAbstract + Send + Sync>
        })
        .collect::<Vec<_>>()
}
