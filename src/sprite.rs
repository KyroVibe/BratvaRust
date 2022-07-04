use std::collections::HashMap;

use glium::{Display};
use glium::texture::{SrgbTexture1d};

pub struct Float2 {
    pub x: f32,
    pub y: f32,
}

pub struct Quad {
    pos: [f32; 2],
    size: f32,
}

pub struct TextureManager {
    next_guid: u128,
    textures: HashMap<u128, SrgbTexture1d>,
}

pub struct Sprite {
    pub pos: Float2,
    pub size: f32,
    pub tex_id: u128,
}

impl Float2 {
    pub fn new(x: f32, y: f32) -> Self {
        Float2 { x: x, y: y }
    }
}

impl Into<Float2> for (f32, f32) {
    fn into(self) -> Float2 {
        Float2::new(self.0, self.1)
    }
}

impl Quad {
    pub fn new(x: f32, y: f32, size: f32) -> Self {
        Quad { pos: [x, y], size: size }
    }
}

impl TextureManager {
    pub fn new() -> Self {
        TextureManager { next_guid: 0, textures: HashMap::new() }
    }

    pub fn load_texture(&mut self, facade: &Display, pixels: Vec<(f32, f32, f32)>) -> u128 {
        self.next_guid += 1;

        let texture = SrgbTexture1d::new(facade, pixels).expect("Failed to load texture");
        self.textures.insert(self.next_guid.clone(), texture);
        self.next_guid.clone()
    }
}

impl Sprite {
    pub fn new(pos: Float2, size: f32) -> Self {
        Sprite { pos: pos, size: size, tex_id: 0 }
    }
}

impl Into<Quad> for Sprite {
    fn into(self) -> Quad {
        Quad::new(self.pos.x, self.pos.y, self.size)
    }
}
