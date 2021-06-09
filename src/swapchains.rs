use std::sync::Arc;

use vulkano::image::{ImageUsage, SwapchainImage};
use vulkano::swapchain::{Surface, Swapchain};
use vulkano::sync::SharingMode;

use winit::window::Window;

use crate::device::LogicalDevice;

pub fn get_swapchain(
    surface: &Arc<Surface<Window>>,
    logical_device: &LogicalDevice,
) -> (Arc<Swapchain<Window>>, Vec<Arc<SwapchainImage<Window>>>) {
    let caps = surface
        .capabilities(logical_device.device.physical_device())
        .unwrap();
    let graphical_queue = &logical_device.graphical_queue.clone();
    let present_queue = &logical_device.present_queue.clone();

    let alpha = caps.supported_composite_alpha.iter().next().unwrap();
    let format = caps.supported_formats[0].0;
    let dim = surface.window().inner_size().into();

    let sharing: SharingMode = if graphical_queue != present_queue {
        vec![graphical_queue, present_queue].as_slice().into()
    } else {
        graphical_queue.into()
    };

    Swapchain::start(logical_device.device.clone(), surface.clone())
        .num_images(caps.min_image_count)
        .format(format)
        .dimensions(dim)
        .usage(ImageUsage::color_attachment())
        .sharing_mode(sharing)
        .composite_alpha(alpha)
        .build()
        .expect("failed to create swapchain")
}
