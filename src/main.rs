#![cfg_attr(debug_assertions, allow(dead_code, unused_imports))]

mod utils;
mod document;
mod terminal;

use utils::*;
use document::Document;
use terminal::Editor;

use std::env;
use std::io::{
    prelude::*,
    Stdin,
    Write,
    stdout
};

fn main() -> Result<(), std::io::Error>{
    let mut app = App::build(
        env::args().collect(), //ARGS
        None,                  //CONFIG 
    )?;
    app.run()
}
