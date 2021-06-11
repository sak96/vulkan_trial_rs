use std::sync::Arc;

use vulkano::{
    buffer::{BufferUsage, CpuAccessibleBuffer},
    device::Device,
};

#[derive(Default, Clone)]
pub struct Vertex {
    position: [f32; 3],
    color: [f32; 3],
}

vulkano::impl_vertex!(Vertex, position, color);
impl Vertex {
    fn get_vertex() -> Vec<Vertex> {
        //   o
        //  wgy
        //   r
        //   b
        vec![
            // left face (white)
            Vertex {
                position: [-0.5, -0.5, -0.5],
                color: [0.9, 0.9, 0.9],
            },
            Vertex {
                position: [-0.5, 0.5, 0.5],
                color: [0.9, 0.9, 0.9],
            },
            Vertex {
                position: [-0.5, -0.5, 0.5],
                color: [0.9, 0.9, 0.9],
            },
            Vertex {
                position: [-0.5, -0.5, -0.5],
                color: [0.9, 0.9, 0.9],
            },
            Vertex {
                position: [-0.5, 0.5, -0.5],
                color: [0.9, 0.9, 0.9],
            },
            Vertex {
                position: [-0.5, 0.5, 0.5],
                color: [0.9, 0.9, 0.9],
            },
            // right face (yellow)
            Vertex {
                position: [0.5, -0.5, -0.5],
                color: [0.8, 0.8, 0.1],
            },
            Vertex {
                position: [0.5, 0.5, 0.5],
                color: [0.8, 0.8, 0.1],
            },
            Vertex {
                position: [0.5, -0.5, 0.5],
                color: [0.8, 0.8, 0.1],
            },
            Vertex {
                position: [0.5, -0.5, -0.5],
                color: [0.8, 0.8, 0.1],
            },
            Vertex {
                position: [0.5, 0.5, -0.5],
                color: [0.8, 0.8, 0.1],
            },
            Vertex {
                position: [0.5, 0.5, 0.5],
                color: [0.8, 0.8, 0.1],
            },
            // top face (orange)
            Vertex {
                position: [-0.5, -0.5, -0.5],
                color: [0.9, 0.6, 0.1],
            },
            Vertex {
                position: [0.5, -0.5, 0.5],
                color: [0.9, 0.6, 0.1],
            },
            Vertex {
                position: [-0.5, -0.5, 0.5],
                color: [0.9, 0.6, 0.1],
            },
            Vertex {
                position: [-0.5, -0.5, -0.5],
                color: [0.9, 0.6, 0.1],
            },
            Vertex {
                position: [0.5, -0.5, -0.5],
                color: [0.9, 0.6, 0.1],
            },
            Vertex {
                position: [0.5, -0.5, 0.5],
                color: [0.9, 0.6, 0.1],
            },
            // bottom face (red)
            Vertex {
                position: [-0.5, 0.5, -0.5],
                color: [0.8, 0.1, 0.1],
            },
            Vertex {
                position: [0.5, 0.5, 0.5],
                color: [0.8, 0.1, 0.1],
            },
            Vertex {
                position: [-0.5, 0.5, 0.5],
                color: [0.8, 0.1, 0.1],
            },
            Vertex {
                position: [-0.5, 0.5, -0.5],
                color: [0.8, 0.1, 0.1],
            },
            Vertex {
                position: [0.5, 0.5, -0.5],
                color: [0.8, 0.1, 0.1],
            },
            Vertex {
                position: [0.5, 0.5, 0.5],
                color: [0.8, 0.1, 0.1],
            },
            // nose face (blue)
            Vertex {
                position: [-0.5, -0.5, 00.5],
                color: [0.1, 0.1, 0.8],
            },
            Vertex {
                position: [0.5, 0.5, 00.5],
                color: [0.1, 0.1, 0.8],
            },
            Vertex {
                position: [-0.5, 0.5, 00.5],
                color: [0.1, 0.1, 0.8],
            },
            Vertex {
                position: [-0.5, -0.5, 00.5],
                color: [0.1, 0.1, 0.8],
            },
            Vertex {
                position: [0.5, -0.5, 00.5],
                color: [0.1, 0.1, 0.8],
            },
            Vertex {
                position: [0.5, 0.5, 00.5],
                color: [0.1, 0.1, 0.8],
            },
            // tail face (green)
            Vertex {
                position: [-0.5, -0.5, -00.5],
                color: [0.1, 0.8, 0.1],
            },
            Vertex {
                position: [0.5, 0.5, -00.5],
                color: [0.1, 0.8, 0.1],
            },
            Vertex {
                position: [-0.5, 0.5, -00.5],
                color: [0.1, 0.8, 0.1],
            },
            Vertex {
                position: [-0.5, -0.5, -00.5],
                color: [0.1, 0.8, 0.1],
            },
            Vertex {
                position: [0.5, -0.5, -00.5],
                color: [0.1, 0.8, 0.1],
            },
            Vertex {
                position: [0.5, 0.5, -00.5],
                color: [0.1, 0.8, 0.1],
            },
        ]
    }

    pub fn get_buffer(device: &Arc<Device>) -> Arc<CpuAccessibleBuffer<[Vertex]>> {
        CpuAccessibleBuffer::from_iter(
            device.clone(),
            BufferUsage::all(),
            false,
            Vertex::get_vertex().into_iter(),
        )
        .expect("could not create cpu access buffer")
    }
}
