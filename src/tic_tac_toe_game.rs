extern crate termcolor;
use std::io::Write;
use termcolor::{Color, ColorChoice, ColorSpec, StandardStream, WriteColor};

pub(crate) struct TicTacToeGame {
    player_one_color: Option<Color>,
    player_two_color: Option<Color>,
}
impl TicTacToeGame {
    pub fn new() {}
}
