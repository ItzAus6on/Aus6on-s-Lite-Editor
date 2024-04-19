use crate::Terminal;
use crossterm::{event::KeyCode, terminal::{
    disable_raw_mode, 
    enable_raw_mode, 
}};

const VERSION: &str = "INDEV_0419";

pub struct Position{
    pub x: usize,
    pub y: usize,
}


pub struct Editor {
    should_quit: bool,
    terminal: Terminal,
    cursor_position: Position,
}

impl Editor {
    pub fn default() -> Editor{
        Self {
            should_quit: false,
            terminal: Terminal::default().expect("Failed to initialize terminal"),
            cursor_position: Position {x: 0, y: 0},
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
        Ok(())
    }

    fn move_cursor(&mut self, key: KeyCode) {
        let Position {mut x, mut y} = self.cursor_position;
        let size = self.terminal.size();
        let height = size.height.saturating_sub(1) as usize;
        let width = size.width.saturating_sub(1) as usize;
        match key {
            KeyCode::Up => y = y.saturating_sub(1),
            KeyCode::Down => {
                if y < height {
                    y = y.saturating_add(1);
                }
            }
            KeyCode::Left => x = x.saturating_sub(1),
            KeyCode::Right => {
                if x < width {
                    x = x.saturating_add(1);
                }
            }
            KeyCode::PageUp => y = 0,
            KeyCode::PageDown => y = height,
            KeyCode::End => x = width,
            KeyCode::Home => x = 0,
            _ => (),
        }
        self.cursor_position = Position {x, y}
    }

    fn refresh_screen(&self) -> Result<(), std::io::Error>{
        Terminal::cursor_hide();
        Terminal::cursor_position(&Position {x: 0, y: 0});
        self.draw_rows();
        Terminal::cursor_position(&self.cursor_position);
        Terminal::cursor_show();
        Terminal::flush()
    }

    fn draw_rows(&self) {
        let height = self.terminal.size().height;
        for row in 0..height - 1{
            Terminal::clear_current_line();
            if row == height / 3 {
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
