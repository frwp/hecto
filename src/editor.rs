use core::panic;
use std::io::{stdin, stdout, Write};

use termion::event::Key;
use termion::input::TermRead;
use termion::raw::IntoRawMode;

pub struct Editor {
    should_quit: bool,
}

impl Editor {
    pub fn run(&mut self) {
        let _stdout = stdout().into_raw_mode().unwrap();

        loop {
            if let Err(error) = self.refresh_screen() {
                die(error);
            }
            if let Err(error) = self.process_keypress() {
                die(error);
            }
            if self.should_quit {
                break;
            }
        }
    }

    pub fn default() -> Self {
        Editor { should_quit: false }
    }

    fn process_keypress(&mut self) -> Result<(), std::io::Error> {
        let pressed_key = read_key()?;
        match pressed_key {
            Key::Ctrl('q') => self.should_quit = true,
            _ => (),
        }
        Ok(())
    }

    fn refresh_screen(&self) -> Result<(), std::io::Error> {
        print!("{}{}", termion::clear::All, termion::cursor::Goto(1, 1));
        if self.should_quit {
            println!("Goodbye.\r");
        }
        std::io::stdout().flush()
    }
}

fn read_key() -> Result<Key, std::io::Error> {
    loop {
        if let Some(key) = stdin().lock().keys().next() {
            return key;
        }
    }
}

fn die(e: std::io::Error) {
    print!("{}", termion::clear::All);
    panic!("{e}");
}
