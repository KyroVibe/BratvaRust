use crate::util::vertex3::Vertex3;

pub struct Cube {
    color: [f32; 3],
}

impl Cube {
    pub fn new(color: [f32; 3]) -> Self {
        Cube {
            color: color,
        }
    }

    pub fn create_cube_buffers(display: &glium::Display) -> Option<(glium::VertexBuffer<Vertex3>, glium::IndexBuffer<u16>)> {
        let vertex_buffer = {
    
            glium::VertexBuffer::new(display,
                &[
                    // BACK
                    Vertex3 { position: [-0.5, -0.5, -0.5], color: [1.0, 1.0, 1.0], normal: [0.0, 0.0, -1.0] },
                    Vertex3 { position: [-0.5,  0.5, -0.5], color: [1.0, 0.0, 0.0], normal: [0.0, 0.0, -1.0] },
                    Vertex3 { position: [ 0.5,  0.5, -0.5], color: [0.0, 1.0, 0.0], normal: [0.0, 0.0, -1.0] },
    
                    Vertex3 { position: [ 0.5,  0.5, -0.5], color: [0.0, 1.0, 0.0], normal: [0.0, 0.0, -1.0] },
                    Vertex3 { position: [ 0.5, -0.5, -0.5], color: [1.0, 1.0, 1.0], normal: [0.0, 0.0, -1.0] },
                    Vertex3 { position: [-0.5, -0.5, -0.5], color: [1.0, 1.0, 1.0], normal: [0.0, 0.0, -1.0] },
    
                    // LEFT
                    Vertex3 { position: [-0.5, -0.5,  0.5], color: [1.0, 1.0, 1.0], normal: [-1.0, 0.0, 0.0] },
                    Vertex3 { position: [-0.5,  0.5,  0.5], color: [0.0, 0.0, 1.0], normal: [-1.0, 0.0, 0.0] },
                    Vertex3 { position: [-0.5,  0.5, -0.5], color: [1.0, 0.0, 0.0], normal: [-1.0, 0.0, 0.0] },
    
                    Vertex3 { position: [-0.5,  0.5, -0.5], color: [1.0, 0.0, 0.0], normal: [-1.0, 0.0, 0.0] },
                    Vertex3 { position: [-0.5, -0.5, -0.5], color: [1.0, 1.0, 1.0], normal: [-1.0, 0.0, 0.0] },
                    Vertex3 { position: [-0.5, -0.5,  0.5], color: [1.0, 1.0, 1.0], normal: [-1.0, 0.0, 0.0] },
    
                    // FRONT
                    Vertex3 { position: [-0.5, -0.5,  0.5], color: [1.0, 1.0, 1.0], normal: [0.0, 0.0, 1.0] },
                    Vertex3 { position: [-0.5,  0.5,  0.5], color: [0.0, 0.0, 1.0], normal: [0.0, 0.0, 1.0] },
                    Vertex3 { position: [ 0.5,  0.5,  0.5], color: [1.0, 1.0, 0.0], normal: [0.0, 0.0, 1.0] },
    
                    Vertex3 { position: [ 0.5,  0.5,  0.5], color: [1.0, 1.0, 0.0], normal: [0.0, 0.0, 1.0] },
                    Vertex3 { position: [ 0.5, -0.5,  0.5], color: [1.0, 1.0, 1.0], normal: [0.0, 0.0, 1.0] },
                    Vertex3 { position: [-0.5, -0.5,  0.5], color: [1.0, 1.0, 1.0], normal: [0.0, 0.0, 1.0] },
    
                    // RIGHT
                    Vertex3 { position: [ 0.5, -0.5,  0.5], color: [1.0, 1.0, 1.0], normal: [1.0, 0.0, 0.0] },
                    Vertex3 { position: [ 0.5,  0.5,  0.5], color: [1.0, 1.0, 0.0], normal: [1.0, 0.0, 0.0] },
                    Vertex3 { position: [ 0.5,  0.5, -0.5], color: [0.0, 1.0, 0.0], normal: [1.0, 0.0, 0.0] },
    
                    Vertex3 { position: [ 0.5,  0.5, -0.5], color: [0.0, 1.0, 0.0], normal: [1.0, 0.0, 0.0] },
                    Vertex3 { position: [ 0.5, -0.5, -0.5], color: [1.0, 1.0, 1.0], normal: [1.0, 0.0, 0.0] },
                    Vertex3 { position: [ 0.5, -0.5,  0.5], color: [1.0, 1.0, 1.0], normal: [1.0, 0.0, 0.0] },
    
                    // TOP
                    Vertex3 { position: [-0.5,  0.5, -0.5], color: [1.0, 0.0, 0.0], normal: [0.0, 1.0, 0.0] },
                    Vertex3 { position: [-0.5,  0.5,  0.5], color: [0.0, 0.0, 1.0], normal: [0.0, 1.0, 0.0] },
                    Vertex3 { position: [ 0.5,  0.5,  0.5], color: [1.0, 1.0, 0.0], normal: [0.0, 1.0, 0.0] },
    
                    Vertex3 { position: [ 0.5,  0.5,  0.5], color: [1.0, 1.0, 0.0], normal: [0.0, 1.0, 0.0] },
                    Vertex3 { position: [ 0.5,  0.5, -0.5], color: [0.0, 1.0, 0.0], normal: [0.0, 1.0, 0.0] },
                    Vertex3 { position: [-0.5,  0.5, -0.5], color: [1.0, 0.0, 0.0], normal: [0.0, 1.0, 0.0] },
    
                    // BOTTOM
                    Vertex3 { position: [-0.5, -0.5, -0.5], color: [1.0, 1.0, 1.0], normal: [0.0, -1.0, 0.0] },
                    Vertex3 { position: [ 0.5, -0.5,  0.5], color: [1.0, 1.0, 1.0], normal: [0.0, -1.0, 0.0] },
                    Vertex3 { position: [-0.5, -0.5,  0.5], color: [1.0, 1.0, 1.0], normal: [0.0, -1.0, 0.0] },
    
                    Vertex3 { position: [-0.5, -0.5, -0.5], color: [1.0, 1.0, 1.0], normal: [0.0, -1.0, 0.0] },
                    Vertex3 { position: [ 0.5, -0.5, -0.5], color: [1.0, 1.0, 1.0], normal: [0.0, -1.0, 0.0] },
                    Vertex3 { position: [ 0.5, -0.5,  0.5], color: [1.0, 1.0, 1.0], normal: [0.0, -1.0, 0.0] },
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
        let index_buffer = glium::IndexBuffer::new(display, glium::index::PrimitiveType::TrianglesList,
            // &[0u16, 1, 2, 3,4,5,6,7,8,9,10,11,12,13,14,15,16,17,18,19,20,21,22,23,24,25,26,27,28,29,30,31,32,33,34,35]
            a.as_slice()
        ).unwrap();

        Some((vertex_buffer, index_buffer))
    }

    pub fn create_tetrahedron_buffers(display: &glium::Display) -> Option<(glium::VertexBuffer<Vertex3>, glium::IndexBuffer<u16>)> {
        let vertex_buffer = {

            let root = 3f32.sqrt() / 2.0;
    
            glium::VertexBuffer::new(display,
                &[
                    // BOTTOM
                    Vertex3 { position: [ root, -0.5, -0.5], color: [0.0, 1.0, 0.0], normal: [0.0, 0.0, -1.0] },
                    Vertex3 { position: [-root, -0.5, -0.5], color: [0.0, 0.0, 1.0], normal: [0.0, 0.0, -1.0] },
                    Vertex3 { position: [  0.0, -0.5,  1.0], color: [1.0, 0.0, 0.0], normal: [0.0, 0.0, -1.0] },
    
                    // BACK
                    Vertex3 { position: [-0.5, -0.5,  0.5], color: [0.0, 1.0, 0.0], normal: [-1.0, 0.0, 0.0] },
                    Vertex3 { position: [-0.5,  0.5,  0.5], color: [0.0, 0.0, 1.0], normal: [-1.0, 0.0, 0.0] },
                    Vertex3 { position: [-0.5,  0.5, -0.5], color: [1.0, 0.0, 0.0], normal: [-1.0, 0.0, 0.0] },
    
                    // FRONT LEFT
                    Vertex3 { position: [-0.5, -0.5,  0.5], color: [0.0, 1.0, 0.0], normal: [0.0, 0.0, 1.0] },
                    Vertex3 { position: [-0.5,  0.5,  0.5], color: [0.0, 0.0, 1.0], normal: [0.0, 0.0, 1.0] },
                    Vertex3 { position: [ 0.5,  0.5,  0.5], color: [1.0, 0.0, 0.0], normal: [0.0, 0.0, 1.0] },
    
                    // FRONT RIGHT
                    Vertex3 { position: [ 0.5, -0.5,  0.5], color: [0.0, 1.0, 0.0], normal: [1.0, 0.0, 0.0] },
                    Vertex3 { position: [ 0.5,  0.5,  0.5], color: [0.0, 0.0, 1.0], normal: [1.0, 0.0, 0.0] },
                    Vertex3 { position: [ 0.5,  0.5, -0.5], color: [1.0, 0.0, 0.0], normal: [1.0, 0.0, 0.0] },
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
        let index_buffer = glium::IndexBuffer::new(display, glium::index::PrimitiveType::TrianglesList,
            // &[0u16, 1, 2, 3,4,5,6,7,8,9,10,11,12,13,14,15,16,17,18,19,20,21,22,23,24,25,26,27,28,29,30,31,32,33,34,35]
            a.as_slice()
        ).unwrap();

        Some((vertex_buffer, index_buffer))
    }
}
