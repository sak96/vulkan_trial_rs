use std::sync::Arc;

use vulkano::descriptor::PipelineLayoutAbstract;
use vulkano::device::Device;
use vulkano::pipeline::{vertex::SingleBufferDefinition, GraphicsPipeline};
use vulkano::render_pass::{RenderPass, Subpass};

use crate::shaders::{fs, vs};
use crate::model::Vertex;

pub type ConcreteGraphicsPipeline = GraphicsPipeline<
    SingleBufferDefinition<Vertex>,
    Box<dyn PipelineLayoutAbstract + Send + Sync + 'static>,
>;
pub fn get_pipeline(
    device: &Arc<Device>,
    renderpass: &Arc<RenderPass>,
) -> Arc<ConcreteGraphicsPipeline> {
    let vs = vs::Shader::load(device.clone()).unwrap();
    let fs = fs::Shader::load(device.clone()).unwrap();

    let pipeline = Arc::new(
        GraphicsPipeline::start()
            .vertex_input_single_buffer()
            .vertex_shader(vs.main_entry_point(), ())
            // .polygon_mode_line()
            .triangle_list()
            .viewports_dynamic_scissors_irrelevant(1)
            .fragment_shader(fs.main_entry_point(), ())
            .depth_stencil_simple_depth()
            .render_pass(Subpass::from(renderpass.clone(), 0).unwrap())
            .build(device.clone())
            .unwrap(),
    );
    pipeline
}
