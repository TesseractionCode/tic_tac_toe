use std::io::{stdin, Stdin};

// TEST CHANGE

mod gameboard;

use gameboard::*;

/// Take a usize from the user in the console.
fn input_usize(cin: &Stdin) -> usize {
    let mut input = String::new();
    cin.read_line(&mut input).ok();

    let num = input.trim().parse::<usize>().unwrap();

    num
}

/// Ask the user to enter a location in the console and add a player to the board at that location.
fn add_player_with_input(cin: &Stdin, player: Square, board: &mut GameBoard) -> Result<bool, String> {
    println!("({}) Enter the row and column numbers where you would like to place.", player.to_string());

    println!("Row: ");
    let row = input_usize(cin);
    println!("Column: ");
    let column = input_usize(cin);

    board.place_player(player, row, column)
}

fn run_game() {
    let mut board = GameBoard::new(3);

    println!("{}", board.to_string());
    println!();
    

    let cin = stdin();

    let mut player_switch = false;
    loop {
        let player = if player_switch {
            Square::O
        } else {
            Square::X };

        let board_result = add_player_with_input(&cin, player, &mut board);
        
        match board_result {
            Ok(is_win) => {
                if is_win {
                    println!("\n{} won!!", player.to_string());
                    println!("\n{}", board.to_string());
                    input_usize(&cin);
                    return
                } player_switch = !player_switch;
            },
            Err(err_msg) => {println!("Try Again! {err_msg}");}
        }
        println!("{}", board.to_string());

        println!();
        if board.is_full() {
            println!("Draw!")
        }
    }

    
}

fn main() {
    run_game();
}
