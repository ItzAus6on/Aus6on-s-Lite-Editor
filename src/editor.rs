use crate::Document;
use crate::Terminal;
use crate::Row;
use crossterm::{event::KeyCode, terminal::{
    disable_raw_mode, 
    enable_raw_mode, 
}};
use core::panic;
use std::env;

const VERSION: &str = "INDEV_0429";

#[derive(Default)]
pub struct Position{
    pub x: usize,
    pub y: usize,
}


pub struct Editor {
    should_quit: bool,
    terminal: Terminal,
    cursor_position: Position,
    offset: Position,
    document: Document,
}

impl Editor {
    pub fn default() -> Editor{
        let args: Vec<String> = env::args().collect();
        let document = if args.len() > 1 {
            let file_name = &args[1];
            Document::open(&file_name).unwrap_or_else(|err|{
                die(err);
                std::process::exit(1);
            })
        } else {
            Document::default()
        };

        Self {
            should_quit: false,
            terminal: Terminal::default().expect("Failed to initialize terminal"),
            document,
            cursor_position: Position::default(),
            offset: Position::default(),
        }
    }

    pub fn run(&mut self) {
        enable_raw_mode().unwrap();
        loop {
            if let Err(e) = Self::refresh_screen(self) {
                die(e);
            }

            if let Err(e) = Self::keypress(self) {
                die(e);
            }

            if self.should_quit {
                Terminal::clear_screen();
                disable_raw_mode().unwrap();
                println!("Goodbye. \r");
                break;
            }
        }
    }

    fn keypress(&mut self) -> Result<(), std::io::Error> {
        let pressed_key = Terminal::read_key()?;
        match pressed_key {
            KeyCode::Char('q')  => self.should_quit = true,
            KeyCode::Up 
            | KeyCode::Down 
            | KeyCode::Left 
            | KeyCode::Right
            | KeyCode::PageUp
            | KeyCode::PageDown
            | KeyCode::End
            | KeyCode::Home => self.move_cursor(pressed_key),
            _ => (),
        }
        self.scroll();
        Ok(())
    }

    fn scroll(&mut self) {
        let Position {x, y} = self.cursor_position;
        let height = self.terminal.size().height as usize;
        let width = self.terminal.size().width as usize;
        let offset = &mut self.offset;

        if y < offset.y {
            offset.y = y;
        } else if y > offset.y.saturating_add(height) {
            offset.y = offset.y.saturating_add(height).saturating_add(1);
        }

        if x < offset.x {
            offset.x = x;
        } else if x > offset.x.saturating_add(width) {
            offset.x = offset.x.saturating_add(width).saturating_add(1);
        }
    }

    fn move_cursor(&mut self, key: KeyCode) {
        let terminal_height = self.terminal.size().height as usize;
        let Position {mut x, mut y} = self.cursor_position;
        let height = self.document.len();
        let mut width = if let Some(row) = self.document.row(y) {
            row.len()
        } else {
            0
        };
        match key {
            KeyCode::Up => y = y.saturating_sub(1),
            KeyCode::Down => {
                if y < height {
                    y = y.saturating_add(1);
                }
            }
            KeyCode::Left => {
                if x > 0 {
                    x -= 1;
                } else if y > 0 {
                    y -= 1;
                    if let Some(row) = self.document.row(y) {
                        x = row.len();
                    } else {
                        x = 0;
                    }
                }
            }
            KeyCode::Right => {
                if x < width {
                    x += 1;
                } else if y < height {
                    y += 1;
                    x = 0;
                }
            }
            KeyCode::PageUp => {
                y = if y > terminal_height {
                    y - terminal_height
                } else {
                    0
                }
            }
            KeyCode::PageDown => {
                y = if y.saturating_add(terminal_height) > height {
                    y + terminal_height as usize
                } else {
                    height
                }
            }
            KeyCode::End => x = width,
            KeyCode::Home => x = 0,
            _ => (),
        }
        width = if let Some(row) = self.document.row(y) {
            row.len()
        } else {
            0
        };
        if x > width {
            x = width;
        }
        self.cursor_position = Position {x, y}
    }

    fn refresh_screen(&self) -> Result<(), std::io::Error>{
        Terminal::cursor_hide();
        Terminal::cursor_position(&Position::default());
        self.draw_rows();
        Terminal::cursor_position(&Position {
            x: self.cursor_position.x.saturating_add(self.offset.x),
            y: self.cursor_position.y.saturating_add(self.offset.y),
        });
        Terminal::cursor_show();
        Terminal::flush()
    }

    fn draw_row(&self, row: &Row) {
        let width = self.terminal.size().width as usize;
        let start = self.offset.x;
        let end = self.offset.x + width;
        let row = row.render(start, end);
        println!("{}\r", row);
    }

    fn draw_rows(&self) {
        let height = self.terminal.size().height;
        for terminal_row in 0..height - 1{
            Terminal::clear_current_line();
            if let Some(row) = self.document.row(terminal_row as usize + self.offset.y) {
                self.draw_row(row);
            } else if self.document.is_empty() && terminal_row == height / 3{
                self.draw_welcome_message();
            } else {
                println!("~\r");
            }
        }
    }   

    fn draw_welcome_message(&self) {
        let mut welcome_message = format!("Aus6on's Lite editor -- {}", VERSION);
        let width = self.terminal.size().width as usize;
        let len = welcome_message.len();
        let padding = width.saturating_sub(len) / 2;
        let spaces = " ".repeat(padding.saturating_sub(1));
        welcome_message = format!("~{}{}", spaces, welcome_message);
        welcome_message.truncate(width);
        println!("{}\r", welcome_message);
    }
}

fn die(e: std::io::Error) {
    Terminal::clear_screen();
    panic!("Application Error: {}", e);
}
