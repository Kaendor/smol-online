use bevy::{
    app::{App, Plugin},
    prelude::*,
    render::color::Color,
};

use self::systems::increment_team_score;

pub struct TeamsPlugin;

mod systems;

impl Plugin for TeamsPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<IncrementScore>().add_systems(
            Update,
            increment_team_score.run_if(on_event::<IncrementScore>()),
        );
    }
}

#[derive(Component, Debug)]
pub struct Team {
    color: Color,
    score: u128,
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
    team: Entity,
}

impl Player {
    pub fn new(team: Entity) -> Self {
        Player { team }
    }
}

#[derive(Debug, Clone, Event)]
pub struct IncrementScore {
    player: Entity,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_team_increment_score() {
        let mut team = Team::new(Color::RED);
        team.increment_score();
        assert_eq!(team.score, 1);
    }

    // test if a incrementscore event increment the player's team score
    #[test]
    fn test_increment_score_event() {
        let mut team = Team::new(Color::RED);
        let player = Entity::from_raw(0);
        let event = IncrementScore { player };
        team.increment_score();
        assert_eq!(team.score, 1);
    }
}
