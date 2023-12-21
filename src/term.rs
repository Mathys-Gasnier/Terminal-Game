use std::{
    io::{ self, Write },
    time::Duration,
    str, vec
};
use crossterm::{
    QueueableCommand,
    ExecutableCommand,
    cursor,
    terminal::{ self, enable_raw_mode, disable_raw_mode },
    event::{
        read, poll,
        Event, EnableMouseCapture, DisableMouseCapture
    },
    style
};

pub enum WrapMode {
    Normal,
    Cut
}

pub struct Term {
    stdout: io::Stdout
}

impl Term {

    pub fn new() -> io::Result<Self> {
        let mut term = Self {
            stdout: io::stdout()
        };

        term.enable()?;

        Ok(term)
    }

    fn enable(&mut self) -> io::Result<&mut Self> {
        enable_raw_mode()?;
        self.stdout.execute(EnableMouseCapture)?;
    
        self.clear()?
            .flush()?;

        Ok(self)
    }

    pub fn poll_event(&mut self) -> io::Result<Option<Event>> {
        if poll(Duration::from_millis(1_000))? {
            return Ok(Some(read()?))
        }
        Ok(None)
    }

    pub fn clear(&mut self) -> io::Result<&mut Self> {
        self.stdout
            .queue(terminal::Clear(terminal::ClearType::All))?
            .queue(cursor::MoveTo(0, 0))?;
        Ok(self)
    }

    pub fn flush(&mut self) -> io::Result<&mut Self> {
        self.stdout.flush()?;
        Ok(self)
    }

    pub fn move_cursor(&mut self, x: u16, y: u16) -> io::Result<&mut Self> {
        self.stdout.queue(cursor::MoveTo(x, y))?;
        Ok(self)
    }

    pub fn write(&mut self, str: &str) -> io::Result<&mut Self> {
        self.stdout.queue(style::Print(str))?;
        Ok(self)
    }

    pub fn print(&mut self, str: &str, x: u16, y: u16) -> io::Result<&mut Self> {
        self.move_cursor(x, y)?
            .write(str)?;
        Ok(self)
    }

    pub fn print_wrap(&mut self, str: &str, x: u16, y: u16, max_width: u16, wrap_mode: WrapMode) -> io::Result<&mut Self> {
        debug_assert!(max_width > 0, "max_width of print_wrap should be > 0");

        str.chars()
            .collect::<Vec<char>>()
            .chunks(max_width as usize)
            .enumerate()
            .take(match wrap_mode {
                WrapMode::Normal => usize::MAX,
                WrapMode::Cut => 1
            })
            .map(|(idx, segment)| -> io::Result<()> {
                self.print(&segment.iter().collect::<String>(), x, y + idx as u16)?;
                Ok(())
            })
            .collect::<io::Result<()>>()?;
        
        Ok(self)
    }

    pub  fn line(&mut self, char: char, x: u16, y: u16, length: u16, vertical: bool) -> io::Result<&mut Self> {
        if !vertical {
            self.print(&String::from_utf8(vec![char as u8; length as usize]).expect("Failed to create str, line"), x, y)?;
        }else {
            for y_off in 0..length {
                self.print(&char.to_string(), x, y + y_off)?;
            }
        }
        Ok(self)
    }

    pub fn border_rect(&mut self, char: char, x: u16, y: u16, width: u16, height: u16) -> io::Result<&mut Self> {
        self.line(char, x, y, width, false)?
            .line(char, x, y + height, width, false)?
            .line(char, x, y, height, true)?
            .line(char, x + width, y, height, true)?;
        Ok(self)
    }

    pub fn fill_rect(&mut self, char: char, x: u16, y: u16, width: u16, height: u16) -> io::Result<&mut Self> {
        for y_off in 0..height {
            self.line(char, x, y + y_off, width, false)?;
        }
        Ok(self)
    }

    fn disable(&mut self) -> io::Result<&mut Self> {
        self.clear()?
            .flush()?
            .stdout.execute(DisableMouseCapture)?;
        disable_raw_mode()?;

        Ok(self)
    }

}

impl Drop for Term {
    fn drop(&mut self) {
        self.disable().expect("Terminal failed to be reset");
    }
}