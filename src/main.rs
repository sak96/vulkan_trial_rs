use std::sync::Arc;

use vulkano::buffer::CpuAccessibleBuffer;
use vulkano::device::{Device, Queue};
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
mod instance;
mod pipeline;
mod renderpass;
mod shaders;
mod swapchains;
mod vertex;
mod window;

struct Hex {
    event_loop: EventLoop<()>,
    device: Arc<Device>,
    graphical_queue: Arc<Queue>,
    _debug_callback: Option<DebugCallback>,
    present_queue: Arc<Queue>,
    swapchain: Arc<Swapchain<Window>>,
    #[allow(dead_code)]
    surface: Arc<Surface<Window>>,
    #[allow(dead_code)]
    images: Vec<Arc<SwapchainImage<Window>>>,
    #[allow(dead_code)]
    renderpass: Arc<RenderPass>,
    #[allow(dead_code)]
    vertex_buffer: Arc<CpuAccessibleBuffer<[crate::vertex::Vertex]>>,
    #[allow(dead_code)]
    pipeline: Arc<crate::pipeline::ConcreteGraphicsPipeline>,
    #[allow(dead_code)]
    framebuffers: Vec<Arc<dyn FramebufferAbstract + Send + Sync>>,
    resizehelper: dynamicstate::ResizeHelper,
}

impl Hex {
    pub fn new() -> Self {
        let instance = crate::instance::create_instance();
        let _debug_callback = crate::instance::setup_debug_callback(&instance);
        let (event_loop, surface) = crate::window::init_window(&instance);
        let (device, graphical_queue, present_queue) =
            crate::device::create_logical_device(&instance, &surface);
        let (swapchain, images) =
            crate::swapchains::get_swapchain(&surface, &device, &graphical_queue, &present_queue);
        let vertex_buffer = crate::vertex::Vertex::get_buffer(&device);
        let renderpass = crate::renderpass::get_render_pass(&device, &swapchain);
        let pipeline = crate::pipeline::get_pipeline(&device, &renderpass);
        let framebuffers = crate::framebuffers::get_frame_buffer(&images, &renderpass);
        let resizehelper = crate::dynamicstate::ResizeHelper::new(&swapchain);
        Self {
            event_loop,
            device,
            surface,
            graphical_queue,
            present_queue,
            swapchain,
            images,
            _debug_callback,
            renderpass,
            vertex_buffer,
            pipeline,
            framebuffers,
            resizehelper,
        }
    }

    pub fn run(self) {
        let mut offset = 0.0;
        let mut recreate_swapchain = true;
        let Self {
            device,
            event_loop,
            graphical_queue,
            present_queue,
            vertex_buffer,
            pipeline,
            surface,
            renderpass,
            mut swapchain,
            mut framebuffers,
            mut images,
            mut resizehelper,
            ..
        } = self;
        let mut previous_frame_end = Some(sync::now(device.clone()).boxed());
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
                let push_constants: Vec<_> = (0..3)
                    .map(|i| crate::shaders::vs::ty::PushConstantData {
                        offset: [0.0 + offset, -0.4 + i as f32 * 0.25],
                        color: [0.0, 0.0, 0.2 + 0.2 * i as f32, 1.0],
                    })
                    .collect();
                offset = (offset + 1.52) % 2.5 - 1.5;

                let cmd_buffer = crate::commandbuffers::get_command_buffers(
                    &pipeline,
                    &graphical_queue,
                    &device,
                    &frame,
                    &vertex_buffer,
                    &resizehelper,
                    &push_constants,
                );
                let future = previous_frame_end
                    .take()
                    .unwrap()
                    .join(acquire_future)
                    .then_execute(graphical_queue.clone(), cmd_buffer)
                    .unwrap()
                    .then_swapchain_present(present_queue.clone(), swapchain.clone(), image_num)
                    .then_signal_fence_and_flush();

                match future {
                    Ok(future) => {
                        previous_frame_end = Some(future.boxed());
                    }
                    Err(FlushError::OutOfDate) => {
                        recreate_swapchain = true;
                        previous_frame_end = Some(sync::now(device.clone()).boxed());
                    }
                    Err(e) => {
                        println!("Failed to flush future: {:?}", e);
                        previous_frame_end = Some(sync::now(device.clone()).boxed());
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
