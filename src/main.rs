extern crate gl;
extern crate glutin;

use gl::types::*;
use glutin::event::{Event, WindowEvent};
use glutin::event_loop::{ControlFlow, EventLoop};
use glutin::window::{Window, WindowBuilder};
use glutin::ContextBuilder;

fn main() {
    let event_loop = EventLoop::new();
    let window = WindowBuilder::new()
        .with_title("Hello Triangle")
        .with_inner_size(glutin::dpi::PhysicalSize::new(800, 600))
        .build(&event_loop)
        .unwrap();

    let context = ContextBuilder::new()
        .with_vsync(true)
        .build_windowed(window, &event_loop)
        .unwrap();

    let gl = gl::Gl::load_with(|symbol| context.get_proc_address(symbol) as *const _);

    unsafe {
        context.make_current().unwrap();
        gl.ClearColor(0.0, 0.0, 0.0, 1.0);

        let vertex_shader = gl.CreateShader(gl::VERTEX_SHADER);
        gl.ShaderSource(
            vertex_shader,
            1,
            &format!("#version 330
                in vec3 position;
                void main() {{
                    gl_Position = vec4(position, 1.0);
                }}
            ").as_ptr() as *const *const GLchar,
            std::ptr::null(),
        );
        gl.CompileShader(vertex_shader);

        let fragment_shader = gl.CreateShader(gl::FRAGMENT_SHADER);
        gl.ShaderSource(
            fragment_shader,
            1,
            &format!("#version 330
                out vec4 color;
                void main() {{
                    color = vec4(1.0, 0.0, 0.0, 1.0);
                }}
            ").as_ptr() as *const *const GLchar,
            std::ptr::null(),
        );
        gl.CompileShader(fragment_shader);

        let program = gl.CreateProgram();
        gl.AttachShader(program, vertex_shader);
        gl.AttachShader(program, fragment_shader);
        gl.LinkProgram(program);
        gl.UseProgram(program);

        let vertices: [f32; 9] = [-0.5, -0.5, 0.0, 0.5, -0.5, 0.0, 0.0, 0.5, 0.0];

        let vbo = 0;
        gl.GenBuffers(1, &vbo);
        gl.BindBuffer(gl::ARRAY_BUFFER, vbo);
        gl.BufferData(
            gl::ARRAY_BUFFER,
            (vertices.len() * std::mem::size_of::<f32>()) as GLsizeiptr,
            vertices.as_ptr() as *const GLvoid,
            gl::STATIC_DRAW,
        );

        let vao = 0;
        gl.GenVertexArrays(1, &vao);
        gl.BindVertexArray(vao);
        gl.EnableVertexAttribArray(0);
        gl.VertexAttribPointer(
            0,
            3,
            gl::FLOAT,
            gl::FALSE,
            (3 * std::mem::size_of::<f32>()) as GLint,
            std::ptr::null(),
        );
    }

    event_loop.run(move |event, _, control_flow| {
        match event {
            Event::WindowEvent { event, .. } => match event {
                WindowEvent::CloseRequested => *control_flow = ControlFlow::Exit,
                WindowEvent::Resized(physical_size) => {
                    context.resize(physical_size)
                }
                _ => (),
            },
            Event::RedrawRequested(_) => {
                unsafe {
                    gl.Clear(gl::COLOR_BUFFER_BIT);
                    gl.DrawArrays(gl::TRIANGLES, 0, 3);
                }
                context.swap_buffers().unwrap();
            }
            _ => (),
        }
    });
}

