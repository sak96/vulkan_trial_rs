pub mod vs {
    vulkano_shaders::shader! {
        ty: "vertex",
        src: "
              #version 450

              layout(location = 0) in vec3 position;
              layout(location = 1) in vec3 color;
              layout(location = 0) out vec4 vertex_color;

              layout(push_constant) uniform PushConstantData {
                  mat4 transform;
              } push;

              void main() {
                  gl_Position = push.transform * vec4(position, 1.0);
                  vertex_color = vec4(color, 1.0);
              }
          "
    }
}

pub mod fs {
    vulkano_shaders::shader! {
        ty: "fragment",
        src: "
                #version 450

                layout(location =0) in vec4 vertex_color;
                layout(location =0) out vec4 f_color;
                void main() {
                    f_color = vertex_color;
                }
            "
    }
}
