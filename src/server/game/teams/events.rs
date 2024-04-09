use bevy::prelude::*;

#[derive(Debug, Clone, Event)]
pub struct IncrementScore {
    pub(crate) player: Entity,
}
