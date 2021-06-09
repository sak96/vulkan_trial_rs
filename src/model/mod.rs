use std::{cell::RefCell, sync::Arc};

use glam::{Mat2, Mat3, Vec2};
use vulkano::{buffer::CpuAccessibleBuffer, device::Device};

mod vertex;

pub use vertex::Vertex;

const MAX_DIM: usize = 2;
type Dims = [f32; MAX_DIM];
type Rotate = [f32; MAX_DIM - 1];
type Color = [f32; 4];

pub struct GameObject {
    id: usize,
    translate: Dims,
    scale: Dims,
    rotate: Rotate,
    color: Color,
    pub vertex_buffer: Arc<CpuAccessibleBuffer<[Vertex]>>,
}

thread_local! {
    static OBJECT_COUNT: RefCell<usize> = RefCell::new(0);
}

impl GameObject {
    pub fn new(
        device: &Arc<Device>,
        color: Color,
        translate: Dims,
        scale: Dims,
        rotate: Rotate,
    ) -> Self {
        let id = OBJECT_COUNT.with(|count| {
            let mut game_count = count.borrow_mut();
            let id = *game_count;
            *game_count += 1;
            id
        });
        let vertex_buffer = Vertex::get_buffer(&device);
        Self {
            id,
            vertex_buffer,
            translate,
            scale,
            rotate,
            color,
        }
    }

    fn get_id(&self) -> usize {
        self.id
    }

    pub fn get_push_data(&self) -> crate::shaders::vs::ty::PushConstantData {
        let rotation = Mat3::from_rotation_z(self.rotate[0]);
        let scale = Mat3::from_scale(Vec2::from_slice(&self.scale));
        let transform = rotation * scale ;
        crate::shaders::vs::ty::PushConstantData {
            color: self.color,
            transform: Into::<Mat2>::into(transform).to_cols_array_2d(),
            translate: self.translate,
        }
    }
}
