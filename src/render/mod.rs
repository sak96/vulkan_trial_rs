use std::sync::Arc;

use vulkano::{
    command_buffer::{
        AutoCommandBufferBuilder, DynamicState, PrimaryAutoCommandBuffer, SubpassContents,
    },
    device::{Device, Queue},
    image::SwapchainImage,
    render_pass::{FramebufferAbstract, RenderPass},
    swapchain::{self, AcquireError, Surface, Swapchain, SwapchainAcquireFuture},
    sync::{self, FlushError, GpuFuture},
};

use winit::window::Window;

use crate::device::LogicalDevice;

mod dynamicstate;
mod framebuffers;
mod renderpass;
mod swapchains;

pub struct Render {
    device: Arc<Device>,
    swapchain: Arc<Swapchain<Window>>,
    resizehelper: dynamicstate::ResizeHelper,
    surface: Arc<Surface<Window>>,
    images: Vec<Arc<SwapchainImage<Window>>>,
    pub renderpass: Arc<RenderPass>,
    acquire_future: Option<SwapchainAcquireFuture<Window>>,
    framebuffers: Vec<Arc<dyn FramebufferAbstract + Send + Sync>>,
    recreate_swapchain: bool,
    current_image_index: usize,
    previous_frame_end: Option<Box<dyn GpuFuture>>,
}

impl Render {
    pub fn recreate_swapchain(&mut self) {
        self.recreate_swapchain = true
    }
    pub fn new(logical_device: &LogicalDevice, surface: &Arc<Surface<Window>>) -> Self {
        let previous_frame_end = Some(sync::now(logical_device.device.clone()).boxed());
        let (swapchain, images) = swapchains::get_swapchain(&surface, &logical_device);
        let resizehelper = dynamicstate::ResizeHelper::new(&swapchain);
        let renderpass = renderpass::get_render_pass(&logical_device.device, &swapchain);
        let framebuffers = framebuffers::get_frame_buffer(&images, &renderpass);
        Self {
            device: logical_device.device.clone(),
            surface: surface.clone(),
            swapchain,
            images,
            renderpass,
            framebuffers,
            previous_frame_end,
            resizehelper,
            acquire_future: Default::default(),
            recreate_swapchain: Default::default(),
            current_image_index: Default::default(),
        }
    }

    pub fn get_command_buffer_builder(
        &mut self,
        graphical_queue: Arc<Queue>,
    ) -> Option<AutoCommandBufferBuilder<PrimaryAutoCommandBuffer>> {
        self.previous_frame_end.as_mut().unwrap().cleanup_finished();
        if self.recreate_swapchain {
            self.recreate_swapchain = self.resizehelper.resize(
                &self.surface,
                &self.renderpass,
                &mut self.swapchain,
                &mut self.framebuffers,
                &mut self.images,
            );
        }
        let (image_num, suboptimal, acquire_future) =
            match swapchain::acquire_next_image(self.swapchain.clone(), None) {
                Ok(r) => r,
                Err(AcquireError::OutOfDate) => {
                    self.recreate_swapchain = true;
                    return None;
                }
                Err(e) => panic!("failed due to {:?}", e),
            };
        self.acquire_future = Some(acquire_future);
        self.current_image_index = image_num;
        if suboptimal {
            self.recreate_swapchain = true;
        }
        let frame = self.framebuffers[image_num].clone();
        let clear_values = vec![[0.0, 0.0, 0.0, 1.0].into()];
        let mut cmd_builder = AutoCommandBufferBuilder::primary(
            self.device.clone(),
            graphical_queue.family(),
            vulkano::command_buffer::CommandBufferUsage::OneTimeSubmit,
        )
        .unwrap();
        cmd_builder
            .begin_render_pass(frame.clone(), SubpassContents::Inline, clear_values.clone())
            .unwrap();
        Some(cmd_builder)
    }
    pub fn render(
        &mut self,
        mut cmd_builder: AutoCommandBufferBuilder<PrimaryAutoCommandBuffer>,
        graphical_queue: &Arc<Queue>,
        present_queue: &Arc<Queue>,
    ) {
        if cmd_builder.end_render_pass().is_err() {
            eprintln!("render pass already ended");
        }
        let command_buffer = Arc::new(cmd_builder.build().unwrap());
        let future = self
            .previous_frame_end
            .take()
            .unwrap()
            .join(
                self.acquire_future
                    .take()
                    .expect("expected acquire_future to be present"),
            )
            .then_execute(graphical_queue.clone(), command_buffer)
            .unwrap()
            .then_swapchain_present(
                present_queue.clone(),
                self.swapchain.clone(),
                self.current_image_index,
            )
            .then_signal_fence_and_flush();

        match future {
            Ok(future) => {
                self.previous_frame_end = Some(future.boxed());
            }
            Err(FlushError::OutOfDate) => {
                self.recreate_swapchain = true;
                self.previous_frame_end = Some(sync::now(self.device.clone()).boxed());
            }
            Err(e) => {
                println!("Failed to flush future: {:?}", e);
                self.previous_frame_end = Some(sync::now(self.device.clone()).boxed());
            }
        }
    }

    pub fn inner(&self) -> &DynamicState {
        &self.resizehelper.inner()
    }
}
