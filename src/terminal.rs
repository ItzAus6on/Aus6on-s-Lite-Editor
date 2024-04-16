use std::io::{self, Write};
use crossterm::terminal::{
    self, 
    ClearType
};
use crossterm::event::{
    self,
    Event,
    KeyCode,
    KeyEvent,
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

    pub fn read_key() -> io::Result<char> {
        loop {
            if let Event::Key(KeyEvent{
                code: KeyCode::Char(c),
                ..
            }) = event::read()? 
            {
                return Ok(c);
            }
        }
    }

    pub fn clear_screen() {
        terminal::Clear(ClearType::All);
    }

    pub fn cursor_position(x: u16, y: u16) {
        let x = x.saturating_add(1);
        let y = y.saturating_add(1);
        cursor::MoveTo(x, y);
    }

    pub fn flush() -> Result<(), std::io::Error> {
        std::io::stdout().flush()
    }

    pub fn cursor_hide() {
        println!("{}", cursor::Hide);
    }

    pub fn cursor_show() {
        println!("{}", cursor::Show);
    }

    pub fn clear_current_line() {
        terminal::Clear(ClearType::CurrentLine);
    }

}