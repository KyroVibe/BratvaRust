pub struct Cube {
    color: [f32; 3],
}

impl Cube {
    pub fn new(color: [f32; 3]) -> Self {
        Cube {
            color: color,
        }
    }
}
