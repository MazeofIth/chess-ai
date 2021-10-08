use crate::lib::Color;
use crate::lib::Game;
use crate::lib::GameState;
use crate::lib::Piece;
use crate::lib::PieceType;
use std::collections::HashMap;
use std::thread::sleep;
use std::time::{Duration, Instant};

impl Game {
    // The below code is for a terrible AI, uncommented because everything will change
    fn evaluate_board_state(&mut self) -> (i32, i32) {
        let mut white_value_sum = 0;
        let mut black_value_sum = 0;
        for i in 0..8 {
            for j in 0..8 {
                if self.board[j as usize][i as usize] != None {
                    {
                        let currentcolor = self.board[j as usize][i as usize].unwrap().color;
                        let piecetype = self.board[j as usize][i as usize].unwrap().piecetype;
                        let value = piecetype.value();
                        if currentcolor == Color::White {
                            white_value_sum += value;
                        } else {
                            black_value_sum += value;
                        }
                        ////println!("{:?}", piecetype)
                    }
                }
            }
        }
        ////println!("White: {:?}", white_value_sum);
        ////println!("Black: {:?}", black_value_sum);
        return (white_value_sum, black_value_sum);
    }

    // The below code is for a terrible AI, uncommented because everything will change
    fn ai_get_sequential_move(
        &mut self,
        i: usize,
        all_possible_moves: &Vec<Vec<i8>>,
        very_useful_map: &HashMap<Vec<i8>, Vec<i8>>,
    ) -> (String, String, Vec<i8>, Vec<i8>) {
        let owncolor = self.color;
        //let (all_possible_moves, very_useful_map) = self.get_all_possible_moves(&owncolor);
        //println!("{:?}", owncolor);
        let mut reali = i;
        ////println!("{:?}", all_possible_moves);
        ////println!("{:?}", very_useful_map);
        /*if all_possible_moves.len() == 0 {

        }*/
        //println!("{:?}", reali);
        if i >= all_possible_moves.len() {
            reali = all_possible_moves.len() - 1
        }
        let randommoveto = &all_possible_moves[reali].clone();
        ////println!("{:?}", randommoveto);
        let randommove = very_useful_map[randommoveto].clone();
        ////println!("{:?}", randommove);
        let randommovestring = "".to_string(); //Game::convert_vec_to_string(&vec![randommove.to_vec()])[0].clone();
                                               ////println!("A random move string: {:?}", randommovestring);
        let randommovetostring = "".to_string();
        //Game::convert_vec_to_string(&vec![randommoveto.to_vec()])[0].clone();
        ////println!("A random move to string: {:?}", randommovetostring);
        ////println!("All possible: {:?}", very_useful_map);
        return (
            randommovestring,
            randommovetostring,
            randommove.to_vec(),
            randommoveto.to_vec(),
        );
    }

    // The below code is for a terrible AI, uncommented because everything will change

    /*fn get_data_of_opposite_color(&mut self) -> i32 {
        let mut owncolor = self.color;
        let mut best_evaluation = 150;
        let mut best_move: String = "".to_string();
        let mut best_move_to: String = "".to_string();
        let opposite_color = Game::opposite_color_func(owncolor);
        self.color = opposite_color;
        let saved_boardstate = self.board;
        println!("Owncolor: {:?} , Self.color: {:?}, Opposite_color: {:?}", owncolor,  self.color, opposite_color);

        let (all_possible_moves, very_useful_map) = self.get_all_possible_moves(&opposite_color);
        let count = all_possible_moves.iter().count();
        for mut j in 0..count {
            let saved_boardstate = self.board;
            //let (randommovestring, randommovetostring, randommove, randommoveto) = Game::ai_get_random_move(self);
            let (randommovestring, randommovetostring, randommove, randommoveto) =
                Game::ai_get_sequential_move(self, j, &all_possible_moves, &very_useful_map);
                //println!("Middle color: {:?}", self.color);
            Game::simplified_make_move(
                self,
                &randommovestring,
                randommovetostring.to_string(),
                false,
            );
            //self.print();
            // Evals best move
            let (white_value_sum, black_value_sum) = Game::evaluate_board_state(self);
            // It's the opposite's color evaluation which is relevant, and you want to choose the least value
            if owncolor == Color::White && best_evaluation > white_value_sum {
                best_evaluation = white_value_sum;
                best_move = randommovestring;
                best_move_to = randommovetostring;
                    //self.print();
                //println!("Best (black) move: {:?} {:?}", best_move, best_move_to)

            } else if owncolor == Color::Black && best_evaluation > black_value_sum {
                best_evaluation = black_value_sum;
                best_move = randommovestring;
                best_move_to = randommovetostring;
                self.print();
                //println!("Best (white) move: {:?} {:?}", best_move, best_move_to);
            }

            self.board = saved_boardstate;
        }
        self.print();
        println!("Best evaluation: {:?}", best_evaluation);
        println!("Best move: {:?} {:?}", best_move, best_move_to);
        return best_evaluation;
    }*/

