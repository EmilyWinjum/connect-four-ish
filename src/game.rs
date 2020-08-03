pub mod game_base {
    use std::fmt;

    const ROWS: usize = 6;
    const COLS: usize = 7;
    const WIN_LENGTH: usize = 4;
    
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
    }
    impl fmt::Debug for Player {
        // Binds each player to a token character
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            write!(f, "{}", match self {
                Player::One => 'X',
                Player::Two => 'O',
            })
        }
    }

    /*
    * GameState:
    *   A struct used to hold and store all data related to a game instance
    */
    pub struct GameState {
        board: Vec<Vec<Option<Player>>>, // contains all the player tokens
        pub current_player: Player, // shows who's turn it is currently
        current_turn: usize, // shows the current count for turns taken
        winner: Option<Player>, // if Some, then there is a winner
        game_over: bool, // represents whether the game is over
    }
    impl fmt::Display for GameState {
        // Prints out all information held by the GameState struct
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result  {
            let mut game_board_string = String::new();

            for (index, row) in self.board.iter().enumerate() {
                if index == 0 {
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
                    "\nTurn {:?}, \n{:?} to move.\n", 
                    self.current_turn, 
                    self.current_player
                ).as_str());
            }

            write!(f, "{}", game_board_string)
        }
    }
    impl GameState {
        // Creates a new board, first player specified
        pub fn new_board(first_player: Player) -> GameState {
            GameState {
                board: vec![vec![None; COLS]; ROWS],
                current_player: first_player,
                current_turn: 0,
                winner: None,
                game_over: false,
            }
        }
        // places a token on the board
        pub fn place_token(&mut self, col: usize) -> Result<bool, &str> {
            if col < 1 || col > COLS {
                Err("Please choose an available column, 1-7.")
            } else {
                for (index, row) in self.board.iter_mut().enumerate().rev() {
                    if row[col - 1].is_none() {
                        row[col - 1] = Some(self.current_player);
                        match self.check_for_win(index, col - 1) {
                            true => {
                                self.winner = Some(self.current_player);
                                self.game_over = true;
                                return Ok(true);
                            },
                            _ => { 
                                if self.next_turn() >= COLS * ROWS {
                                    self.game_over = true;
                                    return Ok(true);
                                }
                                return Ok(false);
                            },
                        };
                    }
                }
                Err("Column full! please choose somewhere else!")
            }
        }
        // Iterates the turn count up one, and switches the active player
        fn next_turn(&mut self) -> usize {
            match self.current_player {
                Player::One => self.current_player = Player::Two,
                Player::Two => self.current_player = Player::One,
            }
            self.current_turn += 1;
            self.current_turn
        }
        pub fn check_for_win(&self, row: usize, col: usize) -> bool {

            // Check |
            if ROWS - row >= WIN_LENGTH.into() {
                if self.count_tokens(row, col, 1, 0) >= WIN_LENGTH {
                    return true;
                }
            }

            // Check /
            if self.count_tokens(row, col, 1, -1) 
            + self.count_tokens(row, col, -1, 1) > WIN_LENGTH {
                return true;
            }

            // Check \
            if self.count_tokens(row, col, 1, 1) 
            + self.count_tokens(row, col, -1, -1) > WIN_LENGTH {
                return true;
            }

            //Check -
            if self.count_tokens(row, col, 0, -1) 
            + self.count_tokens(row, col, 0, 1) > WIN_LENGTH {
                return true;
            }

            false
        }
        // counts the tokens in a given direction. 
        // row/col_mult use 1, -1, or 0 as multipliers
        fn count_tokens(
            &self, 
            row: usize, 
            col: usize, 
            row_mult: isize, 
            col_mult: isize
        ) -> usize {
            let mut count: usize = 1;
            loop {
                let new_row = match row_mult {
                    -1 => row.checked_sub(count).unwrap_or_else(|| 8),
                    1 => row.checked_add(count).unwrap_or_else(|| 8),
                    _ => row,
                };
                let new_col = match col_mult {
                    -1 => col.checked_sub(count).unwrap_or_else(|| 8),
                    1 => col.checked_add(count).unwrap_or_else(|| 8),
                    _ => col,
                };
                if self.board.get(new_row).is_some() {
                    if self.board[new_row].get(new_col).is_some() {
                        if Some(self.current_player) == self.board[new_row][new_col] {
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
    // gets an integer from a user's input, within the specified range.
    pub fn get_integer_input<'a>() -> Result<usize, &'a str> {

        let mut input = String::new();

        io::stdin().read_line(&mut input)
            .expect("Failed to read line");

        let input: Result<usize, &str> = match input
            .trim()
            .parse() {
                Ok(num) => Ok(num),
            Err(_) => Err("Please input a number!"),
        };

        input
    }
}