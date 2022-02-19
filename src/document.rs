#[allow(unused_imports)]
use std::path::Path;
use std::fs::{File, self};

use std::io::{
    self,
    Write,
    Read,
    Stdout,
    Stdin,
    Error
};

use termion::{
    raw::{IntoRawMode},
    screen::{self},
};


#[derive(Debug, Copy, Clone)]
pub struct Symbol {
    
    /// Symbol - Structure that contains number and symbol that has type u8

    pub id: usize,
    pub symbol: u8
}

impl Symbol {

    pub fn create_symbol(symbol: u8, id: usize) -> Self {
        Symbol {
            id,
            symbol
        }
    }

}


#[derive(Debug, Clone)]
pub struct Line {
    
    /// Line - Structure that contains number of line and content, that consists of Symbol
    /// structure Vector

    pub id: usize,
    pub content: Vec<Symbol>
}

impl Line {
    
    pub fn create(string: &String, id: usize) -> Self {
        let mut line = Line {
            id,
            content: Vec::<Symbol>::new()
        };
        
        let mut index: usize = 1;
        for i in string.chars() {
            line.content.push(Symbol {
                id: index,
                symbol: i as u8
            });
            index += 1;
        }

        line
    }

}


#[derive(Debug)]
pub struct Document {
    
    /// Document - Structure that contains file, his name and content, that consists of a Vector of
    /// Line structure

    pub filename: String,
    pub file: File,
    pub content: Vec<Line>
}


impl Document {
    pub fn create(filename: String) -> Result<Self, io::Error> {
        Document::create_if_not_exist(&filename.clone())?;
        Ok (
            Document {
                filename: filename.clone(),
                file: File::open(filename.clone()).expect("Unable to open file!"),
                content: Vec::<Line>::new()
            }
        )
    }

    pub fn read(&mut self) -> Result<(), io::Error> {
        let file = fs::read(&self.filename).expect("Something went wrong!");
        

        let mut index: usize = 1;
        let mut string = String::from("");

        for symb in file {
            string.push(symb as char);
            if symb == '\n' as u8 {
                self.content.push(
                    Line::create(&string, index)
                );
                index += 1;
                string = String::from("");
                continue;
            }
        }
        Ok(())
    } 

    fn create_if_not_exist(filename: &String) -> std::io::Result<()> {
        let result = Ok(());
        if !Document::chech_existance(filename) {
            File::create(filename)?;
        }
        result
    }

    fn chech_existance(filename: &String) -> bool {
        Path::new(filename).exists()
    }

    fn save(&self) {
        //
    }
}
