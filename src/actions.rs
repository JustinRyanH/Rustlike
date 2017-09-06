//! Actions that are distributable into world that changes state

/// Action that can be performed on the world
#[derive(Clone, Copy)]
pub enum Action {
    /// Moves Player by X amount and Y amount
    MovePlayerBy {
        /// Horizontal Movement
        x: i32,
        /// Vertical Movement
        y: i32,
    },
    /// No active action was taken
    Noop,
}
