use winit::event::{Event, WindowEvent};
use winit::event_loop::{ControlFlow, EventLoop};

mod device;
mod instance;
mod model;
mod render;
mod shaders;
mod simple_display;
mod window;

struct Hex {
    event_loop: EventLoop<()>,
    logical_device: crate::device::LogicalDevice,
    serpenskis: Vec<crate::model::GameObject>,
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
            crate::model::GameObject::new(
                &logical_device.device,
                [0.0, 0.0, 0.5],
                [0.1, 0.5, 0.1],
                [0.0, 0.0, 0.0],
            ),
            crate::model::GameObject::new(
                &logical_device.device,
                [0.0, -1.0, 0.0],
                [0.5, 1.0, 0.0],
                [std::f32::consts::PI, 0.0, 0.0],
            ),
            crate::model::GameObject::new(
                &logical_device.device,
                [0.0, 1.0, 0.0],
                [0.5, 0.5, 0.0],
                [2.0 * std::f32::consts::PI, 0.0, 0.0],
            ),
        ];
        Self {
            event_loop,
            logical_device,
            serpenskis,
            render,
        }
    }

    pub fn run(self) {
        let Self {
            logical_device,
            event_loop,
            serpenskis,
            mut render,
            ..
        } = self;
        let simple_display =
            simple_display::Pipeline::new(&logical_device.device, &render.renderpass);
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
                    simple_display.render_game_objects(
                        &mut cmd_builder,
                        &serpenskis,
                        &render.inner(),
                    );
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
