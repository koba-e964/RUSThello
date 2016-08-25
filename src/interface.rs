//! This module provides interface functionalities and manages all the input/output part of the program

use std::string::String;
use std::cmp::Ordering;
use std::io::{self, Write};
// use player;
use reversi::{Side};
use reversi::board::{BOARD_SIZE, Coord};
use reversi::game::{PlayerAction};
use reversi::turn::{State, Turn};
use ::{Result, Action, OtherAction};

// ASCII version
#[cfg(target_os = "redox")]
const DARK_DISK:  char = 'X';
#[cfg(target_os = "redox")]
const LIGHT_DISK: char = 'O';
#[cfg(target_os = "redox")]
const EMPTY_CELL: char = '-';
#[cfg(target_os = "redox")]
const LEGAL_MOVE: char = ' ';

// ANSI version
#[cfg(not(target_os = "redox"))]
const DARK_DISK:  char = '●';
#[cfg(not(target_os = "redox"))]
const LIGHT_DISK: char = '○';
#[cfg(not(target_os = "redox"))]
const EMPTY_CELL: char = '∙';
#[cfg(not(target_os = "redox"))]
const LEGAL_MOVE: char = '*';

pub enum UserCommand {
	NewGame,
	HumanPlayer,
	AiWeak,
	AiMedium,
	AiStrong,
	Help,
	Credits,
	Quit,
}

const INTRO: &'static str = "\n\n
\t-------------------------
\t------- RUSThello -------
\t-------------------------
\t  a simple Reversi game
\twritten in Rust with love";

#[cfg(target_os = "redox")]
pub fn intro() {
	println!("{}", INTRO);
	println!("\t  Redox Edition");
	println!("\t        v. {}", env!("CARGO_PKG_VERSION"));
}

#[cfg(not(target_os = "redox"))]
pub fn intro() {
	println!("{}", INTRO);
	println!("\t        v. {}", env!("CARGO_PKG_VERSION"));
}

const MAIN_MENU: &'static str =
"\n\n
\t-------------------------
\t------- MAIN MENU -------
\t-------------------------
\tn - New match
\th - Help
\tc - Credits
\tq - Quit RUSThello
\t-------------------------";

pub fn main_menu() {
	println!("{}", MAIN_MENU);
}

const NEW_PLAYER_MENU: &'static str =
"\n\n
\t-------------------------
\t---- CHOOSE A PLAYER ----
\t-------------------------
\th - Human Player
\tw - Weak   AI
\tm - Medium AI
\ts - Strong AI
\tq - Quit match
\t-------------------------";

pub fn new_player_menu() {
	println!("{}", NEW_PLAYER_MENU);
}

const COMMANDS_INFO: &'static str =
"\n\n
\tStarting new game...
\tType a cell's coordinates to place your disk there.
\tExaple: \"c4\" (or \"C4\", \"4c\", \"4C\", etc...).
\tType 'help' or 'h' to display a help message.
\tType 'undo' or 'u' to undo the last move.
\tType 'quit' or 'q' to abandon the game.";

pub fn commands_info() {
	println!("{}", COMMANDS_INFO);
}

const HELP: &'static str = "\n\n
\t-------------------------
\t-------- REVERSI --------
\t-------------------------
\tReversi is a board game where two players compete against each other. \
The game is played on a 8x8 board, just like chess but for the squares’ colour which is always green. \
There are 64 identical pieces called disks, which are white on one side and black on the other. \
A player is Dark, using disks’ black side, and the other one is Light, using disks' white side. \
The game starts with four disks already placed at the centre of the board, two for each side. \
Dark moves first.\n
\tLet’s say it’s Dark’s turn, for simplicity's sake, as for Light the rules are just the same. \
Dark has to place a disk in a free square of the board, with the black side facing up. \
Whenever the newly placed black disk and any other previously placed black disk enclose a sequence of white disks (horizontal, vertical or diagonal and of any length), all of those flip and turn black. \
It is mandatory to place the new disk such that at least a white disk is flipped, otherwise the move is not valid.\n
\tUsually players’ turn alternate, passing from one to the other. \
When a player cannot play any legal move, the turn goes back to the other player, thus allowing the same player to play consecutive turns. \
When neither player can play a legal move, the game ends. \
Usually, this happens when the board is completely filled up with disks (for a total of 60 turns). \
Games also happen sometimes to end before that, leaving empty squares on the board.\n
\tWhen the game ends, the player with more disks turned to its side wins. \
Ties are possible as well, if both player have the same number of disks.
\t-------------------------\n\n
\t-------------------------
\t------- RUSThello -------
\t-------------------------
\tTo play RUSThello you first have to choose who is playing on each side, Dark and Light. \
You can choose a human players or an AI. \
Choose human for both players and challenge a friend, or test your skills against an AI, or even relax as you watch two AIs competing against each other: all combinations are possible!\n
\tAs a human player, you move by entering the coordinates (a letter and a number) of the square you want to place your disk on, e.g. all of 'c4', 'C4', '4c' and '4C' are valid and equivalent coordinates. \
For your ease of use, all legal moves are marked on the board by an asterisk.\n
\tFurthermore, on your turn you can also input special commands: 'undo' (or 'u') to undo your last move (and yes, you can 'undo' as many times as you like), 'help' (or 'h') to see this help message again, and 'quit' (or 'q') to quit the game.
\t-------------------------";

