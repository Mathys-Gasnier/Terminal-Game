use std::{
    io::{ self, Write },
    time::Duration,
    str, vec, cmp
};
use crossterm::{
    QueueableCommand,
    ExecutableCommand,
    cursor::{ self, position },
    terminal::{ self, enable_raw_mode, disable_raw_mode, size },
    event::{
        read, poll,
        Event, EnableMouseCapture, DisableMouseCapture, KeyCode, KeyEvent, MouseEvent, MouseEventKind
    },
    style
};

mod term;
use term::{Term, WrapMode};

struct Game {
    term: Term,
    command_buffer: String,
    line_buffer: Vec<String>,
    scroll_offset: u16,
    event: Option<Event>
}

impl Game {

    pub fn new() -> io::Result<Self> {
        Ok(Self {
            term: Term::new()?,
            command_buffer: String::new(),
            line_buffer: vec![],
            scroll_offset: 0,
            event: None
        })
    }

    pub fn update(&mut self) -> io::Result<bool> {
        self.event = self.term.poll_event()?;

        if let Some(Event::Key(KeyEvent { code, .. })) = &self.event {
            match code {
                KeyCode::Esc => return Ok(true),
                KeyCode::Enter => {
                    self.line_buffer.push(format!("# {}", self.command_buffer));
                    self.command_buffer.clear();
                }
                KeyCode::Char(char) => self.command_buffer.push(char.clone()),
                _ => ()
            }
        }else if let Some(Event::Mouse(MouseEvent { kind, .. })) = &self.event {
            match kind {
                MouseEventKind::ScrollUp => {
                    self.scroll_offset += 1;
                    let height = size()?.1 - 4;
                    if self.line_buffer.len() <= height as usize {
                        self.scroll_offset = 0;
                    }else {
                        self.scroll_offset = cmp::min(self.scroll_offset, self.line_buffer.len() as u16 - height);
                    }
                },
                MouseEventKind::ScrollDown => {
                    if self.scroll_offset != 0 {
                        self.scroll_offset -= 1;
                    }
                },
                _ => ()
            }
        }

        Ok(false)
    }

    pub fn draw(&mut self) -> io::Result<()> {
        self.term.clear()?;

        for (idx, line) in self.line_buffer
            .iter()
            .rev()
            .skip(self.scroll_offset as usize)
            .take(size()?.1 as usize - 4)
            .rev()
            .enumerate()
        {
            self.term.print_wrap(&line, 1, 1 + idx as u16, size()?.0 - 2, WrapMode::Normal)?;
        }

        /*
        if let Some(event) = &self.event {
            self.term.print(&format!("{:?}", event), 1, 1)?;
        }else {
            self.term.print("...", 1, 1)?;
        }
        */

        self.term.move_cursor(1 + self.command_buffer.len() as u16, size()?.1 - 2)?
            .border_rect('#', 0, 0, size()?.0, size()?.1)?
            .line('#', 1, size()?.1 - 3, size()?.0 - 2, false)?
            .print(&self.command_buffer, 1, size()?.1 - 2)?
            .flush()?;
        Ok(())
    }

}

fn main() -> io::Result<()> {
    let mut game = Game::new()?;

    game.draw()?;
    loop {
        if game.update()? {
            break;
        }
        game.draw()?;
    }

    Ok(())
}
