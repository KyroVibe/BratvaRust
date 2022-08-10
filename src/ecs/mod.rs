use std::collections::{HashMap};

#[derive(Clone, Copy, Eq, PartialEq, Hash)]
pub struct EntityHandle(u64);

// impl PartialEq for EntityHandle {
//     fn eq(&self, rhs: &Self) -> bool {
//         self.0 == rhs.0
//     }
// }

// impl Eq for EntityHandle { }

pub struct Entity {
    handle: EntityHandle,
    name: std::string::String,
}

pub struct EntityManager {
    entity_map: HashMap<EntityHandle, Entity>,
    next_guid: u64,
}

impl Entity {

}

impl EntityManager {
    pub fn new() -> Self {
        EntityManager {
            next_guid: 0,
            entity_map: HashMap::new()
        }
    }

    pub fn create_entity(&mut self, name: &str) -> EntityHandle {
        self.next_guid += 1;
        let handle: EntityHandle = EntityHandle(self.next_guid);
        let ent: Entity = Entity {
            handle: handle,
            name: String::from(name),
        };
        self.entity_map.insert(handle, ent);

        handle
    }

    pub fn get(&self, handle: EntityHandle) -> Result<&Entity, &'static str> {
        if !self.entity_map.contains_key(&handle) {
            Err("No entity with that handle exists")
        } else {
            Ok(self.entity_map.get(&handle).unwrap())
        }
    }

    pub fn get_mut(&mut self, handle: EntityHandle) -> Result<&mut Entity, &'static str> {
        if !self.entity_map.contains_key(&handle) {
            Err("No entity with that handle exists")
        } else {
            Ok(self.entity_map.get_mut(&handle).unwrap())
        }
    }
}
