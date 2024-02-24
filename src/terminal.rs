use crate::Position;

use std::io::{self, Write};
use termion::event::Key;
use termion::input::TermRead;
use termion::raw::{IntoRawMode, RawTerminal};

pub struct Size {
    pub width: u16,
    pub height: u16,
}

pub struct Terminal {
    size: Size,
    _stdout: RawTerminal<io::Stdout>,
}

impl Default for Terminal {
    fn default() -> Self {
        let size = termion::terminal_size().expect("Failed to obtain terminal size");

        Self {
            size: Size {
                width: size.0,
                height: size.1,
            },
            _stdout: io::stdout()
                .into_raw_mode()
                .expect("Failed to initialize terminal"),
        }
    }
}

impl Terminal {
    pub fn size(&self) -> &Size {
        &self.size
    }

    pub fn clear_screen() {
        print!("{}", termion::clear::All);
    }

    pub fn cursor_position(pos: &Position) {
        let Position { mut x, mut y } = *pos;
        x = x.saturating_add(1);
        y = y.saturating_add(1);

        let x = x as u16;
        let y = y as u16;

        print!("{}", termion::cursor::Goto(x, y));
    }

    pub fn flush() -> Result<(), io::Error> {
        io::stdout().flush()
    }

    pub fn read_key() -> Result<Key, io::Error> {
        loop {
            if let Some(key) = io::stdin().lock().keys().next() {
                return key;
            }
        }
    }

    pub fn cursor_hide() {
        print!("{}", termion::cursor::Hide);
    }

    pub fn cursor_show() {
        print!("{}", termion::cursor::Show);
    }

    pub fn clear_current_line() {
        print!("{}", termion::clear::CurrentLine);
    }
}
