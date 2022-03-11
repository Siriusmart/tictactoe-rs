use console::Term;
use std::{
    error::Error,
    io::{self, Write},
};
use tictactoe::*;

fn main() -> Result<(), Box<dyn Error>> {
    let stdin = io::stdin();
    let mut stdout = io::stdout();
    let term = Term::stdout();

    let mut p1name = String::new();
    let mut p2name = String::new();

    term.clear_screen()?;

    print!("Enter player 1 name: ");
    stdout.flush()?;
    stdin.read_line(&mut p1name)?;

    print!("Enter player 2 name: ");
    stdout.flush()?;
    stdin.read_line(&mut p2name)?;

    let mut board = Board::new(
        &mut p1name.trim().to_string(),
        &mut p2name.trim().to_string(),
    );

    loop {
        term.clear_screen()?;
        print!(
            "[{}] {}'s turn!\n\n{}\n\nPlease enter the corresponding number you want to place: ",
            board.this_shape().to_string(),
            board.round_player().name(),
            board.display()
        );
        stdout.flush()?;
        let mut userinput = String::new();
        stdin.read_line(&mut userinput)?;
        let index: usize = match userinput.trim().parse::<usize>() {
            Ok(n) => n,
            Err(e) => {
                eprintln!("{}", e);
                press_enter_to_continue();
                continue;
            }
        };

        if let Err(e) = board.place(index) {
            eprintln!("{}", e);
            press_enter_to_continue();
            continue;
        }

        if let Some(player) = board.check() {
            term.clear_screen()?;
            println!(
                "[{}] {} has won the game!\n\n{}\n",
                board.this_shape().other().to_string(),
                player.name(),
                board.display()
            );
            break;
        }
    }

    Ok(())
}

fn press_enter_to_continue() {
    let stdin = io::stdin();
    let mut stdout = io::stdout();

    let mut wait = String::new();
    print!("Press enter to continue...\n\n");
    stdout.flush().unwrap();
    stdin.read_line(&mut wait).unwrap();
}
