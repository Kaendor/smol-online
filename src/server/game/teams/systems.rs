use super::{components::Player, components::Team, events::IncrementScore};
use bevy::prelude::*;

pub fn increment_team_score(
    mut events: EventReader<IncrementScore>,
    mut teams: Query<&mut Team>,
    players: Query<&Player>,
) {
    for event in events.read() {
        if let Ok(mut team) = players
            .get(event.player)
            .and_then(|player| teams.get_mut(player.team))
        {
            team.increment_score();
        }
    }
}
