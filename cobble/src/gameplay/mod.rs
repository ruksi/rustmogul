use bevy::prelude::*;
use rand::prelude::*;

pub use board_token::ActiveBoardId;
use board_token::*;
use camera::*;
use conflict::*;
use light::*;
pub use robot_token::RobotToken;
use robot_token::*;
use synchronize::*;
use tile_token::*;

use crate::cleanup::*;
use crate::control::PointerEvent;
use crate::ledger::actions::CreateRobot;
use crate::ledger::reactions::*;
use crate::ledger::LedgerPlugin;
use crate::states::{GlobalState, PlayMenuState, PlayState};
use crate::timers::*;

mod board_token;
mod camera;
mod conflict;
mod light;
mod robot_token;
mod synchronize;
mod tile_token;

pub struct GameplayPlugin;

impl Plugin for GameplayPlugin {
    #[rustfmt::skip]
    fn build(
        &self,
        app: &mut App,
    ) {
        app.add_plugins(LedgerPlugin);

        // initialization
        app.init_resource::<BoardDatabase>();
        app.add_systems(
            OnEnter(GlobalState::InPlay),
            (
                exclusively_initialize_conflict,
                ensure_play_menu_closed,
                spawn_light,
                spawn_camera,
                spawn_board,
                |mut next: ResMut<NextState<PlayState>>| next.set(PlayState::Initialized),
            ).chain(),
        );

        // interaction
        app.add_systems(Update, handle_keybindings.run_if(in_state(GlobalState::InPlay)));
        app.add_systems(Update, handle_active_board_change.run_if(in_state(GlobalState::InPlay)));
        app.add_systems(Update, handle_orbit_camera.run_if(in_state(GlobalState::InPlay)));

        // reacting to reactions
        app.add_systems(Update, react_to_board_synchronized.run_if(on_event::<BoardSynchronized>()));
        app.add_systems(Update, react_to_robot_created.run_if(on_event::<RobotCreated>()));
        app.add_systems(Update, react_to_robot_moved.run_if(on_event::<RobotMoved>()));
        app.add_systems(Update, react_to_tile_created.run_if(on_event::<TileCreated>()));

        // triggering actions
        app.add_systems(Update, act_on_robot_or_tile_drop.run_if(on_event::<PointerEvent>()));

        // time
        app.add_systems(Update, tick_set_visibility_timers.run_if(in_state(GlobalState::InPlay)));

        // cleanup
        app.add_systems(
            OnExit(GlobalState::InPlay),
            (
                ensure_play_menu_closed,
                LedgerPlugin::cleanup,
                |mut next: ResMut<NextState<PlayState>>| next.set(PlayState::None),
                cleanup_for::<CleanOnPlayExit>,
            ).chain(),
        );
    }
}

fn handle_keybindings(
    mut active_board: ResMut<ActiveBoardId>,
    mut tile_query: Query<&TileToken>,
    mut actions: EventWriter<CreateRobot>,
    mut next_play_menu_state: ResMut<NextState<PlayMenuState>>,
    play_menu_state: Res<State<PlayMenuState>>,
    input: Res<Input<KeyCode>>,
    board_database: Res<BoardDatabase>,
) {
    if input.just_pressed(KeyCode::Escape) {
        if *play_menu_state.get() == PlayMenuState::Open {
            next_play_menu_state.set(PlayMenuState::Closed);
        } else {
            next_play_menu_state.set(PlayMenuState::Open);
        }
    }

    if input.just_pressed(KeyCode::F1) && !active_board.is(board_database.board_ids[0]) {
        active_board.0 = board_database.board_ids[0];
    }
    if input.just_pressed(KeyCode::F2) && !active_board.is(board_database.board_ids[1]) {
        active_board.0 = board_database.board_ids[1];
    }

    if input.just_pressed(KeyCode::F3) {
        let mut generator = thread_rng();
        let prototype = ["barker", "microwave", "magic_missile"]
            .choose(&mut generator)
            .unwrap();
        if let Some(random_tile) = tile_query.iter_mut().choose(&mut generator) {
            actions.send(CreateRobot {
                board_id: active_board.0,
                prototype_id: prototype.to_string(),
                x: random_tile.x,
                y: random_tile.y,
            });
        }
    }
}

fn ensure_play_menu_closed(
    mut next_play_menu_state: ResMut<NextState<PlayMenuState>>,
    play_menu_state: Res<State<PlayMenuState>>,
) {
    if *play_menu_state.get() == PlayMenuState::Open {
        next_play_menu_state.set(PlayMenuState::Closed);
    }
}

#[cfg(test)]
mod tests {
    use crate::test_utils::tests::*;

    use super::*;

    #[test]
    fn test_ensure_play_menu_closed() {
        let mut app = App::new();
        app.add_state::<PlayMenuState>();
        app.add_systems(Update, ensure_play_menu_closed);
        assert_state!(&mut app, PlayMenuState::Closed);
        app.update();
        assert_state!(&mut app, PlayMenuState::Closed);
        set_next_state(&mut app, PlayMenuState::Open);
        app.update();
        assert_state!(&mut app, PlayMenuState::Open);
        app.update();
        assert_state!(&mut app, PlayMenuState::Closed);
    }
}
