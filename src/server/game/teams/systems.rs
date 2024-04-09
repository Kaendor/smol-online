use super::{
    components::Player,
    components::Team,
    events::{IncrementScore, NewPlayer},
};
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

pub fn add_new_player(
    mut events: EventReader<NewPlayer>,
    teams: Query<&Team>,
    mut commands: Commands,
) {
    for event in events.read() {
        if let Ok(_team) = teams.get(event.team) {
            let new_player = Player::new(event.team);
            commands.spawn(new_player);
        } else {
            warn!("Failed to add new player to team");
        }
    }
}
