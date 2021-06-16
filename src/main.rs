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
    game_objects: Vec<crate::model::GameObject>,
    render: crate::render::Render,
}

impl Hex {
    pub fn new() -> Self {
        let instance = crate::instance::create_instance();
        let (event_loop, surface) = crate::window::init_window(&instance);
        let logical_device =
            crate::device::LogicalDevice::create_logical_device(&instance, &surface);
        let render = crate::render::Render::new(&logical_device, &surface);
        let game_objects = vec![
            crate::model::GameObject::new(
                &logical_device.device,
                [0.0, 0.0, 0.5],
                [0.1, 0.5, 0.1],
                [0.0, 0.0, 0.0],
            ),
            crate::model::GameObject::new(
                &logical_device.device,
                [0.0, -10.0, 10.0],
                [0.5, 1.0, 0.5],
                [std::f32::consts::PI, 0.0, 0.0],
            ),
            crate::model::GameObject::new(
                &logical_device.device,
                [5.0, 10.0, 15.0],
                [0.5, 0.5, 0.5],
                [2.0 * std::f32::consts::PI, 0.0, 0.0],
            ),
        ];
        Self {
            event_loop,
            logical_device,
            game_objects,
            render,
        }
    }

    pub fn run(self) {
        let Self {
            logical_device,
            event_loop,
            mut game_objects,
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
                let aspect = render.get_aspect_ratio();
                let camera = glam::Mat4::perspective_lh(
                    std::f32::consts::PI * (50.0 / 90.0),
                    aspect,
                    0.1,
                    20.0,
                );
                // let camera = glam::Mat4::orthographic_lh(
                //     aspect * 20.0,
                //     -aspect * 20.0,
                //     20.0,
                //     -20.0,
                //     20.0,
                //     0.1,
                // );
                // let camera = glam::Mat4::IDENTITY;

                if let Some(mut cmd_builder) =
                    render.get_command_buffer_builder(logical_device.graphical_queue.clone())
                {
                    simple_display.render_game_objects(
                        &mut cmd_builder,
                        &mut game_objects,
                        &render.inner(),
                        &camera,
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
