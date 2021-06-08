use std::sync::Arc;

use vulkano::instance::Instance;
use vulkano::swapchain::Surface;
use vulkano_win::VkSurfaceBuild;

use winit::event_loop::EventLoop;
use winit::window::Window;
use winit::window::WindowBuilder;

pub fn init_window(instance: &Arc<Instance>) -> (EventLoop<()>, Arc<Surface<Window>>) {
    let event_loop = EventLoop::new();
    let surface = WindowBuilder::new()
        .with_title(env!("CARGO_PKG_NAME"))
        .build_vk_surface(&event_loop, instance.clone())
        .expect("failed to crate window builder");
    (event_loop, surface)
}
