use std::sync::Arc;

use winit::event::{Event, WindowEvent};
use winit::event_loop::{ControlFlow, EventLoop};

mod device;
mod dynamicstate;
mod framebuffers;
mod game;
mod instance;
mod pipeline;
mod render;
mod renderpass;
mod shaders;
mod swapchains;
mod vertex;
mod window;

struct Hex {
    event_loop: EventLoop<()>,
    logical_device: crate::device::LogicalDevice,
    serpenskis: Vec<crate::game::Serpenskis>,
    pipeline: Arc<crate::pipeline::ConcreteGraphicsPipeline>,
    render: crate::render::Render,
}

impl Hex {
    pub fn new() -> Self {
        println!(
            "{}",
            std::mem::size_of::<crate::shaders::vs::ty::PushConstantData>()
        );
        let instance = crate::instance::create_instance();
        let (event_loop, surface) = crate::window::init_window(&instance);
        let logical_device =
            crate::device::LogicalDevice::create_logical_device(&instance, &surface);
        let render = crate::render::Render::new(&logical_device, &surface);
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
        let pipeline = crate::pipeline::get_pipeline(&logical_device.device, &render.renderpass);
        Self {
            event_loop,
            logical_device,
            serpenskis,
            pipeline,
            render,
        }
    }

    pub fn run(self) {
        let Self {
            logical_device,
            event_loop,
            serpenskis,
            pipeline,
            mut render,
            ..
        } = self;
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
            } => render.recreate_swapchain(),
            Event::RedrawEventsCleared => {
                if let Some(mut cmd_builder) =
                    render.get_command_buffer_builder(logical_device.graphical_queue.clone())
                {
                    for serpenski in serpenskis.iter() {
                        cmd_builder
                            .draw(
                                pipeline.clone(),
                                &render.inner(),
                                serpenski.vertex_buffer.clone(),
                                (),
                                serpenski.get_push_data(),
                                vec![],
                            )
                            .unwrap();
                    }
                    render.render(
                        cmd_builder,
                        &logical_device.graphical_queue.clone(),
                        &logical_device.present_queue.clone(),
                    );
                }
            }
            _ => {}
        });
    }
}

fn main() {
    Hex::new().run()
}
