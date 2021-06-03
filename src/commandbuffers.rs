use std::sync::Arc;

use vulkano::{
    buffer::CpuAccessibleBuffer,
    command_buffer::{AutoCommandBufferBuilder, PrimaryAutoCommandBuffer, SubpassContents},
    device::{Device, Queue},
    render_pass::FramebufferAbstract,
};

use crate::{dynamicstate::ResizeHelper, pipeline::ConcreteGraphicsPipeline, vertex::Vertex};

pub fn get_command_buffers(
    pipeline: &Arc<ConcreteGraphicsPipeline>,
    graphical_queue: &Arc<Queue>,
    device: &Arc<Device>,
    framebuffers: &Vec<Arc<dyn FramebufferAbstract + Send + Sync>>,
    vertex_buffer: &Arc<CpuAccessibleBuffer<[Vertex]>>,
    resizehelper: &ResizeHelper,
) -> Vec<Arc<PrimaryAutoCommandBuffer>> {
    let clear_values = vec![[0.0, 0.0, 0.0, 1.0].into()];
    framebuffers
        .iter()
        .map(|framebuffer| {
            let mut cmd_builder = AutoCommandBufferBuilder::primary(
                device.clone(),
                graphical_queue.family(),
                vulkano::command_buffer::CommandBufferUsage::SimultaneousUse,
            )
            .unwrap();
            cmd_builder
                .begin_render_pass(
                    framebuffer.clone(),
                    SubpassContents::Inline,
                    clear_values.clone(),
                )
                .unwrap()
                .draw(
                    pipeline.clone(),
                    &resizehelper.inner(),
                    vertex_buffer.clone(),
                    (),
                    (),
                    vec![],
                )
                .unwrap()
                .end_render_pass()
                .unwrap();
            Arc::new(cmd_builder.build().unwrap())
        })
        .collect()
}
