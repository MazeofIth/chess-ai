//! A collection of semi-random shape and image drawing examples.

//let mut game = eliasfl_chess::Game::new();
use crate::lib::Game;
use crate::lib::GameState;
use crate::lib::Piece;
use crate::lib::PieceType;
use glam::*;
//use crate::lib::Color;
//use eliasfl_chess::*;
use ggez::event;
use ggez::event::MouseButton;
use ggez::graphics::{self, Color, DrawMode, DrawParam};
use ggez::input::keyboard::KeyCode;
use ggez::input::keyboard::KeyMods;
//use ggez::timer;
use ggez::{Context, GameResult};
use std::collections::HashMap;
use std::env;
use std::path;
mod ai;
mod lib;
use std::thread::sleep;
use std::time::{Duration, Instant};
use include_dir::{include_dir, Dir};
use std::path::Path;

const PROJECT_DIR: Dir = include_dir!("resources");

struct MainState {
    image1: graphics::Image,
    image2: graphics::Image,
    image3: graphics::Image,
    image4: graphics::Image,
    image5: graphics::Image,
    image6: graphics::Image,
    image7: graphics::Image,
    image8: graphics::Image,
    image9: graphics::Image,
    image10: graphics::Image,
    image11: graphics::Image,
    image12: graphics::Image,
    changevariable: bool,
    xpos: f64,
    ypos: f64,
    game: lib::Game,
    selected: bool,
    selectedvector: Vec<i32>,
    string_from: String,
    possiblemoves: Vec<Vec<i8>>,
    gameover: bool,
    updatecallai: bool,
    time: Instant,
}

/// A chess board is 8x8 tiles.
const GRID_SIZE: (i16, i16) = (8, 8);
/// Sutible size of each tile.
const GRID_CELL_SIZE: (i16, i16) = (45, 45);

/// Size of the application window.
const SCREEN_SIZE: (f32, f32) = (
    GRID_SIZE.0 as f32 * GRID_CELL_SIZE.0 as f32,
    GRID_SIZE.1 as f32 * GRID_CELL_SIZE.1 as f32,
);

pub fn convert_vec_to_string(_position: &Vec<Vec<i8>>) -> Vec<String> {
    let mut letter_coordinate_vec = vec![];
    let letter_vec = ["A", "B", "C", "D", "E", "F", "G", "H"];

    /*Add the index in letter_vec corresponding to the vector's
    first number to letter_coordinate_vec, as well as the second
    number converted to a string*/

    for i in 0.._position.iter().count() {
        if _position[i][0] >= 0 && _position[i][1] >= 0 {
            /*println!("{:?}", letter_vec[_position[i][0] as usize].to_string()
            + &(8 - _position[i][1]).to_string());*/
            letter_coordinate_vec.push(
                letter_vec[_position[i][0] as usize].to_string()
                    + &(8 - _position[i][1]).to_string(),
            )
        }
    }
    return letter_coordinate_vec;
}

/*Convert between the string (e.g. "A1") and vector (e.g. [0, 1])
representation of coordinates, because make_move uses strings as parameters*/
fn convert_string_to_vec(_position: String) -> Vec<i8> {
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

    return new_coordinate;
}

// GUI Color representations
const BLACK: Color = Color::new(228.0 / 255.0, 196.0 / 255.0, 108.0 / 255.0, 1.0);
const WHITE: Color = Color::new(150.0 / 255.0, 75.0 / 255.0, 0.0 / 255.0, 1.0);
// 139,69,19)
const BROWN: Color = Color::new(101.0 / 255.0, 67.0 / 255.0, 0.0 / 33.0, 1.0);
const RED: Color = Color::new(200.0 / 255.0, 157.0 / 255.0, 0.0 / 124.0, 1.0);

//const bishop1 : include_dir::File = PROJECT_DIR.get_file("black_bishop.png"); //.unwrap();
//const bishop : include_dir::File = PROJECT_DIR.to_file("black_bishop.png").unwrap(); //.unwrap();

