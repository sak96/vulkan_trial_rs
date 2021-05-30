use std::sync::Arc;

use vulkano::descriptor::PipelineLayoutAbstract;
use vulkano::device::Device;
use vulkano::pipeline::viewport::Viewport;
use vulkano::pipeline::{vertex::SingleBufferDefinition, GraphicsPipeline};
use vulkano::render_pass::{RenderPass, Subpass};
use vulkano::swapchain::Swapchain;
use winit::window::Window;

use crate::shaders::{fs, vs};
use crate::vertex::Vertex;

pub type ConcreteGraphicsPipeline = GraphicsPipeline<
    SingleBufferDefinition<Vertex>,
    Box<dyn PipelineLayoutAbstract + Send + Sync + 'static>,
>;
pub fn get_pipeline(
    device: &Arc<Device>,
    renderpass: &Arc<RenderPass>,
    swapchain: &Arc<Swapchain<Window>>,
) -> Arc<ConcreteGraphicsPipeline> {
    let vs = vs::Shader::load(device.clone()).unwrap();
    let fs = fs::Shader::load(device.clone()).unwrap();

    let dim = swapchain.dimensions();
    let dimensions = [dim[0] as f32, dim[1] as f32];

    let pipeline = Arc::new(
        GraphicsPipeline::start()
            .vertex_input_single_buffer()
            .vertex_shader(vs.main_entry_point(), ())
            .polygon_mode_line()
            .triangle_list()
            // .viewports_dynamic_scissors_irrelevant(1)
            .viewports(vec![Viewport {
                origin: [0.0, 0.0],
                dimensions,
                depth_range: 0.0..1.0,
            }])
            .fragment_shader(fs.main_entry_point(), ())
            .render_pass(Subpass::from(renderpass.clone(), 0).unwrap())
            .build(device.clone())
            .unwrap(),
    );
    pipeline
}