    /*fn get_data_of_my_color(&mut self, make : bool) {
        let mut owncolor = self.color;
        let mut best_evaluation = 10;
        let mut best_move: String = "A2".to_string();
        let mut best_move_to: String = "A3".to_string();
        let opposite_color = Game::opposite_color_func(owncolor);
        let saved_boardstate = self.board;
        let (all_possible_moves, very_useful_map) = self.get_all_possible_moves(&owncolor);
        //let mut best_outcome_for_opposite = 150;
        let mut all_equal_moves: HashMap<String, String> = HashMap::new();
        for mut j in 0..all_possible_moves.iter().count() {
            let saved_boardstate = self.board;
            //let (randommovestring, randommovetostring, randommove, randommoveto) = Game::ai_get_random_move(self);
            let (randommovestring, randommovetostring, randommove, randommoveto) =
                Game::ai_get_sequential_move(self, j, &all_possible_moves, &very_useful_map);

            Game::simplified_make_move(
                self,
                &randommovestring,
                randommovetostring.to_string(),
                false,
            );

            // Evals best move
            let best_outcome_for_opposite = Game::get_data_of_opposite_color(self);
            println!("best outcome for opposite final: {:?}", best_outcome_for_opposite);

            self.color = owncolor;
            // Best outcome for opposite is black's worst score (after white's best move), you want to make it as
            // high (bad for white) as possible, which is why the if statemnet above chooses the highest
            if best_evaluation < best_outcome_for_opposite {
                best_evaluation = best_outcome_for_opposite;
                best_move = randommovestring;
                best_move_to = randommovetostring;
                all_equal_moves = HashMap::new();
                all_equal_moves.insert(best_move.clone(), best_move_to.clone());
            } else if best_evaluation == best_outcome_for_opposite {
                all_equal_moves.insert(randommovestring.clone(), randommovetostring);
        }
            self.board = saved_boardstate;
        }

        // Makes actual AI move
        //println!("Hellooooo!");
        //Game::print(self);
        ////println!("testing board: {:?}", self.board);
        //println!("Whose turn: {:?}", owncolor);
        //println!("Best evaulation: {:?}", best_evaluation);
        ////println!("White: {:?}", white_value_sum);
        ////println!("Black: {:?}", black_value_sum);
        let someequalmove = all_equal_moves.iter().nth(0).unwrap();
        best_move = someequalmove.0.to_string();
        best_move_to = someequalmove.1.to_string();
        //println!("best_move : {:?}", hashwef);
        Game::simplified_make_move(self, &best_move, best_move_to.to_string(), true);
        Game::print(self);
    }*/

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
                        ////println!("{:?}", piecetype)
                    }
                }
            }
        }
        ////println!("White: {:?}", white_value_sum);
        ////println!("Black: {:?}", black_value_sum);
        if self.color == Color::White {
            //println!("{:?}", self.color);
            return white_value_sum - black_value_sum; // The bot wants this to be as low as possible (negative)
        } else {
            return -(white_value_sum - black_value_sum); // The bot wants this to be as high as possible (positive)
        }
        // Depth search 2 would be color white
    }

    fn recursive_search(&mut self, depth: i32) -> (i32, Vec<i8>, Vec<i8>) {
        if depth == 0 {
            //println!("Return!");
            self.positions_evaluated += 1;
            return (Game::better_evaluate(self), vec![], vec![]);
            //println!("Return nhnmmm!");
        }
        let mut all_equal_moves: HashMap<Vec<i8>, Vec<i8>> = HashMap::new();
        //let currentboardeval = Game::better_evaluate(self), "".to_string(),"".to_string())
        //println!("{:?}", currentboardeval);
        let owncolor = self.color;
        let mut best_evaluation = -100000;
        //let mut best_move: String = "".to_string();
        //let mut best_move_to: String = "".to_string();
        //let opposite_color = Game::opposite_color_func(owncolor);
        //self.color = opposite_color;
        let saved_boardstate = self.board;
        //println!("Owncolor: {:?} , Self.color: {:?}, Opposite_color: {:?}", owncolor,  self.color, opposite_color);
        let (all_possible_moves, very_useful_map) = self.get_all_possible_moves(&owncolor);
        let count = all_possible_moves.iter().count();
        for j in 0..count {
            //let (randommovestring, randommovetostring, randommove, randommoveto) = Game::ai_get_random_move(self);

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

            /*else if best_evaluation == best_outcome_for_opposite {
                all_equal_moves.insert(randommovestring.clone(), randommovetostring);
            }*/
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
        /*for mut i in 0..1 {
            //let now = Instant::now();
            Game::get_data_of_my_color(self, false);
            let (white_value_sum, black_value_sum) = Game::evaluate_board_state(self);
            if white_value_sum < 100 || black_value_sum < 100 {
                //println!("{:?} lost on Turn {:?}", self.color, i);
                return true
            }
            ////println!("Chess AI took: {:?}", now.elapsed());
        }*/
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
    }

    /*pub fn chess_ai(&mut self) {
        for mut i in 0..1000 {
            let mut owncolor = self.color;
            let mut checkmate = false;
            let mut best_evaluation = 150;
            let mut best_move: String = "A2".to_string();
            let mut best_move_to: String = "A3".to_string();
            let opposite_color = Game::opposite_color_func(owncolor);
            let saved_boardstate = self.board;
            let (all_possible_moves, very_useful_map) = self.get_all_possible_moves(&owncolor);

            // You've changed this !!!
            for j in 0..all_possible_moves.iter().count() {
                let saved_boardstate = self.board;
                //let (randommovestring, randommovetostring, randommove, randommoveto) = Game::ai_get_random_move(self);
                let (randommovestring, randommovetostring, randommove, randommoveto) =
                    Game::ai_get_sequential_move(self, j, &all_possible_moves, &very_useful_map);

                Game::make_move(
                    self,
                    &randommovestring,
                    randommovetostring.to_string(),
                    false,
                );

                let (white_value_sum, black_value_sum) = Game::evaluate_board_state(self);
                ////println!("Current color : {:?}", owncolor);
                if self.color == Color::White && best_evaluation > black_value_sum {
                    best_evaluation = black_value_sum;
                    best_move = randommovestring;
                    best_move_to = randommovetostring;
                } else if self.color == Color::Black && best_evaluation > white_value_sum {
                    best_evaluation = white_value_sum;
                    best_move = randommovestring;
                    best_move_to = randommovetostring;
                }

                self.board = saved_boardstate;
                //self.color = owncolor;
                //self.color = opposite_color;
                ////println!("Current color : {:?}", self.color);
                //Game::print(self);

                /*if Game::check_check(self) {
                    if Game::checkmate(self) {
                        //println!("Checkmate!!!");
                        //println!("{:?}'s turn", self.color);
                        i += 1000;
                        checkmate = true;
                    }
                    //println!("Check!!!");
                    //i += 1000;
                    //self.board = saved_boardstate;
                }*/
                //if !checkmate {
                if j == all_possible_moves.iter().count() - 1 {
                    //println!("Hellooooo!");
                    Game::print(self);
                    ////println!("testing board: {:?}", self.board);
                    //println!("Whose turn: {:?}", owncolor);
                    //println!("Best evaulation: {:?}", best_evaluation);
                    //println!("White: {:?}", white_value_sum);
                    //println!("Black: {:?}", black_value_sum);
                    Game::make_move(self, &best_move, best_move_to.to_string(), true);
                    Game::print(self);
                }
            }
            let (white_value_sum, black_value_sum) = Game::evaluate_board_state(self);
            ////println!({"{:?} to {:?}"}, best_move, best_move_to);
            ////println!("{:?} {:?}", {"{:?}"}, &best_move);
            ////println!("{:?} {:?}", {"{:?}"}, &best_move_to);
            //let hmm = best_move.clone();

            if white_value_sum < 100 || black_value_sum < 100 {
                //println!("{:?} lost on Turn {:?}", self.color, i);
                break;
            }
        }
        //Game::min_max(self);
    }*/
}
