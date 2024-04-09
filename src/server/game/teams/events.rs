use bevy::prelude::*;

#[derive(Debug, Clone, Event)]
pub struct IncrementScore {
    pub(crate) player: Entity,
}

#[derive(Debug, Clone, Event)]
pub struct NewPlayer {
    pub(crate) team: Entity,
}
