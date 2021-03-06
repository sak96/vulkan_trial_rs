use std::sync::Arc;

use vulkano::device::Device;
use vulkano::format::Format;
use vulkano::render_pass::RenderPass;
use vulkano::swapchain::Swapchain;

use winit::window::Window;

pub fn get_render_pass(
    device: &Arc<Device>,
    swapchain: &Arc<Swapchain<Window>>,
) -> Arc<RenderPass> {
    let render_pass = Arc::new(
        vulkano::single_pass_renderpass!(
            device.clone(),
            attachments: {
                color: {
                    load: Clear,
                    store: Store,
                    format: swapchain.format(),
                    samples: 1,
                },
                depth: {
                    load: Clear,
                    store: DontCare,
                    format: Format::D16Unorm,
                    samples: 1,
                }
            },
            pass: {
                color: [color],
                depth_stencil: {depth}
            }
        )
        .unwrap(),
    );
    render_pass
}
