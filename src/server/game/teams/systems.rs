use super::{
    components::Player,
    components::Team,
    events::{IncrementScore, NewPlayer},
};
use bevy::prelude::*;

pub fn increment_team_score(
    mut events: EventReader<IncrementScore>,
    mut players: Query<(&Player, &mut Team)>,
) {
    for event in events.read() {
        if let Ok((_, mut team)) = players.get_mut(event.player) {
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
            let new_player = Player::new();
            commands.spawn(new_player);
        } else {
            warn!("Failed to add new player to team");
        }
    }
}
