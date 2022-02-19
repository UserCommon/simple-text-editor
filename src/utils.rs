use std::fs::{File, self};

use std::io::{self,};
use std::io::{prelude::*};
use termion::color::{self, Color};


use crate::Document;
use crate::Editor;

pub const UPPER_PADDING: u16 = 1;
pub const TAB_BAR_BG_COLOR: color::Rgb = color::Rgb(51, 51, 61);

pub const BOTTOM_PADDING: u16 = 2;
pub const MESSAGE: &str = "CTRL - C = EXIT";
pub const STATUS_BAR_BG_COLOR: color::Rgb = color::Rgb(129, 255, 157);
pub const STATUS_BAR_FG_COLOR: color::Rgb = color::Rgb(70, 70, 70);


pub const EDITOR_DEADZONE_COLOR: color::Rgb = color::Rgb(60,60,60);
pub const EDITOR_BG_COLOR: color::Rgb = color::Rgb(30, 30, 36);
pub const EDITOR_FG_COLOR: color::Rgb = color::Rgb(255, 255, 255);


pub struct App {
    args: Vec<String>,
    config: Option<File>,
    documents: Vec<Document>,
    editor: Editor, 
}

impl App {
    pub fn build(args: Vec<String>, config: Option<File>) -> Result<Self, io::Error> {
        Ok(
            App {
                args: args.clone(),
                config,
                documents: Vec::<Document>::new(),
                editor: Editor::create()? 
            }
        )
    }

    pub fn run(&mut self) -> Result<(), io::Error>{
        if self.args.len() > 1 {
            let filename = self.args[1].clone();   
            let mut doc = Document::create(filename)?;
            doc.read()?;
            // Добавить позже возможность переключать документы
            self.editor.document = Some(doc); 
        } 

        self.editor.run()?;
        Ok(())
    }
}


