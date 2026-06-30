mod tictactoe;

use std::io::{self, Write};

use tictactoe::{GameState, Position};

fn main() {
    let mut game = GameState::default();

    loop {
        println!("{game}");

        if let Some(player) = game.get_next_player() {
            println!("Player {player}, enter your turn (A1-C3):");
        }

        print!("> ");
        io::stdout().flush().expect("failed to flush stdout");

        let mut input = String::new();
        if let Err(error) = io::stdin().read_line(&mut input) {
            eprintln!("Could not read input: {error}");
            continue;
        }

        let position = match input.trim().parse::<Position>() {
            Ok(position) => position,
            Err(error) => {
                println!("{error}");
                continue;
            }
        };

        match game.make_turn(position) {
            Ok(Some(result)) => {
                println!("{game}");
                println!("{result}");
                break;
            }
            Ok(None) => {}
            Err(error) => println!("{error}"),
        }
    }
}
