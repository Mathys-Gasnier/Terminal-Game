use std::{io, cmp};
use crossterm::{event::{Event, KeyEvent, KeyCode, MouseEvent, MouseEventKind}, terminal::size};
use crate::{term::{Term, WrapMode}, lexer::Lexer, parser::Parser};


pub struct Game {
    term: Term,
    command_buffer: String,
    line_buffer: Vec<String>,
    cursor_offset: u16,
    scroll_offset: u16,
    event: Option<Event>
}

impl Game {

    pub fn new() -> io::Result<Self> {
        Ok(Self {
            term: Term::new()?,
            command_buffer: String::new(),
            line_buffer: vec![],
            cursor_offset: 0,
            scroll_offset: 0,
            event: None
        })
    }

    fn scroll_down(&mut self) {
        if self.scroll_offset != 0 {
            self.scroll_offset -= 1;
        }
    }

    fn scroll_up(&mut self) {
        self.scroll_offset += 1;
        let height = size().expect("Failed to get terminal size").1 - 2;
        if self.line_buffer.len() <= height as usize {
            self.scroll_offset = 0;
        }else {
            self.scroll_offset = cmp::min(self.scroll_offset, self.line_buffer.len() as u16 - height);
        }
    }

    fn cursor_left(&mut self) {
        if self.cursor_offset >= self.command_buffer.len() as u16 {
            return;
        }
        self.cursor_offset += 1;
    }

    fn cursor_right(&mut self) {
        if self.cursor_offset <= 0 {
            return;
        }
        self.cursor_offset -= 1;
    }

    fn char_at_cursor(&mut self, char: char) {
        if !char.is_ascii() {
            return;
        }
        self.command_buffer.insert(self.command_buffer.len() - self.cursor_offset as usize, char);
    }

    fn remove_char_before(&mut self) {
        if self.command_buffer.len() - self.cursor_offset as usize == 0 {
            return;
        }
        self.command_buffer.remove(self.command_buffer.len() - self.cursor_offset as usize - 1);
    }

    fn remove_char_at(&mut self) {
        if self.cursor_offset == 0 {
            return;
        }
        self.command_buffer.remove(self.command_buffer.len() - self.cursor_offset as usize);
        self.cursor_offset -= 1;
    }

    fn submit_command(&mut self) {
        self.cursor_offset = 0;
        self.scroll_offset = 0;
        let command = self.command_buffer.clone();
        self.line_buffer.push(format!("~ {}", command));
        self.command_buffer.clear();

        let tokens = Lexer::tokenize(&command);
        if tokens.is_err() {
            self.line_buffer.push(format!("{}", tokens.unwrap_err()));
            return;
        }
        let instruction = Parser::parse(tokens.unwrap());
        if instruction.is_err() {
            self.line_buffer.push(format!("{}", instruction.unwrap_err()));
            return;
        }
        self.line_buffer.push(format!("{:?}", instruction));
        
        
    }

    pub fn update(&mut self) -> io::Result<bool> {
        self.event = self.term.poll_event()?;

        if let Some(Event::Key(KeyEvent { code, .. })) = &self.event {
            match code {
                KeyCode::Esc => return Ok(true),
                KeyCode::Left => self.cursor_left(),
                KeyCode::Right => self.cursor_right(),
                KeyCode::Char(char) => self.char_at_cursor(char.clone()),
                KeyCode::Backspace => self.remove_char_before(),
                KeyCode::Delete => self.remove_char_at(),
                KeyCode::Enter => self.submit_command(),
                _ => ()
            }
        }else if let Some(Event::Mouse(MouseEvent { kind, .. })) = &self.event {
            match kind {
                MouseEventKind::ScrollUp => self.scroll_up(),
                MouseEventKind::ScrollDown => self.scroll_down(),
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
            .take(size()?.1 as usize - 2)
            .rev()
            .enumerate()
        {
            self.term.print_wrap(&line, 0, idx as u16, size()?.0, WrapMode::Normal)?;
        }

        /*
        if let Some(event) = &self.event {
            self.term.print(&format!("{:?}", event), 1, 1)?;
        }else {
            self.term.print("...", 1, 1)?;
        }
        */

        self.term.line('-', 0, size()?.1 - 2, size()?.0, false)?
            .print(&format!("~ {}", self.command_buffer), 0, size()?.1 - 1)?
            .move_cursor(2 + self.command_buffer.len() as u16 - self.cursor_offset, size()?.1 - 1)?
            .flush()?;
        Ok(())
    }

}