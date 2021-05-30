use std::sync::Arc;

use vulkano::{
    buffer::{BufferUsage, CpuAccessibleBuffer},
    device::Device,
};

#[derive(Default, Clone)]
pub struct Vertex {
    position: [f32; 2],
    color: [f32; 4],
}

vulkano::impl_vertex!(Vertex, position, color);
impl Vertex {
    fn sierpinski(
        vertices: &mut Vec<Vertex>,
        depth: usize,
        top: [f32; 2],
        left: [f32; 2],
        right: [f32; 2],
    ) {
        if depth == 0 {
            for position in std::array::IntoIter::new([top, left, right]) {
                vertices.push(Vertex {
                    position,
                    color: [1.0, 1.0, 0.0, 1.0],
                })
            }
        } else {
            let top_left = [(top[0] + left[0]) / 2.0, (top[1] + left[1]) / 2.0];
            let top_right = [(top[0] + right[0]) / 2.0, (top[1] + right[1]) / 2.0];
            let right_left = [(right[0] + left[0]) / 2.0, (right[1] + left[1]) / 2.0];
            Self::sierpinski(&mut *vertices, depth - 1, top, top_left, top_right);
            Self::sierpinski(&mut *vertices, depth - 1, top_left, left, right_left);
            Self::sierpinski(&mut *vertices, depth - 1, top_right, right_left, right);
        }
    }
    fn get_vertex() -> Vec<Vertex> {
        let mut vertices = vec![];
        Self::sierpinski(&mut vertices, 5, [-0.5, 0.5], [0.5, 0.5], [0.0, -0.5]);
        vertices
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
