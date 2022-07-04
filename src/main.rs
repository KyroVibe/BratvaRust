#[macro_use]
extern crate glium;

mod sprite;

use glium::glutin::dpi::{LogicalSize};
use glium::{glutin, Surface};
use glium::index::PrimitiveType;

fn main() {
    let event_loop = glutin::event_loop::EventLoop::new();
    let wb = glutin::window::WindowBuilder::new()
        .with_inner_size(LogicalSize::new(500.0, 500.0));
    let cb = glutin::ContextBuilder::new();
    let display = glium::Display::new(wb, cb, &event_loop).unwrap();

    // building the vertex buffer, which contains all the vertices that we will draw
    let vertex_buffer = {
        #[derive(Copy, Clone)]
        struct Vertex {
            position: [f32; 2],
            color: [f32; 3],
        }

        implement_vertex!(Vertex, position, color);

        glium::VertexBuffer::new(&display,
            &[
                Vertex { position: [-0.5, -0.5], color: [0.0, 1.0, 0.0] },
                Vertex { position: [-0.5,  0.5], color: [0.0, 0.0, 1.0] },
                Vertex { position: [ 0.5,  0.5], color: [1.0, 0.0, 0.0] },
                Vertex { position: [ 0.5, -0.5], color: [0.0, 0.0, 1.0] },
            ]
        ).unwrap()
    };

    // building the index buffer
    let index_buffer = glium::IndexBuffer::new(&display, PrimitiveType::TrianglesList,
                                               &[0u16, 1, 2, 2, 3, 0]).unwrap();

    // compiling shaders and linking them together
    let program = program!(&display,
        330 => {
            vertex: "
                #version 330
                uniform mat4 mat;
                in vec2 position;
                in vec3 color;
                out vec3 vColor;
                void main() {
                    gl_Position = vec4(position, 0.0, 1.0) * mat;
                    vColor = color;
                }
            ",

            geometry: "
                #version 330
                layout(points) in;
                layout(triangle_strip, max_vertices=6) out;

                in vec2 pos[];
                in float size[];
            ",

            fragment: "
                #version 330
                in vec3 vColor;
                out vec4 f_color;
                void main() {
                    f_color = vec4(vColor, 1.0);
                }
            "
        },
    ).unwrap();

    // Here we draw the black background and triangle to the screen using the previously
    // initialised resources.
    //
    // In this case we use a closure for simplicity, however keep in mind that most serious
    // applications should probably use a function that takes the resources as an argument.
    let mut theta = 3.14159f32 * (1.0 / 6.0); // 30 degrees
    let mut draw = move || {
        // building the uniforms
        theta += 0.001;
        let uniforms = uniform! {
            mat: [
                [ theta.cos(), theta.sin(), 0.0, 0.0],
                [-theta.sin(), theta.cos(), 0.0, 0.0],
                [0.0, 0.0, 1.0, 0.0],
                [0.0, 0.0, 0.0, 1.0f32]
            ]
        };

        // drawing a frame
        let mut target = display.draw();
        target.clear_color(0.0, 0.0, 0.0, 0.0);
        target.draw(&vertex_buffer, &index_buffer, &program, &uniforms, &Default::default()).unwrap();
        target.finish().unwrap();
    };

    // Draw the triangle to the screen.
    draw();

    // the main loop
    event_loop.run(move |event, _, control_flow| {

        draw();

        *control_flow = match event {
            glutin::event::Event::WindowEvent { event, .. } => match event {
                // Break from the main loop when the window is closed.
                glutin::event::WindowEvent::CloseRequested => glutin::event_loop::ControlFlow::Exit,
                // Redraw the triangle when the window is resized.
                glutin::event::WindowEvent::Resized(..) => {
                    glutin::event_loop::ControlFlow::Poll
                },
                _ => glutin::event_loop::ControlFlow::Poll,
            },
            _ => glutin::event_loop::ControlFlow::Poll,
        };
    });
}