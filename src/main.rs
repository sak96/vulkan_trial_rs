use std::sync::Arc;

use vulkano::image::SwapchainImage;
use vulkano::instance::debug::DebugCallback;
use vulkano::render_pass::{FramebufferAbstract, RenderPass};
use vulkano::swapchain::{self, AcquireError, Surface, Swapchain};
use vulkano::sync::{self, FlushError, GpuFuture};

use winit::event::{Event, WindowEvent};
use winit::event_loop::{ControlFlow, EventLoop};
use winit::window::Window;

mod commandbuffers;
mod device;
mod dynamicstate;
mod framebuffers;
mod game;
mod instance;
mod pipeline;
mod renderpass;
mod shaders;
mod swapchains;
mod vertex;
mod window;

struct Hex {
    event_loop: EventLoop<()>,
    logical_device: crate::device::LogicalDevice,
    _debug_callback: Option<DebugCallback>,
    swapchain: Arc<Swapchain<Window>>,
    #[allow(dead_code)]
    surface: Arc<Surface<Window>>,
    #[allow(dead_code)]
    images: Vec<Arc<SwapchainImage<Window>>>,
    #[allow(dead_code)]
    renderpass: Arc<RenderPass>,
    #[allow(dead_code)]
    serpenskis: Vec<crate::game::Serpenskis>,
    #[allow(dead_code)]
    pipeline: Arc<crate::pipeline::ConcreteGraphicsPipeline>,
    #[allow(dead_code)]
    framebuffers: Vec<Arc<dyn FramebufferAbstract + Send + Sync>>,
    resizehelper: dynamicstate::ResizeHelper,
}

impl Hex {
    pub fn new() -> Self {
        println!(
            "{}",
            std::mem::size_of::<crate::shaders::vs::ty::PushConstantData>()
        );
        let instance = crate::instance::create_instance();
        let _debug_callback = crate::instance::setup_debug_callback(&instance);
        let (event_loop, surface) = crate::window::init_window(&instance);
        let logical_device = crate::device::LogicalDevice::create_logical_device(&instance, &surface);
        let (swapchain, images) =
            crate::swapchains::get_swapchain(&surface, &logical_device);
        let serpenskis = vec![
            crate::game::Serpenskis::new(
                &logical_device.device,
                [0.0, 1.0, 0.0, 1.0],
                [0.0, 0.0],
                [1.0, 1.0],
                [0.0],
            ),
            crate::game::Serpenskis::new(
                &logical_device.device,
                [0.0, 1.0, 1.0, 1.0],
                [0.0, 1.0],
                [0.5, 1.0],
                [std::f32::consts::PI],
            ),
            crate::game::Serpenskis::new(
                &logical_device.device,
                [1.0, 1.0, 0.0, 1.0],
                [0.0, 1.0],
                [0.5, 0.5],
                [2.0 * std::f32::consts::PI],
            ),
        ];
        let renderpass = crate::renderpass::get_render_pass(&logical_device.device, &swapchain);
        let pipeline = crate::pipeline::get_pipeline(&logical_device.device, &renderpass);
        let framebuffers = crate::framebuffers::get_frame_buffer(&images, &renderpass);
        let resizehelper = crate::dynamicstate::ResizeHelper::new(&swapchain);
        Self {
            event_loop,
            logical_device,
            surface,
            swapchain,
            images,
            _debug_callback,
            renderpass,
            serpenskis,
            pipeline,
            framebuffers,
            resizehelper,
        }
    }

    pub fn run(self) {
        let mut offset = 0.0;
        let mut recreate_swapchain = true;
        let Self {
            logical_device,
            event_loop,
            serpenskis,
            pipeline,
            surface,
            renderpass,
            mut swapchain,
            mut framebuffers,
            mut images,
            mut resizehelper,
            ..
        } = self;
        let mut previous_frame_end = Some(sync::now(logical_device.device.clone()).boxed());
        event_loop.run(move |event, _, control_flow| match event {
            Event::WindowEvent {
                event: WindowEvent::CloseRequested,
                ..
            } => {
                *control_flow = ControlFlow::Exit;
            }
            Event::WindowEvent {
                event: WindowEvent::Resized(_),
                ..
            } => {
                recreate_swapchain = true;
            }
            Event::RedrawEventsCleared => {
                previous_frame_end.as_mut().unwrap().cleanup_finished();
                if recreate_swapchain {
                    recreate_swapchain = resizehelper.resize(
                        &surface,
                        &renderpass,
                        &mut swapchain,
                        &mut framebuffers,
                        &mut images,
                    );
                }
                let (image_num, suboptimal, acquire_future) =
                    match swapchain::acquire_next_image(swapchain.clone(), None) {
                        Ok(r) => r,
                        Err(AcquireError::OutOfDate) => {
                            recreate_swapchain = true;
                            return;
                        }
                        Err(e) => panic!("failed due to {:?}", e),
                    };
                if suboptimal {
                    recreate_swapchain = true;
                }
                let frame = framebuffers[image_num].clone();
                offset = (offset + 1.52) % 2.5 - 1.5;

                let cmd_buffer = crate::commandbuffers::get_command_buffers(
                    &pipeline,
                    &logical_device.graphical_queue,
                    &logical_device.device,
                    &frame,
                    &resizehelper,
                    &serpenskis.as_slice(),
                );
                let future = previous_frame_end
                    .take()
                    .unwrap()
                    .join(acquire_future)
                    .then_execute(logical_device.graphical_queue.clone(), cmd_buffer)
                    .unwrap()
                    .then_swapchain_present(logical_device.present_queue.clone(), swapchain.clone(), image_num)
                    .then_signal_fence_and_flush();

                match future {
                    Ok(future) => {
                        previous_frame_end = Some(future.boxed());
                    }
                    Err(FlushError::OutOfDate) => {
                        recreate_swapchain = true;
                        previous_frame_end = Some(sync::now(logical_device.device.clone()).boxed());
                    }
                    Err(e) => {
                        println!("Failed to flush future: {:?}", e);
                        previous_frame_end = Some(sync::now(logical_device.device.clone()).boxed());
                    }
                }
            }
            _ => {}
        });
    }
}

fn main() {
    Hex::new().run()
}
