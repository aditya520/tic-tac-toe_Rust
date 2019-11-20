use std::io;

type Board = Vec<Vec<String>>;

enum Turn{
    Player,
    Bot,
}

pub struct Game {
    boards: Board,
    current_turn: Turn,
}

impl Game {
    pub fn new() -> Game {
        let first_row = vec![String::from("1"), String::from("2"), String::from("3")];
        let second_row = vec![String::from("4"), String::from("5"), String::from("6")];
        let third_row = vec![String::from("7"), String::from("8"), String::from("9")];
        
        Game {
            board: vec![first_row,second_row,third_row],
            current_turn: Turn::Player,
        }
    }

    pub fn play_game(&mut self) {
        let mut finished = false;

        while !finished {
            self.play_turn();

            if self.game_is_won() {
                self.print_board();

                match self.current_turn {
                    Turn::Player => println!("You won!");
                    Turn::Bot => println!("You lost!");
                };

                finished = Self::player_is_finished();

                self.reset();
            }

            self.current_turn = self.get_next_turn();
        }
    }

    fn play_turn(&mut self) {
        self.print_board();

        let (token, valid_move) = match self.current_turn {
            Turn::Player => (String::from("X"), self.get_player_move()),
            Turn::Bot => (String::from("O"), self.get_bot_move()),
        };

        let (row, col) = Self::to_board_location(valid_move);

        self.board[row][col] = token;
    }

    fn print_board(&self) {
        let separator = "+---+---+---+";
     
        println!("\n{}", separator);
     
        for row in &self.board {
            println!("| {} |\n{}", row.join(" | "), separator);
        }
     
        print!("\n");
    }

    fn get_player_move(&self) -> u32 {
        loop {
            let mut player_input = String::new();
     
            println!(
                "\nPlease enter your move (an integer between \
                1 and 9): ");
     
            match io::stdin().read_line(&mut player_input) {
                Err(_) => println!(
                    "Error reading input, try again!"),
                Ok(_) => match self.validate(&player_input) {
                    Err(err) => println!("{}", err),
                    Ok(num) => return num,
                },
            }
        }
    }
}