impl MainState {
    /// Load images and create meshes.
    fn new(ctx: &mut Context) -> GameResult<MainState> {
        let image1 = graphics::Image::new(ctx, "/black_pawn.png")?;
        let image2 = graphics::Image::new(ctx, "/white_pawn.png")?;
        let image3 = graphics::Image::new(ctx, "/white_king.png")?;
        let image4 = graphics::Image::new(ctx, "/black_king.png")?;
        let image5 = graphics::Image::new(ctx, "/black_queen.png")?;
        let image6 = graphics::Image::new(ctx, "/white_queen.png")?;
        let image7 = graphics::Image::new(ctx, "/white_bishop.png")?;
        let image8 = graphics::Image::new(ctx, "/white_knight.png")?;
        let image9 = graphics::Image::new(ctx, "/white_rook.png")?;
        let image10 = graphics::Image::new(ctx, "/black_bishop.png")?;
        let image11 = graphics::Image::new(ctx, "/black_knight.png")?;
        let image12 = graphics::Image::new(ctx, "/black_rook.png")?;
        let changevariable = false;
        let xpos = 0 as f64;
        let ypos = 0 as f64;
        //let game = eliasfl_chess::Game::new();
        let mut game = lib::Game::new();
        let board = &game.board;
        let selected = false;
        let selectedvector = vec![0, 1];
        let string_from = "".to_string();
        let possiblemoves = vec![vec![]];
        let gameover = false;
        let updatecallai = false;
        let time = Instant::now();

        println!("{:?}", board);

        let s = MainState {
            image1,
            image2,
            image3,
            image4,
            image5,
            image6,
            image7,
            image8,
            image9,
            image10,
            image11,
            image12,
            changevariable,
            xpos,
            ypos,
            game,
            selected,
            selectedvector,
            string_from,
            possiblemoves,
            gameover,
            updatecallai,
            time,
        };

        Ok(s)
    }
}

