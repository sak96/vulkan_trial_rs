use std::sync::Arc;

use vulkano::{
    command_buffer::{AutoCommandBufferBuilder, PrimaryAutoCommandBuffer, SubpassContents},
    device::{Device, Queue},
    render_pass::FramebufferAbstract,
};

use crate::{dynamicstate::ResizeHelper, pipeline::ConcreteGraphicsPipeline};

pub fn get_command_buffers(
    pipeline: &Arc<ConcreteGraphicsPipeline>,
    graphical_queue: &Arc<Queue>,
    device: &Arc<Device>,
    framebuffer: &Arc<dyn FramebufferAbstract + Send + Sync>,
    resizehelper: &ResizeHelper,
    serpenskis: &[crate::game::Serpenskis],
) -> Arc<PrimaryAutoCommandBuffer> {
    let clear_values = vec![[0.5, 0.5, 0.5, 1.0].into()];
    let mut cmd_builder = AutoCommandBufferBuilder::primary(
        device.clone(),
        graphical_queue.family(),
        vulkano::command_buffer::CommandBufferUsage::OneTimeSubmit,
    )
    .unwrap();
    cmd_builder
        .begin_render_pass(
            framebuffer.clone(),
            SubpassContents::Inline,
            clear_values.clone(),
        )
        .unwrap();
    for serpenski in serpenskis {
        cmd_builder
            .draw(
                pipeline.clone(),
                &resizehelper.inner(),
                serpenski.vertex_buffer.clone(),
                (),
                serpenski.get_push_data(),
                vec![],
            )
            .unwrap();
    }
    cmd_builder.end_render_pass().unwrap();
    Arc::new(cmd_builder.build().unwrap())
}
