pub mod game_base {
    use std::{fmt, ops::AddAssign};
    
    const LOGO: &str = "     ____                            _     _____                     _     _     
    / ___|___  _ __  _ __   ___  ___| |_  |  ___|__  _   _ _ __     (_)___| |__  
   | |   / _ \\| '_ \\| '_ \\ / _ \\/ __| __| | |_ / _ \\| | | | '__|____| / __| '_ \\ 
   | |__| (_) | | | | | | |  __/ (__| |_  |  _| (_) | |_| | | |_____| \\__ \\ | | |
    \\____\\___/|_| |_|_| |_|\\___|\\___|\\__| |_|  \\___/ \\__,_|_|       |_|___/_| |_|\n\n";
    
    /* 
     * Player:
     *   An enum used to define all different in-game players
     */
    #[derive(Clone)]
    #[derive(Copy)]
    #[derive(PartialEq)]
    pub enum Player {
        One,
        Two,
        Three,
        Four
    }
    impl fmt::Debug for Player {
        // Binds each player to a token character
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            write!(f, "{}", match self {
                Player::One => 'X',
                Player::Two => 'O',
                Player::Three => '/',
                Player::Four => '\\',
            })
        }
    }
    impl Player {
        fn next_player(&self) -> Player {
            match self {
                Player::One => Player::Two,
                Player::Two => Player::Three,
                Player::Three => Player:: Four,
                Player::Four => Player::One,
            }
        }

        fn player_index(&self) -> usize {
            match self {
                Player::One => 0,
                Player::Two => 1,
                Player::Three => 2,
                Player::Four => 3,
            }
        }
    }

    /*
     * GameState:
     *   A struct used to hold and store all data related to a game instance
     */
    pub struct GameState {
        grid: Vec<Vec<Option<Player>>>, // contains all the player tokens
        rows: usize, // contains the height of the playing grid
        cols: usize, // contains the width of the playing grid

        players: usize, // contains the number of players for the game
        humans: usize, // contains the number of human players for the game
        current_player: Player, // shows who's turn it is currently
        current_turn: usize, // shows the current count for turns taken

        win_length: usize, // the length of the sequence required to win
        winner: Option<Player>, // if Some, then there is a winner
        game_over: bool, // represents whether the game is over
    }
    impl fmt::Display for GameState {
        // Prints out all information held by the GameState struct
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result  {
            let mut game_board_string = String::new();

            for (row_index, row) in self.grid.iter().enumerate() {
                if row_index == 0 {
                    for value in row.iter().enumerate() {
                        game_board_string.push_str(
                            format!("._{:?}_.", value.0 + 1).as_str()
                        );
                    }
                    game_board_string.push('\n');
                }
                for cell in row.iter() {
                    if let Some(token) = &cell {
                        game_board_string.push_str(
                            format!("[ {:?} ]", token).as_str()
                        );
                    } else {
                        game_board_string.push_str("[   ]");
                    }
                }
                game_board_string.push('\n');
            }

            if self.game_over {
                if let None = self.winner {
                    game_board_string.push_str(format!(
                        "\nThe board's full, it's a tie!\n"
                    ).as_str());
                } else {
                    game_board_string.push_str(format!(
                        "\n{:?} won on turn {:?}!",
                        self.winner.unwrap(),
                        self.current_turn + 1,
                    ).as_str());
                }
            } else {
                game_board_string.push_str(format!(
                    "\nTurn {:?},\n", 
                    self.current_turn, 
                    self.current_player,
                ).as_str());
                for (i, player) in self.players. {
                    if player == self.current_player {
                        let display = format!("[{:?}]", self.current_player).as_str();
                    } else if player
                    game_board_string.push_str(format!(
                        ""
                    ).as_str());
                }
            }

            write!(f, "{}", game_board_string)
        }
    }
    impl GameState {
        // Creates a new board, first player specified
        fn new_board(
            _rows: usize,
            _cols: usize,
            _players: usize,
            _humans: usize,
            _win_length: usize,
        ) -> GameState {
            GameState {
                grid: vec![vec![None; _cols]; _rows],
                rows: _rows,
                cols: _cols,
                players: _players,
                humans: _humans,
                current_player: Player::One,
                current_turn: 0,
                win_length: _win_length,
                winner: None,
                game_over: false,
            }
        }
        // places a token on the board
        fn place_token(&mut self, col: usize) -> Result<bool, String> {
            if col < 1 || col > self.cols {
                let error_str = format!("Please choose an available column, 1-{:?}.", self.cols);
                Err(error_str)
            } else {
                for (row_index, row) in self.grid.iter_mut().enumerate().rev() {
                    if row[col - 1].is_none() {
                        row[col - 1] = Some(self.current_player);
                        match self.check_for_win(row_index, col - 1) {
                            true => {
                                self.winner = Some(self.current_player);
                                self.game_over = true;
                                return Ok(true);
                            },
                            _ => { 
                                if self.next_turn() >= self.cols * self.rows {
                                    self.game_over = true;
                                    return Ok(true);
                                }
                                return Ok(false);
                            },
                        };
                    }
                }
                Err("Column full! please choose somewhere else!".to_string())
            }
        }
        // Iterates the turn count up one, and switches the active player
        fn next_turn(&mut self) -> usize {
            self.current_player = self.current_player.next_player();
            if self.current_player.player_index() >= self.players {
                self.current_player = Player::One;
            }
            self.current_turn += 1;
            self.current_turn
        }
        // Checks for a win condition on the grid
        fn check_for_win(&self, row: usize, col: usize) -> bool {

            // Check |
            if self.rows - row >= self.win_length.into() {
                if self.count_tokens(row, col, 1, 0) >= self.win_length {
                    return true;
                }
            }

            // Check /
            if self.count_tokens(row, col, 1, -1) 
            + self.count_tokens(row, col, -1, 1) > self.win_length {
                return true;
            }

            // Check \
            if self.count_tokens(row, col, 1, 1) 
            + self.count_tokens(row, col, -1, -1) > self.win_length {
                return true;
            }

            //Check -
            if self.count_tokens(row, col, 0, -1) 
            + self.count_tokens(row, col, 0, 1) > self.win_length {
                return true;
            }

            false
        }
        // counts the tokens in a given direction. 
        // row/col_dir use 1, -1, or 0 as multipliers
        fn count_tokens(
            &self, 
            row: usize, 
            col: usize, 
            row_dir: isize, 
            col_dir: isize
        ) -> usize {
            let mut count: usize = 1;
            loop {
                let new_row = match row_dir {
                    -1 => row.checked_sub(count).unwrap_or_else(|| self.cols),
                    1 => row.checked_add(count).unwrap_or_else(|| self.cols),
                    _ => row,
                };
                let new_col = match col_dir {
                    -1 => col.checked_sub(count).unwrap_or_else(|| self.cols),
                    1 => col.checked_add(count).unwrap_or_else(|| self.cols),
                    _ => col,
                };
                if self.grid.get(new_row).is_some() {
                    if self.grid[new_row].get(new_col).is_some() {
                        if Some(self.current_player) == self.grid[new_row][new_col] {
                            count += 1;
                            continue;
                        }
                    }
                }
                break;
            }
            count
        }
    }