impl event::EventHandler<ggez::GameError> for MainState {
    fn update(&mut self, ctx: &mut Context) -> GameResult {
        //const DESIRED_FPS: u32 = 60;

        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult {
        graphics::clear(ctx, [0.1, 0.2, 0.3, 1.0].into());

        // create text representation
        let state_text = graphics::Text::new(
            graphics::TextFragment::from(format!("Game is {:?}.", self.game.get_game_state()))
                .scale(graphics::PxScale { x: 30.0, y: 30.0 }),
        );

        // get size of text
        let text_dimensions = state_text.dimensions(ctx);

        let background_box = graphics::Mesh::new_rectangle(
            ctx,
            DrawMode::fill(),
            graphics::Rect::new(5 as f32, 5 as f32, 5 as f32, 5 as f32),
            [1.0, 1.0, 1.0, 1.0].into(),
        )?;

        // create background rectangle with white coulouring
        let background_box = graphics::Mesh::new_rectangle(
            ctx,
            graphics::DrawMode::fill(),
            graphics::Rect::new(
                (SCREEN_SIZE.0 - text_dimensions.w as f32) / 2f32 as f32 - 8.0,
                (SCREEN_SIZE.0 - text_dimensions.h as f32) / 2f32 as f32,
                text_dimensions.w as f32 + 16.0,
                text_dimensions.h as f32,
            ),
            [1.0, 1.0, 1.0, 1.0].into(),
        )?;

        graphics::draw(ctx, &background_box, DrawParam::default());

        // draw tiles
        for i in 0..64 {
            let rectangle = graphics::Mesh::new_rectangle(
                ctx,
                graphics::DrawMode::fill(),
                graphics::Rect::new_i32(
                    i % 8 * GRID_CELL_SIZE.0 as i32,
                    i / 8 * GRID_CELL_SIZE.1 as i32,
                    GRID_CELL_SIZE.0 as i32,
                    GRID_CELL_SIZE.1 as i32,
                ),
                match i % 2 {
                    0 => match i / 8 {
                        _row if _row % 2 == 0 => WHITE,
                        _ => BLACK,
                    },
                    _ => match i / 8 {
                        _row if _row % 2 == 0 => BLACK,
                        _ => WHITE,
                    },
                },
            )?;
            graphics::draw(ctx, &rectangle, (ggez::mint::Point2 { x: 0.0, y: 0.0 },));
        }

        //&& self.board[self.xpos as i32, self.ypos as i32] != None
        if self.selected {
            /* let mut should_i = false;
            for (key, value) in self.game.board.clone().into_iter() {
                let currentxpos = key.file - 1;
                let currentypos = key.rank - 1;
                if currentxpos == self.xpos as u8 && currentypos == self.ypos as u8 {
                    //if value Piece::Option(eliasfl_chess::Color::White) ==
                    should_i = true;
                }
            }*/
            //graphics::draw(ctx, &self.image5, (glam::Vec2::new(45.0*self.selectedvector[0] as f32, 44.8*self.selectedvector[1] as f32),));
            let rectangle = graphics::Mesh::new_rectangle(
                ctx,
                graphics::DrawMode::fill(),
                graphics::Rect::new_i32(
                    self.xpos as i32 * GRID_CELL_SIZE.0 as i32,
                    self.ypos as i32 * GRID_CELL_SIZE.1 as i32,
                    GRID_CELL_SIZE.0 as i32,
                    GRID_CELL_SIZE.1 as i32,
                ),
                BROWN,
            )?;
            graphics::draw(ctx, &rectangle, (ggez::mint::Point2 { x: 0.0, y: 0.0 },));
        }

        if self.selected {
            for i in 0..self.possiblemoves.iter().count() {
                if self.possiblemoves[0] != vec![] {
                    let currentxpos = self.possiblemoves[i][0];
                    let currentypos = self.possiblemoves[i][1];
                    //if !(self.possiblemoves[i][1] == 6 as i8 && self.possiblemoves[i][0] == 4 as i8) {
                    let rectangle = graphics::Mesh::new_rectangle(
                        ctx,
                        graphics::DrawMode::fill(),
                        graphics::Rect::new_i32(
                            currentxpos as i32 * GRID_CELL_SIZE.0 as i32,
                            currentypos as i32 * GRID_CELL_SIZE.1 as i32,
                            GRID_CELL_SIZE.0 as i32,
                            GRID_CELL_SIZE.1 as i32,
                        ),
                        RED,
                    )?;
                    graphics::draw(ctx, &rectangle, (ggez::mint::Point2 { x: 0.0, y: 0.0 },));
                    //}
                }
            }
        }

        // render updated graphics
        //graphics::present(ctx).expect("Failed to update graphics.");

        // Draw an image.
        /*for i in 0..8 {
            graphics::draw(ctx, &self.image1, (glam::Vec2::new(45.0*i as f32, 43.5),))?;
            graphics::draw(ctx, &self.image2, (glam::Vec2::new(45.0*i as f32, 44.8*6.0),))?;
        }*/
        /*graphics::draw(ctx, &self.image3, (glam::Vec2::new(45.0*4 as f32, 44.8*7.0),))?;
        graphics::draw(ctx, &self.image4, (glam::Vec2::new(45.0*4 as f32, 0.0),))?;
        graphics::draw(ctx, &self.image5, (glam::Vec2::new(45.0*3 as f32, 0.0),))?;
        graphics::draw(ctx, &self.image6, (glam::Vec2::new(45.0*3 as f32, 44.8*7.0),))?;
        graphics::draw(ctx, &self.image7, (glam::Vec2::new(45.0*2 as f32, 44.8*7.0),))?;
        graphics::draw(ctx, &self.image7, (glam::Vec2::new(45.0*5 as f32, 44.8*7.0),))?;
        graphics::draw(ctx, &self.image8, (glam::Vec2::new(45.0*1 as f32, 44.8*7.0),))?;
        graphics::draw(ctx, &self.image8, (glam::Vec2::new(45.0*6 as f32, 44.8*7.0),))?;
        graphics::draw(ctx, &self.image9, (glam::Vec2::new(0.0 as f32, 44.8*7.0),))?;
        graphics::draw(ctx, &self.image9, (glam::Vec2::new(45.0*7 as f32, 44.8*7.0),))?;
        graphics::draw(ctx, &self.image10, (glam::Vec2::new(45.0*2 as f32, 0.0),))?;
        graphics::draw(ctx, &self.image10, (glam::Vec2::new(45.0*5 as f32, 0.0),))?;
        graphics::draw(ctx, &self.image11, (glam::Vec2::new(45.0*1 as f32, 0.0),))?;
        graphics::draw(ctx, &self.image11, (glam::Vec2::new(45.0*6 as f32, 0.0),))?;
        graphics::draw(ctx, &self.image12, (glam::Vec2::new(0.0 as f32, 0.0),))?;
        graphics::draw(ctx, &self.image12, (glam::Vec2::new(45.0*7 as f32, 0.0),))?;*/

        /*if self.changevariable {
            graphics::draw(ctx, &self.image12, (glam::Vec2::new(45.0*self.xpos as f32, 44.8*self.ypos as f32),));
        }*/

        let what_image = |value: Option<lib::Piece>| -> graphics::Image {
            match value {
                Some(Piece {
                    piecetype: PieceType::Bishop,
                    color: lib::Color::White,
                }) => self.image7.clone(),
                Some(Piece {
                    piecetype: PieceType::Bishop,
                    color: lib::Color::Black,
                }) => self.image10.clone(),
                Some(Piece {
                    piecetype: PieceType::Pawn,
                    color: lib::Color::Black,
                }) => self.image1.clone(),
                Some(Piece {
                    piecetype: PieceType::Pawn,
                    color: lib::Color::White,
                }) => self.image2.clone(),
                Some(Piece {
                    piecetype: PieceType::King,
                    color: lib::Color::White,
                }) => self.image3.clone(),
                Some(Piece {
                    piecetype: PieceType::King,
                    color: lib::Color::Black,
                }) => self.image4.clone(),
                Some(Piece {
                    piecetype: PieceType::Queen,
                    color: lib::Color::Black,
                }) => self.image5.clone(),
                Some(Piece {
                    piecetype: PieceType::Queen,
                    color: lib::Color::White,
                }) => self.image6.clone(),
                Some(Piece {
                    piecetype: PieceType::Knight,
                    color: lib::Color::White,
                }) => self.image8.clone(),
                Some(Piece {
                    piecetype: PieceType::Knight,
                    color: lib::Color::Black,
                }) => self.image11.clone(),
                Some(Piece {
                    piecetype: PieceType::Rook,
                    color: lib::Color::White,
                }) => self.image9.clone(),
                Some(Piece {
                    piecetype: PieceType::Rook,
                    color: lib::Color::Black,
                }) => self.image12.clone(),
                _ => self.image5.clone(),
            }
        };

        for currentypos in 0..8 {
            for currentxpos in 0..8 {
                if self.game.board[7 - currentypos][currentxpos] != None {
                    //println!("{:?}", value);
                    //let kind_of_image = self.image12.clone();
                    let kind_of_image = what_image(self.game.board[7 - currentypos][currentxpos]);
                    // println!("{:?} {:?} {:?}", currentxpos, currentypos, value);
                    //println!("{:?}", kind_of_image);

                    graphics::draw(
                        ctx,
                        &kind_of_image,
                        (glam::Vec2::new(
                            45.0 * currentxpos as f32,
                            44.8 * (7 as f32 - currentypos as f32),
                        ),),
                    );

                    //println!("{:?} / {:?} / {:?}", key.file, key.rank, value);
                    //println!("{:?} / {:?} / {:?}", currentxpos, currentypos, value);
                }
            }
        }

        //println!("{:?}", self.game.board);

        //let dst1 = glam::Vec2::new(45.5*i, 43.5);
        //graphics::draw(ctx, &self.image1, (dst2,))?;
        if !self.gameover {
            //self.gameover = lib::Game::better_chess_ai(&mut self.game);
        }

        graphics::present(ctx)?;
        if !self.gameover && self.updatecallai {
            let now = Instant::now();
            self.gameover = lib::Game::better_chess_ai(&mut self.game);
            self.updatecallai = false;
            println!("Chess AI took: {:?}", now.elapsed());
        }
        let realcountdown = 5.0 * 1.0 - self.time.elapsed().as_secs() as f32;
        let currentminutes = ((4.0 * 60.0 - self.time.elapsed().as_secs() as f32) / 60.0).ceil();
        /*println!(
            "Current time: {:.0?}:{:.0?}",
            currentminutes,
            (59.0 - self.time.elapsed().as_secs() as f32)%60.0
        );*/
        if realcountdown == 0.0 {
            //println!("Time out!");
        }

        // draw text with dark gray colouring and center position
        graphics::draw(
            ctx,
            &state_text,
            graphics::DrawParam::default().dest(ggez::mint::Point2 {
                x: (SCREEN_SIZE.0 - text_dimensions.w as f32) / 2f32 as f32,
                y: (SCREEN_SIZE.0 - text_dimensions.h as f32) / 2f32 as f32,
            }),
        )
        .expect("Failed to draw text.");

        Ok(())
    }

    /// Update game on mouse click
    fn mouse_button_up_event(&mut self, ctx: &mut Context, button: MouseButton, x: f32, y: f32) {
        if button == MouseButton::Left && !self.gameover {
            println!("{} {}", x, y);
            self.xpos = ((x as i32 / 45) as f64).floor();
            self.ypos = ((y as i32 / 45) as f64).floor();
            println!("{} {}", self.xpos, self.ypos);
            if self
                .possiblemoves
                .contains(&vec![self.xpos as i8, self.ypos as i8])
                && self.selected
            {
                //if {
                self.selected = false;
                let string_to =
                    &convert_vec_to_string(&vec![vec![self.xpos as i8, self.ypos as i8]]).clone()
                        [0]
                    .to_string();
                let temporary_string_from = &self.string_from;
                println!("{:?} {:?}", temporary_string_from, string_to);
                let gamestate = lib::Game::make_move(
                    &mut self.game,
                    &temporary_string_from.to_string(),
                    string_to.to_string(),
                    true,
                );
                /*if gamestate == GameState::GameOver {
                    self.gameover = true;
                }*/
                //if self.game.GameState = Checkmate
                self.updatecallai = true;
                //eliasfl_chess::Game::make_move(&mut self.game, temporary_string_from.to_string(), string_to.to_string());
                //let ischeckmate = eliasfl_chess::Game::_is_checkmate(&mut self.game, self.game.active_color);
                //println!("{:?}", ischeckmate);
            } else if self.game.board[self.ypos as usize][self.xpos as usize] != None {
                if self.game.board[self.ypos as usize][self.xpos as usize]
                    .unwrap()
                    .color
                    == self.game.color
                {
                    self.string_from =
                        convert_vec_to_string(&vec![vec![self.xpos as i8, self.ypos as i8]])
                            .clone()[0]
                            .to_string();
                    self.selected = true;
                    //let  possiblemoves = eliasfl_chess::Game::get_possible_moves(&mut self.game, self.string_from.clone());
                    let (irrelevant, possiblemoves) = lib::Game::get_possible_moves(
                        &mut self.game,
                        &vec![self.xpos as i8, self.ypos as i8],
                        false,
                    );
                    println!("{:?}", possiblemoves);

                    /*let mut unwrappedpossible =  vec!["e2".to_string()];
                    if possiblemoves != None {
                        unwrappedpossible = possiblemoves.unwrap()
                    }*/
                    /*let mut realpossiblemoves = vec![];
                    //possiblemoves = possiblemoves.unwrap();
                    for i in 0..possiblemoves.len() {
                        realpossiblemoves.push(convert_string_to_vec(possiblemoves[i].to_string()));
                    }*/
                    //println!("{:?}", possiblemoves.unwrap()[0]);
                    self.possiblemoves = possiblemoves;
                    println!("{:?}", self.possiblemoves);
                    self.selectedvector = vec![self.xpos as i32, self.ypos as i32];
                }
            }
            println!("{:?}", self.selected);
            //let board = game.board;
            //println!("{:?}", board);
            //self.draw(ctx);
            /* check click position and update board accordingly */
        }
    }

    fn key_down_event(&mut self, ctx: &mut Context, key: KeyCode, mods: KeyMods, _: bool) {
        match key {
            // Quit if Shift+Ctrl+Q is pressed.
            KeyCode::R => {
                self.game = lib::Game::new();
                self.gameover = false;
                /*let pawntable = vec![
                    -30, -40, -40, -50, -50, -40, -40, -30, -30, -40, -40, -50, -50, -40, -40, -30,
                    -30, -40, -40, -50, -50, -40, -40, -30, -30, -40, -40, -50, -50, -40, -40, -30,
                    -20, -30, -30, -40, -40, -30, -30, -20, -10, -20, -20, -20, -20, -20, -20, -10,
                    20, 20, 0, 0, 0, 0, 20, 20, 20, 30, 10, 0, 0, 10, 30, 20,
                ];
                let mut newtable = vec![];
                for i in 0..pawntable.iter().count() {
                    newtable.push(pawntable[63 - i as usize])
                }
                println!("{:?}", newtable);*/
            }
            _ => (),
        }
        match key {
            // Quit if Shift+Ctrl+Q is pressed.
            KeyCode::A => {
                lib::Game::better_chess_ai(&mut self.game);
            }
            _ => (),
        }
    }
}

pub fn main() -> GameResult {
    let resource_dir = if let Ok(manifest_dir) = env::var("CARGO_MANIFEST_DIR") {
        let mut path = path::PathBuf::from(manifest_dir);
        path.push("resources");
        path
    } else {
        path::PathBuf::from("./resources")
    };
    let bishop = PROJECT_DIR.get_file("black_bishop.png").unwrap();

    let cb = ggez::ContextBuilder::new("drawing", "ggez")
        .add_resource_path(resource_dir)
        .window_setup(
            ggez::conf::WindowSetup::default()
                .title("Skynet") // Set window title "Schack"
                .icon("/skynet.png"), // Set application icon
        )
        .window_mode(
            ggez::conf::WindowMode::default()
                .dimensions(SCREEN_SIZE.0, SCREEN_SIZE.1) // Set window dimenstions
                .resizable(false), // Fixate window size
        );

    let (mut ctx, events_loop) = cb.build()?;

    println!("{}", graphics::renderer_info(&ctx)?);
    let state = MainState::new(&mut ctx).unwrap();
    event::run(ctx, events_loop, state)
}
