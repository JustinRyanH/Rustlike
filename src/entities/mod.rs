//! Elements that are renderable in the universe


/// Player Entity
pub mod player;

use std::iter::IntoIterator;
use std::cmp::Ordering;
use std::clone::Clone;

use serde::export::fmt::Debug;
use opengl_graphics::GlGraphics;
use graphics::{Context};

use render::game::GameViewSettings;

/// Collection of Entities
#[derive(Clone, PartialEq, Debug)]
pub struct EntityCollection(Vec<Box<Entity>>);

impl EntityCollection {
    /// Returns a new Empty Collection
    pub fn new() -> EntityCollection {
        return EntityCollection(vec![])
    }

    /// Adds entry to EntityCollection and returns a fresh copy
    pub fn add(self, entity: Box<Entity>) -> EntityCollection {
        let mut result = EntityCollection(self.0.iter().map(|entity| entity.clone()).collect());
        result.0.push(entity);
        return result;
    }

    /// Removes the entity from the Collection
    pub fn remove(self, entity_id: u64) -> EntityCollection {
        EntityCollection(self.0.iter().filter_map(|entity| {
            if entity.identify() == entity_id { return None }
            return Some(entity.clone());
        }).collect())
    }
}

impl IntoIterator for EntityCollection {
    type Item = Box<Entity>;
    type IntoIter = ::std::vec::IntoIter<Box<Entity>>;

    fn into_iter(self) -> Self::IntoIter {
        self.0.into_iter()
    }
}

/// Allows the Compiler to know that the Entity can be cloned,
/// and that data structures that implement Entity should be clonable
pub trait ClonedEntity {
    /// Creates a cloned box version of entity
    fn clone_box(&self) -> Box<Entity>;
}

impl<T> ClonedEntity for T where T: 'static + Entity + Clone {
    fn clone_box(&self) -> Box<Entity> {
        Box::new(self.clone())
    }
}

impl Clone for Box<Entity> {
    fn clone(&self) -> Box<Entity> {
        self.clone_box()
    }
}

/// Entity is a component that exists in
/// the world
pub trait Entity: ClonedEntity + Drawable + Identifiable + Debug{}

impl PartialEq for Entity {
    fn eq(&self, other: &Entity) -> bool {
        self.identify() == other.identify()
    }
}

impl Eq for Entity {}

impl Ord for Entity {
    fn cmp(&self, other: &Entity) -> Ordering {
        self.identify().cmp(&other.identify())
    }
}

impl PartialOrd for Entity {
    fn partial_cmp(&self, other: &Entity) -> Option<Ordering> {
        self.identify().partial_cmp(&other.identify())
    }
}

/// Hashes the object into a 64 bit integer
pub trait Identifiable {
    /// unique identifier for element
    fn identify(&self) -> u64;
}

/// An object that can be rendered
pub trait Drawable {
    /// Draws the entity to openGL
    fn draw<'a>(&self, settings: &'a GameViewSettings, ctx: &Context, gfx: &mut GlGraphics);
}

#[cfg(test)]
mod tests {
    use entities::Entity;
    use entities::player::PlayerEntity;


    #[test]
    fn test() {
        let entities: Vec<Box<Entity>> = vec![Box::new(PlayerEntity::new([0, 0]))];
        let result: Vec<Box<Entity>> = vec![Box::new(PlayerEntity::new([0, 0]))];
        assert_eq!(entities, result);
    }

    #[cfg(test)]
    mod entities {
        
        use entities::{EntityCollection, Identifiable};
        use entities::player::PlayerEntity;

        #[test]
        fn new() {
            let subject = EntityCollection::new();
            assert!(subject.0.len() == 0);
        }

        #[test]
        fn add() {
            let expected_identity = PlayerEntity::new([0, 0]).identify();
            /// When an Entity is added to a new EntityCollection
            let subject = EntityCollection::new().add(Box::new(PlayerEntity::new([0, 0])));
            
            /// That Entity exists in the returned EntityCollection
            assert!(subject.into_iter().any(|entity| entity.identify() == expected_identity));
        }

        #[test]
        fn remove() {
            let unexpected_identity = PlayerEntity::new([0, 0]).identify();
            
            /// When an Entity is added to a new Collection, and then removed
            let subject = EntityCollection::new().add(Box::new(PlayerEntity::new([0, 0]))).remove(unexpected_identity);

            /// That Entity does not exists in the returned EntityCollection
            assert!(!subject.into_iter().any(|entity| entity.identify() == unexpected_identity));
        }
    }
}
