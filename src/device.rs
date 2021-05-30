use std::sync::Arc;

use vulkano::device::{Device, DeviceExtensions, Features, Queue};
use vulkano::instance::{Instance, PhysicalDevice};
use vulkano::swapchain::Surface;

use winit::window::Window;

pub fn create_logical_device(
    instance: &Arc<Instance>,
    surface: &Arc<Surface<Window>>,
) -> (Arc<Device>, Arc<Queue>, Arc<Queue>) {
    let physical = create_phy_device(&instance);
    let mut graphical_queue_family = None;
    let mut present_queue_family = None;
    for queue_family in physical.queue_families() {
        if graphical_queue_family.is_none() && queue_family.supports_graphics() {
            graphical_queue_family = Some(queue_family.clone());
        }
        if present_queue_family.is_none() && surface.is_supported(queue_family).unwrap_or(false) {
            present_queue_family = Some(queue_family.clone());
        }
    }
    let graphical_queue_family =
        graphical_queue_family.expect("could not find graphical queue family");
    let present_queue_family = present_queue_family.expect("could not find present queue family");
    let mut queue_families = vec![graphical_queue_family];
    if present_queue_family != graphical_queue_family {
        queue_families.push(present_queue_family);
    }

    let (device, mut queues) = {
        Device::new(
            physical,
            &Features {
                // NOTE: this is to allow non filled triangle (swapchain)
                fill_mode_non_solid: true,
                ..Features::none()
            },
            &DeviceExtensions {
                // NOTE: this is to allow swapchain
                khr_swapchain: true,
                ..DeviceExtensions::none()
            },
            // TODO: figure out the priority
            queue_families.into_iter().map(|p| (p, 1.0)),
        )
        .expect("failed to create device")
    };

    let graphical_queue = queues.next().expect("did not get any queue");
    let present_queue = queues.next().unwrap_or(graphical_queue.clone());
    (device, graphical_queue, present_queue)
}

fn create_phy_device<'a>(instance: &'a Arc<Instance>) -> PhysicalDevice<'a> {
    let physical = PhysicalDevice::enumerate(instance)
        .next()
        .expect("no device available");
    println!("{}|{:?}|", physical.name(), physical.ty());
    physical
}
