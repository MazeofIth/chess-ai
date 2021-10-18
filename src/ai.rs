use crate::lib::Color;
use crate::lib::Game;
use crate::lib::GameState;
use crate::lib::Piece;
use crate::lib::PieceType;
use std::collections::HashMap;
use std::thread::sleep;
use std::time::{Duration, Instant};

impl Game {
    
    fn ai_get_sequential_move(
        &mut self,
        i: usize,
        all_possible_moves: &Vec<Vec<i8>>,
        very_useful_map: &HashMap<Vec<i8>, Vec<i8>>,
    ) -> (String, String, Vec<i8>, Vec<i8>) {
        let owncolor = self.color;

        //println!("{:?}", owncolor);
        let mut reali = i;
        //println!("{:?}", all_possible_moves);

        if i >= all_possible_moves.len() {
            reali = all_possible_moves.len() - 1
        }
        let randommoveto = &all_possible_moves[reali].clone();
        //println!("{:?}", randommoveto);
        let randommove = very_useful_map[randommoveto].clone();
        //println!("{:?}", randommove);
        let randommovestring = "".to_string(); 
        //println!("A random move string: {:?}", randommovestring);
        let randommovetostring = "".to_string();
        //println!("A random move to string: {:?}", randommovetostring);
        return (
            randommovestring,
            randommovetostring,
            randommove.to_vec(),
            randommoveto.to_vec(),
        );
    }

    fn better_evaluate(&mut self) -> i32 {
        let mut white_value_sum = 0;
        let mut black_value_sum = 0;
        let pawntable = vec![
            0, 0, 0, 0, 0, 0, 0, 0, 5, 10, 10, -20, -20, 10, 10, 5, 5, -5, -10, 0, 0, -10, -5, 5,
            0, 0, 0, 20, 20, 0, 0, 0, 5, 5, 10, 25, 25, 10, 5, 5, 10, 10, 20, 30, 30, 20, 10, 10,
            50, 50, 50, 50, 50, 50, 50, 50, 0, 0, 0, 0, 0, 0, 0, 0,
        ];
        let knighttable = vec![
            -50, -40, -30, -30, -30, -30, -40, -50, -40, -20, 0, 5, 5, 0, -20, -40, -30, 5, 10, 15,
            15, 10, 5, -30, -30, 0, 15, 20, 20, 15, 0, -30, -30, 5, 15, 20, 20, 15, 5, -30, -30, 0,
            10, 15, 15, 10, 0, -30, -40, -20, 0, 0, 0, 0, -20, -40, -50, -40, -30, -30, -30, -30,
            -40, -50,
        ];
        let bishoptable = vec![
            -20, -10, -10, -10, -10, -10, -10, -20, -10, 5, 0, 0, 0, 0, 5, -10, -10, 10, 10, 10,
            10, 10, 10, -10, -10, 0, 10, 10, 10, 10, 0, -10, -10, 5, 5, 10, 10, 5, 5, -10, -10, 0,
            5, 10, 10, 5, 0, -10, -10, 0, 0, 0, 0, 0, 0, -10, -20, -10, -10, -10, -10, -10, -10,
            -20,
        ];
        let rooktable = vec![
            0, 0, 0, 5, 5, 0, 0, 0, -5, 0, 0, 0, 0, 0, 0, -5, -5, 0, 0, 0, 0, 0, 0, -5, -5, 0, 0,
            0, 0, 0, 0, -5, -5, 0, 0, 0, 0, 0, 0, -5, -5, 0, 0, 0, 0, 0, 0, -5, 5, 10, 10, 10, 10,
            10, 10, 5, 0, 0, 0, 0, 0, 0, 0, 0,
        ];
        let queentable = vec![
            -20, -10, -10, -5, -5, -10, -10, -20, -10, 0, 0, 0, 0, 5, 0, -10, -10, 0, 5, 5, 5, 5,
            5, -10, -5, 0, 5, 5, 5, 5, 0, 0, -5, 0, 5, 5, 5, 5, 0, -5, -10, 0, 5, 5, 5, 5, 0, -10,
            -10, 0, 0, 0, 0, 0, 0, -10, -20, -10, -10, -5, -5, -10, -10, -20,
        ];
        let kingtable = vec![
            20, 30, 10, 0, 0, 10, 30, 20, 20, 20, 0, 0, 0, 0, 20, 20, -10, -20, -20, -20, -20, -20,
            -20, -10, -20, -30, -30, -40, -40, -30, -30, -20, -30, -40, -40, -50, -50, -40, -40,
            -30, -30, -40, -40, -50, -50, -40, -40, -30, -30, -40, -40, -50, -50, -40, -40, -30,
            -30, -40, -40, -50, -50, -40, -40, -30,
        ];
        // There is a king endgame table
        for i in 0..8 {
            for j in 0..8 {
                if self.board[j as usize][i as usize] != None {
                    {
                        let currentcolor = self.board[j as usize][i as usize].unwrap().color;
                        let piecetype = self.board[j as usize][i as usize].unwrap().piecetype;
                        let mut value = piecetype.value();
                        if piecetype == PieceType::Pawn {
                            value += pawntable[j * 8 + i as usize]
                        } else if piecetype == PieceType::Knight {
                            value += knighttable[j * 8 + i as usize]
                        } else if piecetype == PieceType::Bishop {
                            value += bishoptable[j * 8 + i as usize]
                        } else if piecetype == PieceType::Rook {
                            value += rooktable[j * 8 + i as usize]
                        } else if piecetype == PieceType::Queen {
                            value += queentable[j * 8 + i as usize]
                        } else if piecetype == PieceType::King {
                            value += kingtable[j * 8 + i as usize]
                        }
                        if currentcolor == Color::White {
                            white_value_sum += value;
                        } else {
                            black_value_sum += value;
                        }
                        //println!("{:?}", piecetype)
                    }
                }
            }
        }
        //println!("White: {:?}", white_value_sum);
        //println!("Black: {:?}", black_value_sum);
        if self.color == Color::White {
            //println!("{:?}", self.color);
            return white_value_sum - black_value_sum; // The bot wants this to be as low as possible (negative)
        } else {
            return -(white_value_sum - black_value_sum); // The bot wants this to be as high as possible (positive)
        }
    }

