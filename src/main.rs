mod game;

use crate::game::*;

fn main() {
    let mut game_state = game_base::GameState::new_board(game_base::Player::One);
    console::clear_console().expect("Line read failed!");
    println!("{}", game_state);
    loop {

        if !loop {
            let choice = match console::get_integer_input() {
                Ok(val) => val,
                Err(info) => {
                    println!("{}", info); 
                    continue;
                }
            };
        
    
            match game_state.place_token(choice) {
                Ok(win) => {
                    console::clear_console().expect("Line read failed!");
                    println!("{}", game_state);
                    break !win;
                },
                Err(info) => {
                    println!("{}", info);
                    continue;
                },
            };
        } {
            break;
        }

    }
}
