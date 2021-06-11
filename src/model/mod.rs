use std::{cell::RefCell, sync::Arc};

use glam::{Mat4, Vec3};
use vulkano::{buffer::CpuAccessibleBuffer, device::Device};

mod vertex;

pub use vertex::Vertex;

const MAX_DIM: usize = 3;
type Dims = [f32; MAX_DIM];
type Rotate = [f32; MAX_DIM];

pub struct GameObject {
    #[allow(dead_code)]
    id: usize,
    translate: Dims,
    scale: Dims,
    pub rotate: Rotate,
    pub vertex_buffer: Arc<CpuAccessibleBuffer<[Vertex]>>,
}

thread_local! {
    static OBJECT_COUNT: RefCell<usize> = RefCell::new(0);
}

impl GameObject {
    pub fn new(device: &Arc<Device>, translate: Dims, scale: Dims, rotate: Rotate) -> Self {
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
        }
    }

    #[allow(dead_code)]
    fn get_id(&self) -> usize {
        self.id
    }

    pub fn get_push_data(&self) -> crate::shaders::vs::ty::PushConstantData {
        let translate = Mat4::from_translation(Vec3::from_slice(&self.translate));
        let rotation = Mat4::from_rotation_y(self.rotate[0]);
        let rotation = rotation * Mat4::from_rotation_x(self.rotate[1]);
        let rotation = rotation * Mat4::from_rotation_z(self.rotate[2]);
        let scale = Mat4::from_scale(Vec3::from_slice(&self.scale));
        let transform = translate * scale * rotation ;
        crate::shaders::vs::ty::PushConstantData {
            transform: Into::<Mat4>::into(transform).to_cols_array_2d(),
        }
    }
}
