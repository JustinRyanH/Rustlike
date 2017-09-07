//! Elements that are renderable in the universe


/// Player Entity
pub mod player;
/// Debug Entity
pub mod debug;

use std::iter::IntoIterator;
use std::cmp::Ordering;
use std::clone::Clone;

use serde::export::fmt::Debug;
use opengl_graphics::GlGraphics;
use graphics::{Context};

use actions::Action;
use state::Stateful;
use render::game::GameViewSettings;
use render::Drawable;
use entities::player::Player;
use entities::debug::Debug as DebugEntity;

/// Collection of Entities
#[derive(Clone, PartialEq, Debug)]
pub struct EntityCollection(Vec<Entity>);

impl EntityCollection {
    /// Returns a new Empty Collection
    pub fn new() -> EntityCollection {
        return EntityCollection(vec![])
    }

    /// Adds entry to EntityCollection and returns a fresh copy
    pub fn add(self, entity: Entity) -> EntityCollection {
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

impl Stateful for EntityCollection {
    fn next(&self, action: Action) -> EntityCollection {
        return EntityCollection(self.clone().into_iter().map(|e| e.next(action.clone())).collect());
    }
}

impl IntoIterator for EntityCollection {
    type Item = Entity;
    type IntoIter = ::std::vec::IntoIter<Entity>;

    fn into_iter(self) -> Self::IntoIter {
        self.0.into_iter()
    }
}


/// Entity
#[derive(Copy, Clone, PartialEq, Debug)]
pub enum Entity {
    /// Player Entities are entities that the player has direct control over
    Player(Player),
    /// Entities that simply draw debug rectangles on front
    Debug(DebugEntity),
}

impl Stateful for Entity {
    fn next(&self, action: Action) -> Entity {
        match *self {
            Entity::Player(p) => Entity::Player(p.next(action)),
            Entity::Debug(d) => Entity::Debug(d.next(action)),
        }
    }
}

impl Identifiable for Entity {
    fn identify(&self) -> u64 {
        match *self {
            Entity::Player(p) => p.identify(),
            Entity::Debug(d) => d.identify(),
        }
    }
}

impl Drawable for Entity {
    fn draw<'a>(&self, settings: &'a GameViewSettings, ctx: &Context, gfx: &mut GlGraphics) {
        match *self {
            Entity::Player(p) => p.draw(settings, ctx, gfx),
            Entity::Debug(d) => d.draw(settings, ctx, gfx),
        }
    }
}

/// EntityKind represents the companies are are representable in the world
pub trait EntityKind: Drawable + Identifiable + Debug{}

impl PartialEq for EntityKind {
    fn eq(&self, other: &EntityKind) -> bool {
        self.identify() == other.identify()
    }
}

impl Eq for EntityKind {}

impl Ord for EntityKind {
    fn cmp(&self, other: &EntityKind) -> Ordering {
        self.identify().cmp(&other.identify())
    }
}

impl PartialOrd for EntityKind {
    fn partial_cmp(&self, other: &EntityKind) -> Option<Ordering> {
        self.identify().partial_cmp(&other.identify())
    }
}

/// Hashes the object into a 64 bit integer
pub trait Identifiable {
    /// unique identifier for element
    fn identify(&self) -> u64;
}

#[cfg(test)]
mod tests {
    use entities::{Entity};
    use entities::player::Player;


    #[test]
    fn test() {
        let entities: Vec<Entity> = vec![Entity::Player(Player::new([0, 0]))];
        let result: Vec<Entity> = vec![Entity::Player(Player::new([0, 0]))];
        assert_eq!(entities, result);
    }

    #[cfg(test)]
    mod entities {
        
        use entities::{Entity, EntityCollection, Identifiable};
        use entities::player::Player;

        #[test]
        fn new() {
            let subject = EntityCollection::new();
            assert!(subject.0.len() == 0);
        }

        #[test]
        fn add() {
            let expected_identity = Player::new([0, 0]).identify();
            /// When an Entity is added to a new EntityCollection
            let subject = EntityCollection::new().add(Entity::Player(Player::new([0, 0])));
            
            /// That Entity exists in the returned EntityCollection
            assert!(subject.into_iter().any(|entity| entity.identify() == expected_identity));
        }

        #[test]
        fn remove() {
            let unexpected_identity = Player::new([0, 0]).identify();
            
            /// When an Entity is added to a new Collection, and then removed
            let subject = EntityCollection::new().add(Entity::Player(Player::new([0, 0]))).remove(unexpected_identity);

            /// That Entity does not exists in the returned EntityCollection
            assert!(!subject.into_iter().any(|entity| entity.identify() == unexpected_identity));
        }
    }
}
