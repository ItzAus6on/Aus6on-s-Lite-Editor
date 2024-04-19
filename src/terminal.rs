use std::io::{self, Write};
use crate::Position;
use crossterm::terminal::{
    self, 
    ClearType
};
use crossterm::event::{
    self,
    Event,
    KeyCode,
};
use crossterm::cursor;

pub struct Size {
    pub width: u16,
    pub height: u16,
}

pub struct Terminal {
    size: Size,
}

impl Terminal{
    pub fn default() -> Result<Self, std::io::Error> {
        let size = crossterm::terminal::size()?;
        Ok(Self {
            size: Size {
                width: size.0,
                height: size.1,
            },
        })
    }

    pub fn size(&self) -> &Size{
        &self.size
    }

    pub fn read_key() -> io::Result<KeyCode> {
        loop {
            if let Event::Key(key) = event::read()? 
            {   
                return Ok(key.code)
            }
        }
    }

    pub fn clear_screen() {
        terminal::Clear(ClearType::All);
    }

    pub fn cursor_position(position: &Position) {
        let Position{mut x, mut y} = position;
        x = x.saturating_add(1);
        y = y.saturating_add(1);
        let x = x as u16;
        let y = y as u16;
        print!("{}", cursor::MoveTo(x, y));
    }

    pub fn flush() -> Result<(), std::io::Error> {
        std::io::stdout().flush()
    }

    pub fn cursor_hide() {
        print!("{}", cursor::Hide);
    }

    pub fn cursor_show() {
        print!("{}", cursor::Show);
    }

    pub fn clear_current_line() {
        print!("{}", terminal::Clear(ClearType::CurrentLine));
    }

}