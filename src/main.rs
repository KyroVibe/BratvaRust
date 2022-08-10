#[macro_use]
extern crate glium;

mod cube;
mod ecs;

use glium::glutin::dpi::{LogicalSize};
use glium::{glutin, Surface};
use glium::index::PrimitiveType;

fn gen_projection_mat() -> [[f32; 4]; 4] {
    let fov = 3.141592f32 * (1.0 / 2.0);
    let tan_fov: f32 = f32::tan(fov / 2.0);
    let far = 10.0f32;
    let near = 0.05f32;
    [
        [ 1.0 / ((800.0 / 600.0) * tan_fov),           0.0,                            0.0,                                  0.0 ],
        [                               0.0, 1.0 / tan_fov,                            0.0,                                  0.0 ],
        [                               0.0,           0.0,  ((far + near) / (far - near)),  ((2.0 * far * near) / (far - near)) ],
        [                               0.0,           0.0,                            1.0,                               1.0f32 ]
    ]
}

fn main() {
    let event_loop = glutin::event_loop::EventLoop::new();
    let wb = glutin::window::WindowBuilder::new()
        .with_inner_size(LogicalSize::new(800.0, 600.0));
    let cb = glutin::ContextBuilder::new().with_depth_buffer(24);
    let display = glium::Display::new(wb, cb, &event_loop).unwrap();

    let cube = cube::Cube::new([1.0, 1.0, 1.0]);

    // building the vertex buffer, which contains all the vertices that we will draw
    let vertex_buffer = {
        #[derive(Copy, Clone)]
        struct Vertex {
            position: [f32; 3],
            color: [f32; 3],
            normal: [f32; 3],
        }

        implement_vertex!(Vertex, position, color, normal);

        glium::VertexBuffer::new(&display,
            &[
                // BACK
                Vertex { position: [-0.5, -0.5, -0.5], color: [0.0, 1.0, 0.0], normal: [0.0, 0.0, -1.0] },
                Vertex { position: [-0.5,  0.5, -0.5], color: [0.0, 0.0, 1.0], normal: [0.0, 0.0, -1.0] },
                Vertex { position: [ 0.5,  0.5, -0.5], color: [1.0, 0.0, 0.0], normal: [0.0, 0.0, -1.0] },

                Vertex { position: [ 0.5,  0.5, -0.5], color: [0.0, 1.0, 0.0], normal: [0.0, 0.0, -1.0] },
                Vertex { position: [ 0.5, -0.5, -0.5], color: [0.0, 0.0, 1.0], normal: [0.0, 0.0, -1.0] },
                Vertex { position: [-0.5, -0.5, -0.5], color: [1.0, 0.0, 0.0], normal: [0.0, 0.0, -1.0] },

                // LEFT
                Vertex { position: [-0.5, -0.5,  0.5], color: [0.0, 1.0, 0.0], normal: [-1.0, 0.0, 0.0] },
                Vertex { position: [-0.5,  0.5,  0.5], color: [0.0, 0.0, 1.0], normal: [-1.0, 0.0, 0.0] },
                Vertex { position: [-0.5,  0.5, -0.5], color: [1.0, 0.0, 0.0], normal: [-1.0, 0.0, 0.0] },

                Vertex { position: [-0.5,  0.5, -0.5], color: [0.0, 1.0, 0.0], normal: [-1.0, 0.0, 0.0] },
                Vertex { position: [-0.5, -0.5, -0.5], color: [0.0, 0.0, 1.0], normal: [-1.0, 0.0, 0.0] },
                Vertex { position: [-0.5, -0.5,  0.5], color: [1.0, 0.0, 0.0], normal: [-1.0, 0.0, 0.0] },

                // FRONT
                Vertex { position: [-0.5, -0.5,  0.5], color: [0.0, 1.0, 0.0], normal: [0.0, 0.0, 1.0] },
                Vertex { position: [-0.5,  0.5,  0.5], color: [0.0, 0.0, 1.0], normal: [0.0, 0.0, 1.0] },
                Vertex { position: [ 0.5,  0.5,  0.5], color: [1.0, 0.0, 0.0], normal: [0.0, 0.0, 1.0] },

                Vertex { position: [ 0.5,  0.5,  0.5], color: [0.0, 1.0, 0.0], normal: [0.0, 0.0, 1.0] },
                Vertex { position: [ 0.5, -0.5,  0.5], color: [0.0, 0.0, 1.0], normal: [0.0, 0.0, 1.0] },
                Vertex { position: [-0.5, -0.5,  0.5], color: [1.0, 0.0, 0.0], normal: [0.0, 0.0, 1.0] },

                // RIGHT
                Vertex { position: [ 0.5, -0.5,  0.5], color: [0.0, 1.0, 0.0], normal: [1.0, 0.0, 0.0] },
                Vertex { position: [ 0.5,  0.5,  0.5], color: [0.0, 0.0, 1.0], normal: [1.0, 0.0, 0.0] },
                Vertex { position: [ 0.5,  0.5, -0.5], color: [1.0, 0.0, 0.0], normal: [1.0, 0.0, 0.0] },

                Vertex { position: [ 0.5,  0.5, -0.5], color: [0.0, 1.0, 0.0], normal: [1.0, 0.0, 0.0] },
                Vertex { position: [ 0.5, -0.5, -0.5], color: [0.0, 0.0, 1.0], normal: [1.0, 0.0, 0.0] },
                Vertex { position: [ 0.5, -0.5,  0.5], color: [1.0, 0.0, 0.0], normal: [1.0, 0.0, 0.0] },

                // TOP
                Vertex { position: [-0.5,  0.5, -0.5], color: [0.0, 1.0, 0.0], normal: [0.0, 1.0, 0.0] },
                Vertex { position: [-0.5,  0.5,  0.5], color: [0.0, 0.0, 1.0], normal: [0.0, 1.0, 0.0] },
                Vertex { position: [ 0.5,  0.5,  0.5], color: [1.0, 0.0, 0.0], normal: [0.0, 1.0, 0.0] },

                Vertex { position: [ 0.5,  0.5,  0.5], color: [0.0, 1.0, 0.0], normal: [0.0, 1.0, 0.0] },
                Vertex { position: [ 0.5,  0.5, -0.5], color: [0.0, 0.0, 1.0], normal: [0.0, 1.0, 0.0] },
                Vertex { position: [-0.5,  0.5, -0.5], color: [1.0, 0.0, 0.0], normal: [0.0, 1.0, 0.0] },

                // BOTTOM
                Vertex { position: [-0.5, -0.5, -0.5], color: [0.0, 1.0, 0.0], normal: [0.0, -1.0, 0.0] },
                Vertex { position: [ 0.5, -0.5,  0.5], color: [0.0, 0.0, 1.0], normal: [0.0, -1.0, 0.0] },
                Vertex { position: [-0.5, -0.5,  0.5], color: [1.0, 0.0, 0.0], normal: [0.0, -1.0, 0.0] },

                Vertex { position: [-0.5, -0.5, -0.5], color: [0.0, 1.0, 0.0], normal: [0.0, -1.0, 0.0] },
                Vertex { position: [ 0.5, -0.5, -0.5], color: [0.0, 0.0, 1.0], normal: [0.0, -1.0, 0.0] },
                Vertex { position: [ 0.5, -0.5,  0.5], color: [1.0, 0.0, 0.0], normal: [0.0, -1.0, 0.0] },
            ]
        ).unwrap()
    };

    let mut a = Vec::<u16>::new();
    // for x in 0..12u16 {
    //     a.push(x * 3);
    //     a.push((x * 3) + 1);
    //     a.push((x * 3) + 1);
    //     a.push((x * 3) + 2);
    //     a.push((x * 3) + 2);
    //     a.push(x * 3);
    // }
    for x in 0..36u16 {
        a.push(x);
    }

    // building the index buffer
    let index_buffer = glium::IndexBuffer::new(&display, PrimitiveType::TrianglesList,
        // &[0u16, 1, 2, 3,4,5,6,7,8,9,10,11,12,13,14,15,16,17,18,19,20,21,22,23,24,25,26,27,28,29,30,31,32,33,34,35]
        a.as_slice()
    ).unwrap();

    let frag = std::fs::read_to_string("shaders/fragment.glsl").unwrap();
    let vert = std::fs::read_to_string("shaders/vertex.glsl").unwrap();

    // compiling shaders and linking them together
    let program = program!(&display,
        330 => {
            // vertex: "
            //     #version 330
            //     uniform mat4 mat;
            //     in vec2 position;
            //     in vec3 color;
            //     out vec3 vColor;
            //     void main() {
            //         gl_Position = vec4(position, 0.0, 1.0) * mat;
            //         vColor = color;
            //     }
            // ",

            vertex: vert.as_str(),

            /*geometry: "
                #version 330
                layout(points) in;
                layout(triangle_strip, max_vertices=6) out;

                in vec2 pos[];
                in float size[];
            ",*/

            // fragment: "
            //     #version 330
            //     in vec3 vColor;
            //     out vec4 f_color;
            //     void main() {
            //         f_color = vec4(vColor, 1.0);
            //     }
            // "
            fragment: frag.as_str()
        },
    ).unwrap();

    // Here we draw the black background and triangle to the screen using the previously
    // initialised resources.
    //
    // In this case we use a closure for simplicity, however keep in mind that most serious
    // applications should probably use a function that takes the resources as an argument.
    let mut theta = 3.14159f32 * (1.0 / 6.0); // 30 degrees
    // let mut theta = 0.0f32;
    let mut draw = move || {
        // building the uniforms
        theta += 0.0004;
        let uniforms = uniform! {
            mat: [
                [ theta.cos(), 0.0, theta.sin(), 0.0],
                [         0.0, 1.0,         0.0, 0.0],
                [-theta.sin(), 0.0, theta.cos(), 1.0],
                [         0.0, 0.0,         0.0, 1.0f32]
            ],
            // projection_mat: [
            //     [ 1.0, 0.0, 0.0, 0.0 ],
            //     [ 0.0, 1.0, 0.0, 0.2 ],
            //     [ 0.0, 0.0, 0.2, 0.5 ],
            //     [ 0.0, 0.0, 0.0, 1.0f32 ]
            // ],
            projection_mat: gen_projection_mat(),
            // mat: [
            //     [1.0, 0.0, 0.0, 0.0],
            //     [0.0, 1.0, 0.0, 0.0],
            //     [0.0, 0.0, 1.0, 0.0],
            //     [0.0, 0.0, 0.0, 1.0f32]
            // ],
            aspect_ratio: 800.0f32 / 600.0f32
        };

        // drawing a frame
        let params = glium::DrawParameters {
            depth: glium::Depth {
                test: glium::draw_parameters::DepthTest::IfLess,
                write: true,
                .. Default::default()
            },
            .. Default::default()
        };

        let mut target = display.draw();
        target.clear_color_and_depth((0.0, 0.0, 0.0, 0.0), 1.0);
        target.draw(&vertex_buffer, &index_buffer, &program, &uniforms, &params/* &Default::default() */).unwrap();
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
