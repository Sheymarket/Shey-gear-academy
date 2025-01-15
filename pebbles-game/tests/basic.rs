use gstd::Encode;
use gtest::{Program, System};
use pebbles_game_io::*;

const USER: u64 = 10001;

#[test]
fn test_game_initialization() {
    let sys = System::new();

    sys.init_logger();

    let program = Program::from_file(
        &sys,
        "./target/wasm32-unknown-unknown/debug/pebbles_game.opt.wasm",
    );

    sys.mint_to(USER, 500000000000000000);
    assert_eq!(sys.balance_of(USER), 500000000000000000);

    let pebbles_init: PebblesInit = PebblesInit {
        pebbles_count: 10,
        max_pebbles_per_turn: 3,
        difficulty: DifficultyLevel::Easy,
    };

    let _res = program.send_bytes(USER, pebbles_init.encode());

    let game: GameState = program.read_state(()).expect("Failed to read state");

    assert_eq!(game.pebbles_count, pebbles_init.pebbles_count);
    assert_eq!(game.max_pebbles_per_turn, pebbles_init.max_pebbles_per_turn);
    assert_eq!(game.pebbles_remaining, pebbles_init.pebbles_count);
    assert!(game.winner.is_none());
}

#[test]
fn test_player_turn() {
    let system = System::new();
    let program = Program::current(&system);

    system.mint_to(USER, 500000000000000000);
    assert_eq!(system.balance_of(USER), 500000000000000000);

    let pebbles_init = PebblesInit {
        pebbles_count: 10,
        max_pebbles_per_turn: 3,
        difficulty: DifficultyLevel::Easy,
    };
    let _ = program.send(USER, pebbles_init);

    let action = PebblesAction::Turn(3);
    let _ = program.send(USER, action);

    let game_state: GameState = program.read_state(()).expect("Failed to read state");
    assert_eq!(game_state.pebbles_remaining, 10 - 3);
    assert!(game_state.winner.is_none());
}

#[test]
fn test_program_turn() {
    let system = System::new();
    let program = Program::current(&system);

    system.mint_to(USER, 500000000000000000);
    assert_eq!(system.balance_of(USER), 500000000000000000);

    let pebbles_init = PebblesInit {
        pebbles_count: 10,
        max_pebbles_per_turn: 3,
        difficulty: DifficultyLevel::Easy,
    };
    let _ = program.send(USER, pebbles_init);

    let action = PebblesAction::Turn(3);
    let _ = program.send(USER, action);

    let game_state: GameState = program.read_state(()).expect("Failed to read state");
    assert_eq!(game_state.pebbles_remaining, 10 - 3);

    let action = PebblesAction::Turn(1);
    let _ = program.send(USER, action);

    let game_state: GameState = program.read_state(()).expect("Failed to read state");
    assert_eq!(game_state.pebbles_remaining, 10 - 3 - 1);
    assert!(game_state.winner.is_none());
}

#[test]
fn test_game_winner() {
    let system = System::new();
    let program = Program::current(&system);

    system.mint_to(USER, 500000000000000000);
    assert_eq!(system.balance_of(USER), 500000000000000000);

    let pebbles_init = PebblesInit {
        pebbles_count: 3,
        max_pebbles_per_turn: 3,
        difficulty: DifficultyLevel::Easy,
    };
    let _ = program.send(USER, pebbles_init);

    let action = PebblesAction::Turn(3);
    let _ = program.send(USER, action);

    let game_state: GameState = program.read_state(()).expect("Failed to read state");
    assert_eq!(game_state.pebbles_remaining, 0);
    assert_eq!(game_state.winner, Some(Player::User));
}

#[test]
fn test_game_restart() {
    let system = System::new();
    let program = Program::current(&system);

    system.mint_to(USER, 500000000000000000);
    assert_eq!(system.balance_of(USER), 500000000000000000);

    let pebbles_init = PebblesInit {
        pebbles_count: 10,
        max_pebbles_per_turn: 3,
        difficulty: DifficultyLevel::Easy,
    };
    let _ = program.send(USER, pebbles_init);

    let action = PebblesAction::Turn(3);
    let _ = program.send(USER, action);

    let restart_action = PebblesAction::Restart {
        difficulty: DifficultyLevel::Easy,
        pebbles_count: 10,
        max_pebbles_per_turn: 3,
    };
    let _ = program.send(USER, restart_action);

    let game_state: GameState = program.read_state(()).expect("Failed to read state");
    assert_eq!(game_state.pebbles_remaining, 10);
    assert!(game_state.winner.is_none());
}
