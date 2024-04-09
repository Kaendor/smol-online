use bevy::{
    app::{App, Plugin},
    prelude::*,
};

use self::systems::increment_team_score;

pub struct TeamsPlugin;

mod components;
mod events;
mod systems;

impl Plugin for TeamsPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<events::IncrementScore>().add_systems(
            Update,
            increment_team_score.run_if(on_event::<events::IncrementScore>()),
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
