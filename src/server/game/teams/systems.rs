use super::{IncrementScore, Player, Team};
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

#[cfg(test)]
mod tests {
    use bevy::{app::App, render::color::Color};

    use crate::game::teams::{IncrementScore, Player, Team, TeamsPlugin};

    #[test]
    fn increment_team_score_on_event() {
        let mut app = App::new();

        app.add_event::<IncrementScore>();
        app.add_plugins(TeamsPlugin);

        let team = app.world.spawn(Team::new(Color::RED)).id();
        let player = app.world.spawn(Player::new(team)).id();

        app.world.send_event(IncrementScore { player });

        app.update();

        let team = app.world.get::<Team>(team).expect("Team to be found");
        assert_eq!(team.score, 1);
    }
}
