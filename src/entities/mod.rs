//! Elements that are renderable in the universe
/// Player Entity
pub mod player;

use std::cmp::Ordering;

use serde::export::fmt::Debug;
use opengl_graphics::GlGraphics;
use graphics::{Context};

use render::game::GameViewSettings;

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
}
