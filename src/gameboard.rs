use std::borrow::Borrow;

#[derive(Copy, Clone, Debug, PartialEq)]
pub enum Square {
    EMPTY,
    X,
    O,
}

impl ToString for Square {
    fn to_string(&self) -> String {
        match self {
            Self::EMPTY => "_",
            Self::X => "X",
            Self::O => "O",
        }.to_string()
    }
}

pub struct GameBoard {
    board: Vec<Square>,
    size: usize
}

impl GameBoard {
    /// A board size of 3 gives a 3x3 board.
    pub fn new(board_size: usize) -> Self {
        GameBoard {board: vec![Square::EMPTY; board_size.pow(2)], size: board_size}
    } 

    /// Places a player on the board and returns whether they won or not.
    pub fn place_player(&mut self, player: Square, row_index: usize, column_index: usize) -> Result<bool, String> {
        if !self.is_on_board(row_index, column_index) {
            return Err("The given location is off of the board.".to_string())
        }

        match self.get_square(row_index, column_index) {
            Square::EMPTY => {
                self.board[row_index * self.size + column_index] = player;
                Ok(self.is_player_won(row_index, column_index))
            }
            _ => Err("There was already a player in the placed location.".to_string())
        }
    }

    /// Determines if the player at a given location won.
    fn is_player_won(&self, row_index: usize, column_index: usize) -> bool{
        // Get the player type that was placed.
        let player = self.get_square(row_index, column_index);

        let mut is_row_fail = false;
        let mut is_column_fail = false;
        let mut is_diagonal_fail = false;
        let mut is_anti_diagonal_fail = false;
        for i in 0..self.size {
            // Row check
            if self.get_square(row_index, i).ne(player.borrow()) {
                is_row_fail = true;
            }
            // Column check
            if self.get_square(i, column_index).ne(player.borrow()) {
                is_column_fail = true;
            }
            // Diagonal check
            if self.get_square(i, i).ne(player.borrow()) {
                is_diagonal_fail = true;
            }
            // Anti diagonal check
            if self.get_square(i, self.size - i - 1).ne(player.borrow()) {
                is_anti_diagonal_fail = true;
            }
        }

        !(is_row_fail && is_column_fail && is_diagonal_fail && is_anti_diagonal_fail)
    }

    /// Determines if the given location is a valid location on the board.
    fn is_on_board(&self, row_index: usize, column_index: usize) -> bool {
        let acceptable_range = 0..(self.size * 2 - 1);
        acceptable_range.contains(&(row_index + column_index))
    }

    /// Gets the Square enum at the given location.
    fn get_square(&self, row_index: usize, column_index: usize) -> Square {
        // Ensure that the entered indices is within the bounds of the board.
        assert!(self.is_on_board(row_index, column_index));

        self.board[row_index * self.size + column_index]
    }

    /// Return whether the board is full
    pub fn is_full(&self) -> bool {
        for i in 0..self.board.len() {
            if self.board[i].to_string() != "_" {
                return false
            }
        }
        true
    }

}

impl ToString for GameBoard {
    fn to_string(&self) -> String {
        let mut string = String::new();
        for row in 0..self.size {
            string.push_str(&format!("Row {}: |", row));
            for column in 0..self.size {
                let square_name = self.get_square(row, column).to_string();
                string.push_str(&format!(" {} ", &square_name));
            }
            string.push_str("|\n");
        }

        string
    }
}