    fn recursive_search(&mut self, depth: i32) -> (i32, Vec<i8>, Vec<i8>) {
        if depth == 0 {
            self.positions_evaluated += 1;
            //println!("Return!");
            return (Game::better_evaluate(self), vec![], vec![]);
        }
        let mut all_equal_moves: HashMap<Vec<i8>, Vec<i8>> = HashMap::new();

        let owncolor = self.color;
        let mut best_evaluation = -100000;
        //let opposite_color = Game::opposite_color_func(owncolor);
        //self.color = opposite_color;
        let saved_boardstate = self.board;
        //println!("Owncolor: {:?} , Self.color: {:?}, Opposite_color: {:?}", owncolor,  self.color, opposite_color);
        let (all_possible_moves, very_useful_map) = self.get_all_possible_moves(&owncolor);
        let count = all_possible_moves.iter().count();
        for j in 0..count {
            let (randommovestring, randommovetostring, randommove, randommoveto) =
                Game::ai_get_sequential_move(self, j, &all_possible_moves, &very_useful_map);
            //println!("Middle color: {:?}", self.color);
            Game::simplified_make_move(self, randommove.clone(), randommoveto.clone(), true);
            //self.print();
            let (evaluation, irrelevant, irrelevant2) = Game::recursive_search(self, depth - 1);
            let evaluation = -evaluation;
            if evaluation > best_evaluation {
                //println!("I picked eval : {:?}", evaluation);
                best_evaluation = evaluation;
                //best_move = randommovestring;
                //best_move_to = randommovetostring;
                all_equal_moves = HashMap::new();
                all_equal_moves.insert(randommove, randommoveto);
                //all_equal_moves = HashMap::new();
                //all_equal_moves.insert(best_move.clone(), best_move_to.clone());
            } else if evaluation == best_evaluation {
                all_equal_moves.insert(randommove, randommoveto);
            }
            // It's the opposite's color evaluation which is relevant, and you want to choose the least value
            self.color = owncolor;
            self.board = saved_boardstate;
        }
        let someequalmove = all_equal_moves.iter().nth(0).unwrap();
        let best_move = someequalmove.0; //to_string();
        let best_move_to = someequalmove.1; //.to_string();

        //self.color = opposite_color;
        //self.print();
        //println!("Best evaluation: {:?}", best_evaluation);
        //println!("Best move: {:?} {:?}", best_move, best_move_to);
        return (best_evaluation, best_move.to_vec(), best_move_to.to_vec());
    }

    pub fn better_chess_ai(&mut self) -> bool {
        //let now = Instant::now();
        self.positions_evaluated = 0;
        let (irrelevant, best_move, best_move_to) = Game::recursive_search(self, 3);
        println!("best  AI : {:?} {:?}", best_move, best_move_to);
        Game::simplified_make_move(self, best_move, best_move_to, true);
        Game::print(self);
        println!("positions evaluated : {:?}", self.positions_evaluated);
        if Game::better_evaluate(self).abs() > 9000 {
            return true;
        } else {
            return false;
        }
        //println!("Chess AI took: {:?}", now.elapsed());
    }
}
