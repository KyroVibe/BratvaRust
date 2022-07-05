use crate::sprite::{Sprite, TextureManager};

use std::collections::{HashMap};
use std::hash::{Hash};
use glium::Display;

#[derive(Copy, Clone, Hash)]
pub struct SpriteHandle(u128);

pub struct Game {
    display: Display,
    next_sprite_guid: u128,
    sprites: HashMap<SpriteHandle, Sprite>,
    texture_manager: TextureManager,
}

impl PartialEq for SpriteHandle {
    fn eq(&self, other: &SpriteHandle) -> bool {
        other.0 == self.0
    }
}

impl Eq for SpriteHandle { }

impl Game {
    pub fn new(display: Display) -> Self {
        Game { display: display, next_sprite_guid: 0, sprites: HashMap::new(), texture_manager: TextureManager::new() }
    }

    pub fn add_sprite(&mut self, s: Sprite) -> SpriteHandle {
        self.next_sprite_guid += 1;
        let handle: SpriteHandle = SpriteHandle(self.next_sprite_guid.clone());
        self.sprites.insert(handle.clone(), s);
        handle
    }

    pub fn draw_sprites(&self) {

    }

    pub fn get_sprite(&self, handle: &SpriteHandle) -> Option<&Sprite> {
        None
    }

    pub fn get_sprite_mut(&mut self, handle: &SpriteHandle) -> Option<&mut Sprite> {
        None
    }
}
