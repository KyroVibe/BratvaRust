use std::collections::HashMap;
use std::hash::{Hash};

use glium::{Display};
use glium::texture::{SrgbTexture2d};

pub struct Float2 {
    pub x: f32,
    pub y: f32,
}

pub struct Transform {
    pub mat: [[f32; 4]; 4]
}

#[derive(Copy, Clone, Hash)]
pub struct TextureHandle(u128);

pub struct TextureManager {
    next_guid: u128,
    textures: HashMap<TextureHandle, SrgbTexture2d>,
}

pub struct Sprite {
    pub transform: Transform,
    pub texture: TextureHandle,
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

impl Transform {
    pub fn new(mat: Option<[[f32; 4]; 4]>) -> Self {
        if let Some(matrix) = mat {
            return Transform {
                mat: matrix
            };
        } else {
            return Transform {
                mat: [
                    [ 1.0, 0.0, 0.0, 0.0 ],
                    [ 0.0, 1.0, 0.0, 0.0 ],
                    [ 0.0, 0.0, 1.0, 0.0 ],
                    [ 0.0, 0.0, 0.0, 1.0 ]
                ]
            };
        }
    }
}

impl From<(Float2, f32)> for Transform {
    fn from((position, rotation): (Float2, f32)) -> Self {
        Transform::new(Some(
            [
                [  rotation.cos(), rotation.sin(), 0.0, 0.0 ],
                [ -rotation.sin(), rotation.cos(), 0.0, 0.0 ],
                [             0.0,            0.0, 1.0, 0.0 ],
                [      position.x,     position.y, 0.0, 1.0 ]
            ]
        ))
    }
}

impl PartialEq for TextureHandle {
    fn eq(&self, other: &TextureHandle) -> bool {
        other.0 == self.0
    }
}

impl Eq for TextureHandle { }

impl TextureManager {
    pub fn new() -> Self {
        TextureManager { next_guid: 0, textures: HashMap::new() }
    }

    pub fn load_texture(&mut self, facade: &Display, pixels: Vec<Vec<(f32, f32, f32)>>) -> TextureHandle {
        self.next_guid += 1;

        let texture = SrgbTexture2d::new(facade, pixels).expect("Failed to load texture");
        self.textures.insert(TextureHandle(self.next_guid.clone()), texture);
        TextureHandle(self.next_guid.clone())
    }
}

impl Sprite {
    pub fn new(transform: Transform, texture: TextureHandle) -> Self {
        Sprite { transform: transform, texture: texture }
    }
}