    pub fn start_game() {
        if let Err(msg) = super::console::clear_console() {
            println!("{}", msg)
        }
        println!("{}Welcome to Connect Four-ish! 
We're gonna ask a couple questions to get started :)\n", LOGO);
        let get_msg = |min: usize, max: usize| -> String { format!("\n(between {} and {}, please)\n", min, max) };
        let err = |min: usize, max: usize| -> String { format!("\nBetween {} and {}, please.\n", min, max) };

        // get players
        let players = super::console::prompt_user(
            String::from("How many players should there be?") + &msg_end(1, 4),
            err(1, 4),
            1,
            4
        );
                
        // get humans
        let humans = super::console::prompt_user(
            String::from("How many humans should there be?") + &msg_end(1, 4), 
            err(1, players),
            1, 
            players
        );

        // get rows
        let rows = super::console::prompt_user(
            String::from("How many rows should the board have?") + &msg_end(1, 9), 
            err(1, 9),
            1, 
            9
        );

        // get cols
        let cols = super::console::prompt_user(
            String::from("How many columns should the board have?") + &msg_end(1, 9), 
            err(1, 9),
            1, 
            9
        );

        // get sequence
        let seq_max = if rows >= cols { cols } else { rows };
        let sequence = super::console::prompt_user(
            String::from("How long should a winning sequence be?") + &msg_end(1, seq_max), 
            err(1, seq_max),
            1, 
            seq_max
        );

        if let Err(msg) = super::console::clear_console() {
            println!("{}", msg)
        }

        game_loop(GameState::new_board(rows, cols, players, humans, sequence));
    }

    // The loop that defines turn-by-turn behavior
    fn game_loop(mut game_state: GameState) {
        //if let Err(msg) = super::console::clear_console() {
        //    println!("{}", msg)
        //}
        println!("{}\n\n{}", LOGO, game_state);
        loop {

            if loop {
                let choice = super::console::prompt_user("".to_string(), "".to_string(), 1, game_state.cols);
        
                match game_state.place_token(choice) {
                    Ok(win) => {
                        if let Err(msg) = super::console::clear_console() {
                            println!("{}", msg)
                        }
                        println!("{}\n\n{}", LOGO, game_state);
                        break win;
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
}

pub mod console {
    use std::io;

    // clears the contents of the command line
    pub fn clear_console() -> Result<(), String> {
        if std::process::Command::new("cls")
            .status()
            .or_else(|_| std::process::Command::new("clear").status())
            .unwrap()
            .success() { 
                return Ok(()); 
            }
        Err("Failed to clear screen. Sorry :(".to_string())
    }

    // gets an integer from a user's input
    fn get_integer_input<'a>() -> Result<usize, String> {

        let mut input = String::new();

        io::stdin().read_line(&mut input)
            .expect("Failed to read line");

        match input
            .trim()
            .parse() {
                Ok(num) => Ok(num),
                Err(_) => Err("Please input a positive integer!".to_string()),
        }
    }

    // prompts the user with given text, accepting a response within the specified range
    pub fn prompt_user(msg: String, err: String, min: usize, max: usize) -> usize {
        if !msg.is_empty() {
            println!("{}", msg);
        }
        loop {
            match get_integer_input() {
                Ok(val) => {
                    if val >= min && val <= max {
                        break val;
                    } else {
                        if !err.is_empty() {
                            println!("{}", err);
                        }
                        continue;
                    }
                }
                Err(info) => {
                    println!("{}", info); 
                    continue;
                }
            }
        }
    }
}