pub fn help() {
	println!("{}", HELP);
}

#[cfg(target_os = "redox")]
pub fn credits() {
	println!("\n\n
\t-------------------------
\t-------- CREDITS --------
\t-------------------------
\tRUSThello v. {} Redox Edition (https://redox-os.org/)
\tby Enrico Ghiorzi, with the invaluable help of the Redox community
\tCopyright (c) 2015-2016 by Enrico Ghiorzi
\tReleased under the MIT license
\t-------------------------", env!("CARGO_PKG_VERSION"));
}

#[cfg(not(target_os = "redox"))]
pub fn credits() {
	println!("\n\n
\t-------------------------
\t-------- CREDITS --------
\t-------------------------
\tRUSThello v. {}
\tby Enrico Ghiorzi
\tCopyright (c) 2015-2016
\tby Enrico Ghiorzi
\tReleased under MIT license
\t-------------------------", env!("CARGO_PKG_VERSION"));
}

/// Reads user's input
fn get_user_input() -> String {
	let _ = io::stdout().flush();
	let mut input = String::new();
	if io::stdin().read_line(&mut input).is_err() {
		panic!("\tFailed to read input!");
	}
	input = input.trim().to_lowercase();
	input
}

/// It gets an input from the user and tries to parse it, then returns a Option<UserCommand>`.
/// If the input is recognized as a legit command, it returns the relative `Option::Some(UserCommand)`.
/// If the input is not recognized as a legit command, it returns a `Option::None`.
pub fn input_main_menu() -> UserCommand {
	print!("\tInsert input: ");
	loop {
		match &*get_user_input() {
			"n" | "new game"			=> return UserCommand::NewGame,
			"h" | "help"				=> return UserCommand::Help,
			"c" | "credits"				=> return UserCommand::Credits,
			"q" | "quit" | "exit"		=> return UserCommand::Quit,
			_	=> {
				print!("\tInvalid command! Try again: ");
				continue;
			}
		}
	}
}

pub fn choose_new_player(side: Side) -> UserCommand {
	match side {
		Side::Dark  => print!("\t{} Dark  player: ",  DARK_DISK),
		Side::Light => print!("\t{} Light player: ", LIGHT_DISK),
	}
	loop {
		match &*get_user_input() {
			"h" | "human" | "player" | "human player"	=> return UserCommand::HumanPlayer,
			"w" | "weak" | "weak ai"					=> return UserCommand::AiWeak,
			"m" | "medium" | "medium ai"				=> return UserCommand::AiMedium,
			"s" | "strong" | "strong ai"				=> return UserCommand::AiStrong,
			"q" | "quit" | "exit"						=> return UserCommand::Quit,
			_	=> {
				print!("\tInvalid command! Try again: ");
				continue;
			}
		}
	}
}

/// It get_status a human player's input and convert it into a move.
/// If the move if illegal, it ask for another input until the given move is a legal one.
pub fn human_make_move(turn: &Turn) -> Result<Action> {

    if let Some(side) = turn.get_state() {
        match side {
			Side::Dark  => print!("\t{} Dark  moves: ", DARK_DISK),
            Side::Light => print!("\t{} Light moves: ", LIGHT_DISK),
        }
    } else {
		unreachable!();
	}

    loop {
		let input = &*get_user_input();
		match input {
			"h" | "help" 		=> return Ok(PlayerAction::Other(OtherAction::Help)),
			"u" | "undo" 		=> return Ok(PlayerAction::Undo),
			"q" | "quit" 		=> return Ok(PlayerAction::Other(OtherAction::Quit)),
			_other_input => {
				let mut row: Option<usize> = None;
				let mut col: Option<usize> = None;

				for curr_char in input.chars() {
					match curr_char {
						'1'...'8'	=> row = Some(curr_char as usize - '1' as usize),
						'a'...'h'	=> col = Some(curr_char  as usize - 'a' as usize),
						_			=> {}
					}
				}

				if row.is_none() || col.is_none() {
					print!("\tIllegal move, try again: ");
					continue;
				} else {
					let coord = Coord::new(row.unwrap(), col.unwrap());
					if turn.check_move(coord).is_ok() {
						return Ok(PlayerAction::Move(coord));
					} else {
						print!("\tIllegal move, try again: ");
						continue;
					}
				}
			}
		}
    }
}

/// draw_board draws the board (using text characters) in a pleasant-looking way, converting the board in a string (board_to_string) and then printing this.
pub fn draw_board(turn: &Turn) {
    let board = turn.get_board();
    // Declare board_to_string and add column reference at the top
    let mut board_to_string: String = "\n\t   A B C D E F G H\n".to_string();

    // For every row add a row reference to the left
    for row in 0..BOARD_SIZE {
		board_to_string.push_str(&format!("\t{}  ", row + 1));
        // For every column, add the appropriate character depending on the content of the current cell
        for col in 0..BOARD_SIZE {
			let coord = Coord::new(row, col);
			board_to_string.push(
	            match board.get_cell(coord).unwrap() {
	                // Light and Dark cells are represented by white and black bullets
	                Some(disk) => match disk.get_side() {
						Side::Dark  => DARK_DISK,
						Side::Light => LIGHT_DISK,
					},
	                // An empty cell will display a plus or a multiplication sign if the current player can move in that cell
	                // or a little central dot otherwise
	                None => {
						if turn.check_move(coord).is_ok() {
							LEGAL_MOVE
	                    } else {
							EMPTY_CELL
	                	}
					}
	            }
			);
			board_to_string.push(' ');
		}
        // Add a row reference to the right
		board_to_string.push_str(&format!(" {}\n", row + 1));
    }

    // Add column reference at the bottom
    board_to_string.push_str("\t   A B C D E F G H\n");

    // Print board
    println!("{}", board_to_string);

    // Print current score and game info
    let (score_dark, score_light) = turn.get_score();

    match turn.get_state() {
        Some(side) => {
            match side {
				Side::Dark  => println!("\t    {:>2} {} <<< {} {:<2}\n", score_dark, DARK_DISK, LIGHT_DISK, score_light),
                Side::Light => println!("\t    {:>2} {} >>> {} {:<2}\n", score_dark, DARK_DISK, LIGHT_DISK, score_light),
            }
        }
        None => {
            println!("\t    {:>2} {}     {} {:<2}\n", score_dark, DARK_DISK, LIGHT_DISK, score_light);
            match score_dark.cmp(&score_light) {
				Ordering::Greater	=> println!("\t{} Dark wins!", DARK_DISK),
                Ordering::Less		=> println!("\t{} Light wins!", LIGHT_DISK),
                Ordering::Equal		=> println!("\tTie!"),
            }
        }
    }
}

/// Prints a message with info on a move.
pub fn move_message(side: Side, coord: Coord) {
    let char_col = (('a' as u8) + (coord.get_col() as u8)) as char;
    match side {
		Side::Dark  => println!("\t{} Dark  moves: {}{}",  DARK_DISK, char_col, coord.get_row() + 1),
        Side::Light => println!("\t{} Light moves: {}{}", LIGHT_DISK, char_col, coord.get_row() + 1),
    }
}


// Print a last message before a player quits the game
pub fn quitting_message(state: State) {
    match state {
		Some(Side::Dark)  => println!("\tDark is running away, the coward!"),
        Some(Side::Light) => println!("\tLight is running away, the coward!"),
		None => println!("\n\tGoodbye!"),
    }
}

// Print a last message when 'undo' is not possible
pub fn no_undo_message(undecided: Side) {
	match undecided {
		Side::Dark  => println!("\tThere is no move Dark can undo."),
        Side::Light => println!("\tThere is no move Light can undo."),
    }
}
