pub mod vs {
    vulkano_shaders::shader! {
        ty: "vertex",
        src: "
              #version 450

              layout(location = 0) in vec2 position;

              layout(location =0) out vec4 vertex_color;

              layout(push_constant) uniform PushConstantData {
                  vec4 color;
                  vec2 offset;
              } push;

              void main() {
                  gl_Position = vec4(position + push.offset, 0.0, 1.0);
                  vertex_color = push.color;
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
