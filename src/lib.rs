use rand::prelude::*;

#[derive(Clone, PartialEq)]
pub struct Player(String);

impl Player {
    fn new(name: &String) -> Player {
        Player(name.clone())
    }

    pub fn name(&self) -> &String {
        &self.0
    }
}

enum Round {
    Player1,
    Player2,
}

impl Round {
    fn random() -> Round {
        if random() {
            Round::Player1
        } else {
            Round::Player2
        }
    }

    fn other(&mut self) -> Round {
        match *self {
            Round::Player1 => Round::Player2,
            Round::Player2 => Round::Player1,
        }
    }
}

pub struct Board {
    player1: Player,
    player2: Player,
    round: Round,
    board: [Option<Player>; 9],
}

impl Board {
    pub fn new(player1: &String, player2: &String) -> Board {
        const NONE: Option<Player> = None;

        Board {
            player1: Player::new(player1),
            player2: Player::new(player2),
            round: Round::random(),
            board: [NONE; 9],
        }
    }

    pub fn check(&self) -> Option<Player> {
        if self.board[0].is_some()
            && ((self.board[0] == self.board[1] && self.board[1] == self.board[2])
                || (self.board[0] == self.board[3] && self.board[3] == self.board[6]))
        {
            return self.board[0].clone();
        }

        if self.board[8].is_some()
            && ((self.board[8] == self.board[7] && self.board[7] == self.board[6])
                || (self.board[8] == self.board[5] && self.board[5] == self.board[2]))
        {
            return self.board[8].clone();
        }

        if self.board[4].is_some()
            && ((self.board[0] == self.board[4] && self.board[4] == self.board[8])
                || (self.board[1] == self.board[4] && self.board[4] == self.board[7])
                || (self.board[2] == self.board[4] && self.board[4] == self.board[6])
                || (self.board[3] == self.board[4] && self.board[4] == self.board[5]))
        {
            return self.board[5].clone();
        }

        None
    }

    pub fn round_player(&self) -> Player {
        match self.round {
            Round::Player1 => self.player1.clone(),
            Round::Player2 => self.player2.clone(),
        }
    }

    pub fn place(&mut self, mut index: usize) -> Result<(), &str> {
        index -= 1;
        if index > self.board.len() - 1 {
            return Err("index out of bounds");
        }
        if self.board[index].is_none() {
            self.board[index] = Some(self.round_player());
            self.round = self.round.other();
            Ok(())
        } else {
            Err("position alread occupied")
        }
    }

    pub fn display(&self) -> String {
        let mut index: u8 = 0;
        let mapped_board = self.board.clone().map(|item| {
            index += 1;
            match item {
                Some(player) => {
                    if player == self.player1 {
                        String::from("O")
                    } else {
                        String::from("X")
                    }
                }
                None => index.to_string(),
            }
        });
        format!("   |   |   \n {} | {} | {} \n   |   |   \n———+———+———\n   |   |   \n {} | {} | {} \n   |   |   \n———+———+———\n   |   |   \n {} | {} | {} \n   |   |   ", mapped_board[0], mapped_board[1], mapped_board[2], mapped_board[3], mapped_board[4], mapped_board[5], mapped_board[6], mapped_board[7], mapped_board[8])
    }

    pub fn this_shape(&self) -> String {
        match self.round {
            Round::Player1 => String::from("O"),
            Round::Player2 => String::from("X"),
        }
    }
}
