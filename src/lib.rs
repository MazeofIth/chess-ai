// Import modules
//use rand::seq::SliceRandom;
use std::collections::HashMap;
use std::fmt;
use std::io;
use std::io::prelude::*;
extern crate stopwatch;
use stopwatch::{Stopwatch};
use std::time::{Duration, Instant};
use std::thread::sleep;

#[derive(Copy, Clone, Debug, PartialEq)]
pub enum GameState {
    InProgress,
    Check,
    GameOver,
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub enum Color {
    White,
    Black, 
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Piece {
    pub piecetype: PieceType,
    pub color: Color,
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub enum PieceType {
    Pawn,
    Rook,
    Queen,
    King,
    Knight,
    Bishop,
    Corpse,
}

// Defines a value() function for PieceTypes, which the AI uses
// This should be in the AI File
impl PieceType {
    pub fn value(&self) -> i32 {
        match *self {
            PieceType::Pawn => 1,
            PieceType::Knight => 3,
            PieceType::Bishop => 3,
            PieceType::Rook => 5,
            PieceType::Queen => 9,
            PieceType::King => 100,
            _ => 0,
        }
    }
}

pub struct Game {
    pub state: GameState,
    pub board: [[Option<Piece>; 8]; 8],
    pub color: Color,
    pub positions_evaluated: i32,
}

impl Game {
    /// Initialises a new game with a board and the starting color white
    pub fn new() -> Game {
        Game {
            state: GameState::InProgress,
            color: Color::White,
            board: Game::generate_board(),
            positions_evaluated: 0,
        }
    }

    fn generate_board() -> [[Option<Piece>; 8]; 8] {
        let mut currentboard = [[None; 8]; 8];
        let pieces = [
            PieceType::Rook,
            PieceType::Knight,
            PieceType::Bishop,
            PieceType::Queen,
            PieceType::King,
            PieceType::Bishop,
            PieceType::Knight,
            PieceType::Rook,
        ];

        // Fills the second and seventh row with pawns
        for i in 0..8 {
            currentboard[1][i] = Some(Piece {
                piecetype: PieceType::Pawn,
                color: Color::Black,
            });
            currentboard[6][i] = Some(Piece {
                piecetype: PieceType::Pawn,
                color: Color::White,
            });
        }

        // Fills the first and last row with the right pieces
        for i in 0..8 {
            currentboard[0][i] = Some(Piece {
                piecetype: pieces[i],
                color: Color::Black,
            });
            currentboard[7][i] = Some(Piece {
                piecetype: pieces[i],
                color: Color::White,
            });
        }

        //currentboard = [[Some(Piece { piecetype: PieceType::Rook, color: Color::Black }), None, Some(Piece { piecetype: PieceType::Bishop, color: Color::Black }), None, Some(Piece { piecetype: PieceType::King, color: Color::Black }), Some(Piece { piecetype: PieceType::Bishop, color: Color::Black }), Some(Piece { piecetype: PieceType::Knight, color: Color::Black }), None], [Some(Piece { piecetype: PieceType::Pawn, color: Color::Black }), Some(Piece { piecetype: PieceType::Pawn, color: Color::Black }), None, None, Some(Piece { piecetype: PieceType::Queen, color: Color::White }), Some(Piece { piecetype: PieceType::Pawn, color: Color::Black }), None, None], [None, None, Some(Piece { piecetype: PieceType::Knight, color: Color::Black }), None, None, None, None, None], [Some(Piece { piecetype: PieceType::Queen, color: Color::Black }), None, None, Some(Piece { piecetype: PieceType::Pawn, color: Color::Black }), None, None, Some(Piece { piecetype: PieceType::Pawn, color: Color::Black }), Some(Piece { piecetype: PieceType::Pawn, color: Color::Black })], [None, Some(Piece { piecetype: PieceType::Rook, color: Color::Black }), None, Some(Piece { piecetype: PieceType::Pawn, color: Color::White }), Some(Piece { piecetype: PieceType::Pawn, color: Color::White }), None, Some(Piece { piecetype: PieceType::Pawn, color: Color::White }), None], [Some(Piece { piecetype: PieceType::Bishop, color: Color::White }), None, Some(Piece { piecetype: PieceType::Pawn, color: Color::White }), None, None, None, None, Some(Piece { piecetype: PieceType::Pawn, color: Color::White })], [None, Some(Piece { piecetype: PieceType::Rook, color: Color::White }), None, None, None, Some(Piece { piecetype: PieceType::Pawn, color: Color::White }), None, Some(Piece { piecetype: PieceType::Rook, color: Color::White })], [None, Some(Piece { piecetype: PieceType::Knight, color: Color::White }), None, Some(Piece { piecetype: PieceType::King, color: Color::White }), None, Some(Piece { piecetype: PieceType::Bishop, color: Color::White }), Some(Piece { piecetype: PieceType::Knight, color: Color::White }), None]];
        currentboard
    }

    /*Convert between the string (e.g. "A1") and vector (e.g. [0, 1])
    representation of coordinates, because make_move uses strings as parameters*/
    fn convert_string_to_vec(_position: String) -> Vec<i8> {
        //let now = Instant::now();

        // Creates a hashmap with all letters associated to their position in the alphabet
        let mut coordinate_hashmap: HashMap<String, i8> = HashMap::new();
        let alphabet = vec!["A", "B", "C", "D", "E", "F", "G", "H"];
        for i in 0..alphabet.iter().count() {
            coordinate_hashmap.insert(alphabet[i].to_string(), i as i8);
        }

        // Get the first and second character, and then convert the second "letter" to an integer
        let first_letter = _position.chars().nth(0).unwrap();
        let second_letter = _position.chars().nth(1).unwrap().to_string();
        let second_letter: i8 = second_letter.trim().parse().unwrap();

        // Gets the associated integer to the second character
        let new_coordinate = vec![
            coordinate_hashmap[&first_letter.to_string()],
            8 - second_letter,
        ];
        ////println!("Convert string to vec took: {:?}", now.elapsed());

        return new_coordinate;
    }

    // Pair function to convert_string_to_vec
    pub fn convert_vec_to_string(_position: &Vec<Vec<i8>>) -> Vec<String> {
        let mut letter_coordinate_vec = vec![];
        let letter_vec = ["A", "B", "C", "D", "E", "F", "G", "H"];

        /*Add the index in letter_vec corresponding to the vector's
        first number to letter_coordinate_vec, as well as the second
        number converted to a string*/

        for i in 0.._position.iter().count() {
            if _position[i][0] >= 0 && _position[i][1] >= 0 {
                /*//println!("{:?}", letter_vec[_position[i][0] as usize].to_string()
                + &(8 - _position[i][1]).to_string());*/
                letter_coordinate_vec.push(
                    letter_vec[_position[i][0] as usize].to_string()
                        + &(8 - _position[i][1]).to_string(),
                )
            }
        }
        return letter_coordinate_vec;
    }

    /// If the current game state is InProgress and the move is legal,
    /// move a piece and return the resulting state of the game.
    pub fn make_move(&mut self, _from: &String, _to: String, changecolor: bool) -> GameState {
        if Game::get_game_state(self) == GameState::InProgress {
            //println!("{:?}", _from);
            self.print();
            let piece_to_move = Game::convert_string_to_vec(_from.to_string());
            //println!("{:?}", piece_to_move);
            //println!("{:?}", self.color);
            let own_color = self.board[piece_to_move[1] as usize][piece_to_move[0] as usize]
                .unwrap()
                .color;

            // Used to alert of invalid moves when playing in the terminal
            let mut made_move = false;

            if own_color == self.color {
                let square_to_move_to = Game::convert_string_to_vec(_to);

                let (_irrelevant, possible_moves) =
                    Game::get_possible_moves(self, &piece_to_move, false);

                /*Iterates through possible moves, and if it finds that the square to move to is
                in possible moves, set previous position to None and the new position
                to its previous position's data*/
                for i in 0..possible_moves.iter().count() {
                    if square_to_move_to[0] == possible_moves[i][0]
                        && square_to_move_to[1] == possible_moves[i][1]
                    {
                        let ownpiecetype = self.board[piece_to_move[1] as usize]
                            [piece_to_move[0] as usize]
                            .unwrap()
                            .piecetype;
                        self.board[piece_to_move[1] as usize][piece_to_move[0] as usize] = None;
                        self.board[square_to_move_to[1] as usize][square_to_move_to[0] as usize] =
                            Some(Piece {
                                piecetype: ownpiecetype,
                                color: own_color,
                            });
                        made_move = true;
                    }
                }

                if !made_move {
                    //println!("Invalid move!")
                } else {
                    // Changecolor (the last parameter) has to be false for checkmate to work
                    if changecolor {
                        //println!("Changing color");
                        if self.color == Color::White {
                            self.color = Color::Black;
                        } else {
                            self.color = Color::White;
                        }
                    }
                }
            } else {
                // If the wrong player made the move
                //println!("It's {:?}'s turn, you know", self.color)
            }
        }

        self.print();
        Game::set_promotion(self, PieceType::Queen);
        Game::get_game_state(&self)
    }

    pub fn simplified_make_move(&mut self, _from: &String, _to: String, changecolor: bool) -> GameState {
        //let now = Instant::now();

        if Game::get_game_state(self) == GameState::InProgress {
            //println!("{:?}", _from);
            //self.print();
            let piece_to_move = Game::convert_string_to_vec(_from.to_string());
            //println!("{:?}", piece_to_move);
            //println!("{:?}", self.color);
                let square_to_move_to = Game::convert_string_to_vec(_to);

                        let ownpiecetype = self.board[piece_to_move[1] as usize]
                            [piece_to_move[0] as usize]
                            .unwrap()
                            .piecetype;
                        self.board[piece_to_move[1] as usize][piece_to_move[0] as usize] = None;
                        self.board[square_to_move_to[1] as usize][square_to_move_to[0] as usize] =
                            Some(Piece {
                                piecetype: ownpiecetype,
                                color: self.color,
                            });

        //self.print();
        Game::set_promotion(self, PieceType::Queen);
        if changecolor {
            //println!("Changing color");
            if self.color == Color::White {
                self.color = Color::Black;
            } else {
                self.color = Color::White;
            }
        }
    }
    ////println!("Simplified make_move took: {:?}", now.elapsed());
    Game::get_game_state(&self)

}

    /// Set the piece type that a peasant becames following a promotion.
    pub fn set_promotion(&mut self, _piece: PieceType) -> () {
        /*Rotates through the first and last row, and if a
        piece of the desired color is found, change it to _piece*/
        for mut i in 0..16 {
            let mut preffered_color = Color::White;
            let row_to_check = if i > 7 {
                i -= 8;
                preffered_color = Color::Black;
                7
            } else {
                0
            };
            if self.board[row_to_check][i] != None {
                let own_color = self.board[row_to_check][i].unwrap().color;
                if self.board[row_to_check][i].unwrap().piecetype == PieceType::Pawn
                    && own_color == preffered_color
                {
                    let own_color = self.board[row_to_check][i].unwrap().color;
                    self.board[row_to_check][i as usize] = Some(Piece {
                        piecetype: _piece,
                        color: own_color,
                    });
                }
            }
        }
    }

    // Plays the game in the terminal with string inputs
    pub fn play_the_game(&mut self) {
        let stdin = io::stdin();
        Game::print(self);
        //println!("White, enter your first move: ");

        // Creates a vector with all valid moves, to check against input later
        let mut all_valid = vec![];
        let alphabet = vec!["A", "B", "C", "D", "E", "F", "G", "H"];
        for i in 1..9 {
            for j in 1..9 {
                all_valid.push(format!("{}{}", alphabet[i - 1].to_string(), j.to_string()))
            }
        }

        /* Gets the first line (only relevant one), and if its length is
        right the string is converted to uppercase and a move is made */
        for line in stdin.lock().lines() {
            let unwrapped = line.unwrap();
            let unwrapped: &str = &*unwrapped;
            if &unwrapped.len() > &(4 as usize) {
                let start_position = &unwrapped[0..2].to_uppercase();
                let finalposition = &unwrapped[3..5].to_uppercase();
                if all_valid.contains(&start_position) && all_valid.contains(&finalposition) {
                    Game::make_move(
                        self,
                        &start_position.to_string(),
                        finalposition.to_string(),
                        true,
                    );
                    Game::print(self);
                    //println!("{:?}, enter your move: ", self.color);
                } else {
                    //println!("Invalid move! Enter new:")
                }
            } else {
                //println!("Invalid move! Enter new:")
            }
        }
    }

    // Prints the board in unicode
    pub fn print(&self) {
        let now = Instant::now();

        // it prints '2'
     
        let sw = Stopwatch::start_new();
        println!("#-A--B--C--D--E--F--G--H-#");
        let mut lineiter = 9;
        for line in self.board {
            lineiter -= 1;
            print!("{:?}", lineiter);
            for piece in line {
                if piece != None {
                    if piece.unwrap().color == Color::Black {
                        if piece.unwrap().piecetype == PieceType::Pawn {
                            print!(" {} ", "♙");
                        } else if piece.unwrap().piecetype == PieceType::Rook {
                            print!(" {} ", "♖");
                        } else if piece.unwrap().piecetype == PieceType::Knight {
                            print!(" {} ", "♘");
                        } else if piece.unwrap().piecetype == PieceType::Queen {
                            print!(" {} ", "♕");
                        } else if piece.unwrap().piecetype == PieceType::King {
                            print!(" {} ", "♔");
                        } else if piece.unwrap().piecetype == PieceType::Bishop {
                            print!(" {} ", "♗");
                        }
                    } else {
                        if piece.unwrap().piecetype == PieceType::Pawn {
                            print!(" {} ", "♟︎");
                        } else if piece.unwrap().piecetype == PieceType::Rook {
                            print!(" {} ", "♜");
                        } else if piece.unwrap().piecetype == PieceType::Knight {
                            print!(" {} ", "♞");
                        } else if piece.unwrap().piecetype == PieceType::Queen {
                            print!(" {} ", "♛");
                        } else if piece.unwrap().piecetype == PieceType::King {
                            print!(" {} ", "♚");
                        } else if piece.unwrap().piecetype == PieceType::Bishop {
                            print!(" {} ", "♝");
                        } else if piece.unwrap().piecetype == PieceType::Corpse {
                            print!(" {} ", "x"); //☠️
                        }
                    }
                } else {
                    print!(" . ");
                }
            }
            println!("{:?}", lineiter);
            //println!();
        }
        println!("#-A--B--C--D--E--F--G--H-#");
        println!("Thing took {}ns", sw.elapsed_ms());
        //println!("{:?}", now.elapsed());
    }

    pub fn get_game_state(&self) -> GameState {
        self.state
    }

    pub fn opposite_color_func(own_color: Color) -> Color {
        if own_color == Color::White {
            Color::Black
        } else {
            Color::White
        }
    }

    /* Gets all possible moves from a certain color by repeatedly calling
    get_possible_moves for all the pieces it finds by iterating through the board */
    pub fn get_all_possible_moves(
        &mut self,
        opposite_color: &Color,
    ) -> (Vec<Vec<i8>>, HashMap<Vec<i8>, Vec<i8>>) {
        //let now = Instant::now();
        let mut all_possible_moves = vec![];
        let mut move_from_to_hashmap: HashMap<Vec<i8>, Vec<i8>> = HashMap::new();
        for i in 0..8 {
            for j in 0..8 {
                if self.board[j as usize][i as usize] != None {
                    if self.board[j as usize][i as usize].unwrap().color == *opposite_color {
                        let (_irrelevant, possible_moves) =
                            Game::get_possible_moves(self, &vec![i, j], false);
                        for n in possible_moves {
                            move_from_to_hashmap.insert(n.clone(), vec![i, j]);
                            all_possible_moves.push(n);
                        }
                    }
                }
            }
        }
        ////println!("Get all possible moves took: {:?}", now.elapsed());
        return (all_possible_moves, move_from_to_hashmap);
    }

    // Returns the king's position on the board
    fn get_king_position(&mut self) -> Vec<i8> {
        let mut king_position = vec![];
        for i in 0..8 {
            for j in 0..8 {
                if self.board[j as usize][i as usize] != None {
                    if self.board[j as usize][i as usize].unwrap().color == self.color
                        && self.board[j as usize][i as usize].unwrap().piecetype == PieceType::King
                    {
                        king_position = vec![i, j]
                    }
                }
            }
        }
        return king_position;
    }

    // Returns if the king is in check or not
    pub fn check_check(&mut self) -> bool {
        let own_color = self.color;
        let opposite_color = Game::opposite_color_func(own_color);
        let king_position = Game::get_king_position(self);

        let (all_possible_moves, _irrelevantmap) = self.get_all_possible_moves(&opposite_color);
        if all_possible_moves.contains(&king_position) {
            //println!("CHESS! CHESS! CHESS!");
            self.state = GameState::Check;
            return true;
        } else {
            //println!("NO CHESS! NO CHESS! NO CHESS!");
            return false;
        }
    }

    // Returns if it's checkmate or not.
    pub fn checkmate(&mut self) -> bool {
        let own_color = self.color;
        let (myall_possible_moves, useful_hashmap) = self.get_all_possible_moves(&own_color);
        self.print();
        let mut checkmate = true;

        /* Iterates through all your possible moves, makes the move,
        checks if it's still check, sets checkmate to false if it's
        not check for any move, and reverts to the original boardstate.*/
        for i in 0..myall_possible_moves.iter().count() {
            let saved_boardstate = self.board;
            self.make_move(
                &Game::convert_vec_to_string(&vec![useful_hashmap
                    [&myall_possible_moves[i].clone()]
                    .clone()])[0]
                    .clone(),
                Game::convert_vec_to_string(&vec![myall_possible_moves[i].clone()])[0].clone(),
                false,
            );
            let check = Game::check_check(self);
            self.board = saved_boardstate;
            if !check {
                checkmate = false;
            }
        }
        //println!("checkmate: {:?} for {:?}", checkmate, self.color);
        if checkmate {
            self.state = GameState::GameOver
        }
        ////println!("testing board: {:?}", self.board);
        return checkmate;
    }

    /* If a piece is standing on the given tile, return all possible
    new positions of that piece, taking the rules for check into account*/
    pub fn get_possible_moves(
        &mut self,
        _position: &Vec<i8>,
        should_check: bool,
    ) -> (Vec<String>, Vec<Vec<i8>>) {
        // Reverts corpses (possible moves, visualized as X's, during debugging) to None
        for i in 0..8 {
            for j in 0..8 {
                if self.board[j as usize][i as usize]
                    == Some(Piece {
                        piecetype: PieceType::Corpse,
                        color: Color::White,
                    })
                {
                    self.board[j as usize][i as usize] = None
                }
            }
        }

        let my_piece = self.board[_position[1] as usize][_position[0] as usize];
        if my_piece == None {
            ////println!("Do nothing");
            return (vec!["".to_string()], vec![vec![]]);
        } else {
            let own_color = my_piece.unwrap().color;
            let current_piecetype = my_piece.unwrap().piecetype;
            let opposite_color = Game::opposite_color_func(own_color);

            // Adds two i8's together, because usize can't be negative
            fn convert_usize(possiblenegative: i8, othertoconvert: i8) -> usize {
                let sum = possiblenegative + othertoconvert;
                if sum < 0 || sum > 7 {
                    return 7 as usize;
                } else {
                    return sum as usize;
                }
            }

            /*Adds the positions in the current_vector to possible_moves if
            they are in the limits of the board*/
            fn add_function(
                current_vector: Vec<Vec<i8>>,
                mut possible_moves: Vec<Vec<i8>>,
                position: &Vec<i8>,
            ) -> Vec<Vec<i8>> {
                for i in 0..current_vector.iter().count() {
                    if position[0] + current_vector[i][0] < 8
                        && position[0] + current_vector[i][0] >= 0
                        && position[1] + current_vector[i][1] >= 0
                        && position[1] + current_vector[i][1] < 8
                    {
                        possible_moves.push(vec![
                            position[0] + current_vector[i][0],
                            position[1] + current_vector[i][1],
                        ]);
                    }
                }
                return possible_moves;
            }

            // Adds diagonal lines for the bishop and queen pieces
            fn bishop_function(
                bishop_vector: &mut Vec<Vec<i8>>,
                board: &[[Option<Piece>; 8]; 8],
                color: &Color,
                position: &Vec<i8>,
            ) -> Vec<Vec<i8>> {
                let mut continue_loop = true;
                for fakei in 0..32 {
                    // Goes from the input piece out in all directions, unless halted by continue_loop
                    if fakei % 8 == 0 {
                        continue_loop = true;
                    }

                    let i = if fakei < 8 {
                        fakei
                    } else if fakei < 16 {
                        -(fakei - 8)
                    } else if fakei < 24 {
                        -(fakei - 16)
                    } else {
                        fakei - 24
                    };

                    let j = if fakei < 8 {
                        fakei
                    } else if fakei < 16 {
                        fakei - 8
                    } else if fakei < 24 {
                        -(fakei - 16)
                    } else {
                        -(fakei - 24)
                    };

                    if board[convert_usize(position[1], i)][convert_usize(position[0], j)] != None
                        && continue_loop
                    {
                        if board[convert_usize(position[1], i)][convert_usize(position[0], j)]
                            .unwrap()
                            .color
                            != *color
                        {
                            // Adds a potential capture
                            bishop_vector.push(vec![position[0] + j, position[1] + i]);
                            // Stops if encounters a piece (and the piece isn't itself)
                            if i != 0 {
                                continue_loop = false;
                            }
                        } else {
                            // Stops loop if it encounters a piece of the same color
                            if i != 0 {
                                continue_loop = false;
                            }
                        }
                    } else if continue_loop {
                        // Adds empty square to
                        bishop_vector.push(vec![position[0] + j, position[1] + i]);
                    }
                }
                ////println!("bishop_vector: {:?}", bishop_vector.to_vec());
                ////println!("In bishop function");
                return bishop_vector.to_vec();
            }

            // Adds vertical and horizontal lines to rook and queen's possible moves
            fn rook_function(
                possible_moves: &mut Vec<Vec<i8>>,
                position: &Vec<i8>,
                board: &[[Option<Piece>; 8]; 8],
                color: &Color,
            ) -> Vec<Vec<i8>> {
                let mut continue_xloop = true;
                let mut continue_yloop = true;

                // Specifies which direction to go in (starts from the rook's/queen's square)
                let mut go_up_iter = 0;
                let mut go_up = true;

                for mut i in -7..8 {
                    if i == 0 {
                        continue_xloop = true;
                        continue_yloop = true;
                    }
                    if go_up {
                        go_up_iter -= 1;
                        i = go_up_iter;
                    }

                    // I think a bug appears if there's a block on both x and y
                    if (go_up_iter > 8 || go_up_iter < -8) && go_up {
                        go_up_iter = 0;
                        go_up = false
                    }
                    if !go_up {
                        go_up_iter += 1;
                        i = go_up_iter;
                    }

                    // Add horizontal lines, and stop if it encounters a piece
                    if position[0] + i < 8 && position[0] + i >= 0 && continue_xloop {
                        if board[position[1] as usize][convert_usize(position[0], i)] != None {
                            if board[position[1] as usize][convert_usize(position[0], i)]
                                .unwrap()
                                .color
                                != *color
                            {
                                possible_moves.push(vec![position[0] + i, position[1]]);
                                if i != 0 {
                                    continue_xloop = false;
                                }
                            } else {
                                if i != 0 {
                                    continue_xloop = false;
                                }
                            }
                        } else {
                            possible_moves.push(vec![position[0] + i, position[1]]);
                        }
                    }

                    // Add vertical lines, and stop if it encounters a piece
                    if position[1] + i < 8 && position[1] + i >= 0 && continue_yloop {
                        if board[convert_usize(position[1], i)][position[0] as usize] != None {
                            if board[convert_usize(position[1], i)][position[0] as usize]
                                .unwrap()
                                .color
                                != *color
                            {
                                possible_moves.push(vec![position[0], position[1] + i]);
                                if i != 0 {
                                    continue_yloop = false;
                                }
                            } else {
                                if i != 0 {
                                    continue_yloop = false;
                                }
                            }
                        } else {
                            possible_moves.push(vec![position[0], position[1] + i]);
                        }
                    }
                }
                return possible_moves.to_vec();
            }

            let mut new_position = if current_piecetype == PieceType::Pawn {
                let to_add_one = if own_color == Color::Black { 1 } else { -1 };
                let to_add_two = if own_color == Color::Black { 2 } else { -2 };
                let start_position = if own_color == Color::Black { 1 } else { 6 };
                let promotion_position = if own_color == Color::Black { 7 } else { 0 };
                let mut possible_moves = vec![];

                // Adds one step forward to possible_moves if the square is empty
                if self.board[convert_usize(_position[1], to_add_one)][_position[0] as usize]
                    == None
                {
                    possible_moves.push(vec![_position[0], _position[1] + to_add_one]);
                }

                // Adds two steps forward to possible_moves if the pawn is in its initial position
                if _position[1] == start_position
                    && self.board[convert_usize(_position[1], to_add_two)][_position[0] as usize]
                        == None
                    && self.board[convert_usize(_position[1], to_add_one)][_position[0] as usize]
                        == None
                {
                    possible_moves.push(vec![_position[0], _position[1] + to_add_two]);
                }

                // Adds the diagonal capture move if a piece of the opposite color is there
                for i in 0..8 {
                    for j in 0..8 {
                        if self.board[i][j] != None {
                            if self.board[i][j].unwrap().color == opposite_color {
                                if i as i8 == _position[1] + to_add_one
                                    && (j as i8 == _position[0] + 1 || j as i8 == _position[0] - 1)
                                {
                                    possible_moves.push(vec![j as i8, i as i8]);
                                }
                            }
                            // Start of en passant below
                            /*if self.board[i][j].unwrap().color == opposite_color {
                                if j as i8 == 6 - start_position && j as i8 == _position[1] + to_add_one
                                    && i as i8 == _position[1] + to_add_one
                                {
                                    ////println!("Index {} {}", j, i);
                                    ////println!("{:?}", self.board[i][j].unwrap());
                                    possible_moves.push(vec![j as i8, i as i8]);
                                }
                            }*/
                        }
                    }
                }

                possible_moves
            } else if current_piecetype == PieceType::Rook {
                let mut possible_moves = vec![];
                possible_moves =
                    rook_function(&mut possible_moves, &_position, &self.board, &own_color);
                possible_moves
            } else if current_piecetype == PieceType::Knight {
                let mut possible_moves = vec![];

                // All the different moves a knight can make (relative to current position)
                let knight_vector = vec![
                    vec![1, 2],
                    vec![-1, 2],
                    vec![-1, -2],
                    vec![1, -2],
                    vec![2, -1],
                    vec![-2, 1],
                    vec![2, 1],
                    vec![-2, -1],
                ];

                possible_moves = add_function(knight_vector, possible_moves, &_position);
                possible_moves
            } else if current_piecetype == PieceType::King {
                let mut possible_moves = vec![];

                // All the different moves a king can make (relative to current position)
                let kingvector = vec![
                    vec![1, 1],
                    vec![-1, -1],
                    vec![-1, 0],
                    vec![1, 0],
                    vec![0, -1],
                    vec![0, 1],
                    vec![1, -1],
                    vec![-1, 1],
                ];

                possible_moves = add_function(kingvector, possible_moves, &_position);
                possible_moves
            } else if current_piecetype == PieceType::Bishop {
                let mut bishop_vector = vec![];
                let possible_moves =
                    bishop_function(&mut bishop_vector, &self.board, &own_color, &_position);
                ////println!("{:?}", possible_moves);
                possible_moves
            } else if current_piecetype == PieceType::Queen {
                let mut queen_vector = vec![];
                let mut possible_moves =
                    bishop_function(&mut queen_vector, &self.board, &own_color, &_position);
                rook_function(&mut possible_moves, &_position, &self.board, &own_color);

                possible_moves
            } else {
                vec![vec![0, 3]]
            };

            // Sort out the piece's current position from new_position, and negative
            new_position.retain(|x| {
                !(x[0] == _position[0] && x[1] == _position[1])
                    && (x[0] >= 0 && x[1] >= 0)
                    && (x[0] < 8 && x[1] < 8)
            });

            // Sort out duplicates
            new_position.dedup();

            ////println!("{:?}", new_position);

            // Removes moves which contain a piece of the same color (only necessary for knight, king and pawn)
            let mut new_new_position: Vec<Vec<i8>> = vec![];
            if current_piecetype == PieceType::Pawn || current_piecetype == PieceType::Knight || current_piecetype == PieceType::King {
                for i in 0..8 {
                    for j in 0..8 {
                        if new_position.contains(&vec![i, j]) {
                            if self.board[j as usize][i as usize] != None {
                                if self.board[j as usize][i as usize].unwrap().color != own_color {
                                    new_new_position.push(vec![i, j]);
                                }
                            } else {
                                new_new_position.push(vec![i, j]);
                            }
                        }
                    }
                }
            } else {
                new_new_position = new_position
            }
            ////println!("{:?}", new_new_position);

            // Comment out the for and if below if you don't want corpses
            /*for i in 0..8 {
                for j in 0..8 {
                    if new_new_position.contains(&vec![i, j]) {
                        self.board[j as usize][i as usize] = Some(Piece {
                            piecetype: PieceType::Corpse,
                            color: Color::White,
                        });
                    }
                }
            }

            //println!("{:?}", new_new_position);
            if new_new_position.iter().count() > 0 {
                Game::print(&self)
            }*/

            let converted_new_vector = Game::convert_vec_to_string(&new_new_position);
            ////println!("{:?}", converted_new_vector);

            // Check for check
            let mut even_newer_vector: Vec<Vec<i8>> = vec![];
            let mut even_newer_converted_new_vector: Vec<String> = vec![];

            if should_check {
                for mut i in 0..converted_new_vector.iter().count() {
                    let saved_boardstate = self.board;
                    let stringposition = &Game::convert_vec_to_string(&vec![_position.to_vec()])[0];

                    Game::make_move(
                        self,
                        &stringposition,
                        converted_new_vector[i].to_string(),
                        false,
                    );

                    let check = Game::check_check(self);
                    if !check {
                        even_newer_vector.push(new_new_position[i].clone());
                        even_newer_converted_new_vector.push(converted_new_vector[i].clone());
                    }
                    self.board = saved_boardstate;
                }
            } else {
                even_newer_vector = new_new_position;
                even_newer_converted_new_vector = converted_new_vector;
            }

            return (even_newer_converted_new_vector, even_newer_vector);
        }
    }
}

// Makes it possible to print game
impl fmt::Debug for Game {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "")
    }
}
