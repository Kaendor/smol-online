use bevy::{
    app::{App, Plugin},
    prelude::*,
};

use self::{
    events::NewPlayer,
    systems::{add_new_player, increment_team_score},
};

pub struct TeamsPlugin;

mod components;
mod events;
mod systems;

impl Plugin for TeamsPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<events::IncrementScore>()
            .add_event::<NewPlayer>()
            .add_systems(
                Update,
                (
                    increment_team_score.run_if(on_event::<events::IncrementScore>()),
                    add_new_player.run_if(on_event::<events::NewPlayer>()),
                ),
            );
    }
}

#[cfg(test)]
mod tests {
    use bevy::{app::App, render::color::Color};

    use crate::game::teams::{
        components::{Player, Team},
        events::IncrementScore,
        TeamsPlugin,
    };

    use super::events::NewPlayer;

    fn teams_app() -> App {
        let mut app = App::new();

        app.add_plugins(TeamsPlugin);

        app
    }

    // TODO: Add player at client joining server

    // TODO: Send event to increment score on player input

    #[test]
    fn increment_team_score_on_event() {
        let mut app = teams_app();

        let team = app.world.spawn(Team::new(Color::RED)).id();
        let player = app.world.spawn(Player::new(team)).id();

        app.world.send_event(IncrementScore { player });

        app.update();

        let team = app.world.get::<Team>(team).expect("Team to be found");
        assert_eq!(team.score, 1);
    }

    #[test]
    fn add_new_player_on_event() {
        let mut app = teams_app();

        let team = app.world.spawn(Team::new(Color::RED)).id();

        app.world.send_event(NewPlayer { team });

        app.update();

        let player = app.world.query::<&Player>().single(&app.world);

        assert_eq!(player.team, team);
    }
}
