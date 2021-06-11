use std::sync::Arc;

use vulkano::{
    command_buffer::{AutoCommandBufferBuilder, DynamicState, PrimaryAutoCommandBuffer},
    device::Device,
    render_pass::RenderPass,
};

mod pipeline;

pub struct Pipeline {
    pipeline: Arc<pipeline::ConcreteGraphicsPipeline>,
}

impl Pipeline {
    pub fn new(device: &Arc<Device>, renderpass: &Arc<RenderPass>) -> Self {
        let pipeline = pipeline::get_pipeline(&device, &renderpass);
        Self { pipeline }
    }

    pub fn render_game_objects(
        &self,
        cmd_builder: &mut AutoCommandBufferBuilder<PrimaryAutoCommandBuffer>,
        game_objs: &mut [crate::model::GameObject],
        dynamicstate: &DynamicState,
    ) {
        for objs in game_objs.iter_mut() {
            objs.rotate[0] = objs.rotate[0] + 0.01;
            objs.rotate[1] = objs.rotate[1] + 0.01;
            objs.rotate[2] = objs.rotate[2] + 0.01;
            cmd_builder
                .draw(
                    self.pipeline.clone(),
                    &dynamicstate,
                    objs.vertex_buffer.clone(),
                    (),
                    objs.get_push_data(),
                    vec![],
                )
                .unwrap();
        }
    }
}
