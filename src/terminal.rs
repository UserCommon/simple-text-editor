use std::cell::RefCell;

use std::io::{
    self,
    Stdout,
    Stdin,
    Write,
    stdout,
    stdin,
    Read,
    Error
};


use termion::{
    raw::{
        IntoRawMode,
        RawTerminal,
    },
    event::Key,
    input::TermRead,
    terminal_size,
    style::{
        Bold,
        Italic,
    },
    color,
};

use crate::{Document, App};
use crate::{BOTTOM_PADDING, UPPER_PADDING,
            STATUS_BAR_FG_COLOR, STATUS_BAR_BG_COLOR, MESSAGE,
            TAB_BAR_BG_COLOR,
            EDITOR_BG_COLOR, EDITOR_DEADZONE_COLOR};


pub struct TerminalSize {
    pub width: u16,
    pub height: u16
}


#[derive(Default)]
pub struct Position {
    pub x: u16,
    pub y: u16
}

impl std::fmt::Display for Position {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "(Symb: {}| Line: {})", self.x, self.y)
    }
}


pub struct Editor {
    pub runnig: bool,
    pub document: Option<Document>,
    pub raw_stdout: RawTerminal<Stdout>,
    pub terminal_size: TerminalSize,
    pub cursor_position: Position,

    pub last_number: u16
}

impl Editor {

    pub fn create() -> Result<Self, io::Error> {
        let (width, height) = terminal_size()?;
        Ok(
            Editor {
                runnig: true,
                document: None,
                raw_stdout: stdout().into_raw_mode()?,
                terminal_size: TerminalSize {
                                    width: width,
                                    height: height.saturating_sub(BOTTOM_PADDING+UPPER_PADDING)
                               },
                cursor_position: Position::default(),
                last_number: 0
            }
        )
    }
 
    pub fn run(&mut self) -> Result<(), io::Error> {
        while self.runnig {
            self.render()?;
            self.process_key()?;
        }

        Ok(())
    }

    pub fn render(&mut self) -> Result<(), io::Error> {
        let deadzone_padding = 5;
        print!("{}", termion::cursor::Goto::default());
        self.draw();
        print!("{}", termion::cursor::Goto(
            self.cursor_position.x.saturating_add(deadzone_padding),
            self.cursor_position.y.saturating_add(1+UPPER_PADDING),
        ));
        

        self.raw_stdout.flush()
        
    }

    fn draw(&mut self) {
        self.draw_tab_space();
        match self.document {
            None => { self.draw_welcome_screen(); },
            _    => { self.draw_document().unwrap(); }
        }
        self.draw_status_bar();
    }

    fn draw_document(&mut self) -> Result<(), io::Error> {
        let doc = self.document.as_ref().unwrap();
        self.last_number = doc.content.last().unwrap().id as u16;
        let num_padding_width = 2 + self.last_number;

        for line in &doc.content {
            
            // СДЕЛАТЬ РАЗБИВКУ НА СЛОВА ДЛЯ БУДУЩЕГО АНАЛИЗАТОРА
            let mut print_line = String::from("");
            
            for symb in &line.content {
                let mut word = String::from("");

                if symb.symbol as char != ' ' {
                    word.push(symb.symbol as char)
                }
                else {
                    print_line = format!("{} ", word);
                }
                print_line.push(symb.symbol as char); 
            }
            
            let end_spaces = " ".repeat(self.terminal_size.width.saturating_sub(print_line.len() as u16) as usize);
            let start_spaces = " ".repeat((num_padding_width - line.id.to_string().len() as u16).into());
            println!("{}{}{}{}", start_spaces, line.id, print_line.trim(), end_spaces);
        }
        self.raw_stdout.flush()

    }

    fn draw_welcome_screen(&mut self) {
        self.last_number = self.terminal_size.height;
        let num_padding_width = 2+self.last_number.to_string().len();

        let mut num = 1;
        print!("{}{}", termion::clear::CurrentLine, color::Bg(EDITOR_BG_COLOR));
        for row_num in 0..self.terminal_size.height {
            let num_padding = " ".repeat(num_padding_width - num.to_string().len());
            print!("{}", termion::clear::CurrentLine);
            if row_num == self.terminal_size.height/2 {
                let welcome = "Thanks for using simple text editor";
                let padding = " ".repeat(
                                    (self.terminal_size.width/2+1) as usize - welcome.len() / 2
                                  );
                println!("{}{}{}{}{}\r", num_padding, num, color::Bg(EDITOR_BG_COLOR), padding, welcome);
            }
            else {
                println!("{}{}{}\r", num_padding, num, color::Bg(EDITOR_BG_COLOR));
            }
            num += 1;
        }
 
    }

    fn draw_status_bar(&mut self) {
        print!("{}", termion::clear::CurrentLine);
        let status_message = format!("Cursor {}", self.cursor_position);
        let end_spaces = " ".repeat(
            self.terminal_size.width.saturating_sub(status_message.len() as u16) as usize              
        );
        let status = format!("{}{}", status_message, end_spaces);
        print!("{}{}", color::Bg(STATUS_BAR_BG_COLOR), color::Fg(STATUS_BAR_FG_COLOR));
        println!("{}\r", status);
        print!("{}{}", color::Bg(color::Reset), color::Fg(color::Reset));
        print!("{}", termion::clear::CurrentLine);
        print!("{}\r", String::from(MESSAGE));
    }

    fn draw_tab_space(&mut self) {
        print!("{}", termion::clear::CurrentLine);
        
        let message = "TAB SPACE";
        let end_spaces = " ".repeat(self.terminal_size.width.saturating_sub(message.len() as u16) as usize);
         
        print!("{}", color::Bg(TAB_BAR_BG_COLOR));
        print!("{}\r", format!("{}{}", message, end_spaces));
        for _ in 0..UPPER_PADDING {
            println!();
        }

    }

    fn process_key(&mut self) -> Result<(), io::Error> {
        match self.next_key()? {
            Key::Ctrl('c')  => { self.runnig = false; },
            Key::Char(c) => { print!("{}", c); self.raw_stdout.flush().unwrap();},
            Key::Up => {
                self.cursor_position.y = self.cursor_position.y.saturating_sub(1);
            },
            Key::Down => {
                if self.cursor_position.y < self.terminal_size.height - 1 {
                    self.cursor_position.y = self.cursor_position.y.saturating_add(1);
                }
            },
            Key::Left => {
                self.cursor_position.x = self.cursor_position.x.saturating_sub(1);
            },
            Key::Right => {
                if self.cursor_position.x < self.terminal_size.width - 1 {
                    self.cursor_position.x = self.cursor_position.x.saturating_add(1);
                }
            },
            _         => ()
        }

        Ok(())
    }

    fn next_key(&self) -> Result<Key, io::Error> {
        match stdin().keys().next() {
            Some(key) => key,
            None      => Err(io::Error::new(
                                io::ErrorKind::Other,
                                "Weird input"
                            ))
        } 
    }

}
