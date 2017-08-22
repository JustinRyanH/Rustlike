//! Drawable Ite in the universe
use graphics::{Context, Graphics};

use render::game::GameViewSettings;
use entities::player::PlayerEntity;

pub mod player;

/// Used to identify events arguments provided by traits.
#[derive(Clone, PartialEq, Eq, Debug)]
pub enum EntityKind {
    Player(PlayerEntity),
}

pub trait Entity: Sized {
    fn kind(self) -> EntityKind;
    fn draw<'a, G: Graphics>(&self, settings: &'a GameViewSettings, ctx: &Context, gfx: &mut G);
}
