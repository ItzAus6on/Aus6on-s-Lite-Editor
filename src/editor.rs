use crate::Terminal;
use crossterm::terminal::{
    disable_raw_mode, 
    enable_raw_mode, 
};

const VERSION: &str = "INDEV_0416";

pub struct Editor {
    should_quit: bool,
    terminal: Terminal,
}

impl Editor {
    pub fn default() -> Editor{
        Self {
            should_quit: false,
            terminal: Terminal::default().expect("Failed to initialize terminal"),
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
                disable_raw_mode().unwrap();
                break;
            }
        }
    }

    fn keypress(&mut self) -> Result<(), std::io::Error> {
        let pressed_key = Terminal::read_key()?;
        match pressed_key {
            'q' => {
                self.should_quit = true;
                Ok(())
            },
            _ => Ok(()),
        }
    }

    fn refresh_screen(&self) -> Result<(), std::io::Error>{
        Terminal::cursor_hide();
        Terminal::cursor_position(0, 0);
        if self.should_quit {
            Terminal::clear_screen();
            println!("Goodbye. \r");
        } else {
            self.draw_rows();
            Terminal::cursor_position(0, 0);
        }
        Terminal::cursor_show();
        Terminal::flush()
    }

    fn draw_rows(&self) {
        let height = self.terminal.size().height;
        for row in 0..height - 1 {
            Terminal::clear_current_line();
            if row == height / 3 {
                let welcome_message = format!("Lite editor -- version {}", VERSION);
                let width = std::cmp::min(self.terminal.size().width as usize, welcome_message.len());                            
                println!("{}\r", &welcome_message[..width])
            } else {
                println!("~\r");
            }
        }
    }   
}

fn die(e: std::io::Error) {
    Terminal::clear_screen();
    panic!("{}", e);
}
