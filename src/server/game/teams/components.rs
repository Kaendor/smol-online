use bevy::prelude::*;

#[derive(Component, Debug)]
pub struct Team {
    pub(crate) color: Color,
    pub(crate) score: u128,
}

impl Team {
    pub fn new(color: Color) -> Self {
        Team { color, score: 0 }
    }

    /// A player can increment his team score by one
    pub fn increment_score(&mut self) {
        self.score += 1;
    }
}

#[derive(Component, Debug)]
pub struct Player {
    pub(crate) team: Entity,
}

impl Player {
    pub fn new(team: Entity) -> Self {
        Player { team }
    }
}
