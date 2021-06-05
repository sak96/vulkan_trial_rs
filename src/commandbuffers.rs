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
    framebuffer: &Arc<dyn FramebufferAbstract + Send + Sync>,
    vertex_buffer: &Arc<CpuAccessibleBuffer<[Vertex]>>,
    resizehelper: &ResizeHelper,
    push_constants: &[crate::shaders::vs::ty::PushConstantData],
) -> Arc<PrimaryAutoCommandBuffer> {
    // let push_constants: Vec<_> = (0..3)
    //     .map(|i| crate::shaders::vs::ty::PushConstantData {
    //         offset: [0.0, -0.4 + i as f32 * 0.25],
    //         color: [0.2 + 0.2 * i as f32, 0.0, 0.0, 1.0],
    //     })
    //     .collect();
    // let push_constants = vec![
    //     crate::shaders::vs::ty::PushConstantData {
    //         color: [0.0, 0.0, 0.70],
    //         _dummy0: [0; 4],
    //         offset: [-1.0, 0.0],
    //     },
    //     crate::shaders::vs::ty::PushConstantData {
    //         color: [0.0, 0.0, 0.20],
    //         _dummy0: [0; 4],
    //         offset: [0.05, 0.0],
    //     },
    //     crate::shaders::vs::ty::PushConstantData {
    //         color: [0.0, 0.0, 0.50],
    //         _dummy0: [0; 4],
    //         offset: [0.05, 0.0],
    //     },
    // ];
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
    for push_constant in push_constants {
        cmd_builder
            .draw(
                pipeline.clone(),
                &resizehelper.inner(),
                vertex_buffer.clone(),
                (),
                push_constant.clone(),
                vec![],
            )
            .unwrap();
    }
    cmd_builder.end_render_pass().unwrap();
    Arc::new(cmd_builder.build().unwrap())
}
