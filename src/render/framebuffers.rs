use std::sync::Arc;

use vulkano::{
    image::{view::ImageView, SwapchainImage},
    render_pass::{Framebuffer, FramebufferAbstract, RenderPass},
};

use winit::window::Window;

pub fn get_frame_buffer(
    images: &[Arc<SwapchainImage<Window>>],
    render_pass: &Arc<RenderPass>,
) -> Vec<Arc<dyn FramebufferAbstract + Send + Sync>> {
    images
        .iter()
        .map(|image| {
            let view = ImageView::new(image.clone()).unwrap();
            Arc::new(
                Framebuffer::start(render_pass.clone())
                    .add(view)
                    .unwrap()
                    .build()
                    .unwrap(),
            ) as Arc<dyn FramebufferAbstract + Send + Sync>
        })
        .collect::<Vec<_>>()
}
