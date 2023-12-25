use crate::{
    game_object::{GameObject, HandleError, Value},
    lexer::Lexer,
    parser::{Arg, Instruction, Parser},
    term::{Term, WrapMode},
};
use crossterm::{
    event::{Event, KeyCode, KeyEvent, MouseEvent, MouseEventKind},
    terminal::size,
};
use serde::{Deserialize, Serialize};
use std::{cmp, fs, io};

#[derive(Clone, Serialize, Deserialize)]
pub struct Root {
    coins: i64,
}

impl Root {
    fn new() -> Self {
        let save = match fs::read_to_string("game.save") {
            Ok(ok) => ok,
            Err(err) => {
                println!("Failed to load game save {}, exiting", err);
                panic!();
            }
        };
        let root = match serde_json::from_str::<Self>(&save) {
            Ok(ok) => ok,
            Err(err) => {
                println!("Failed to load game save {}, exiting", err);
                panic!();
            }
        };
        root
    }
}

impl GameObject for Root {
    fn handle(&mut self, instruction: Instruction) -> Result<Value, HandleError> {
        match instruction {
            Instruction::Access(key, _) if key == "coins".to_string() => {
                Ok(Value::IntValue(self.coins))
            }
            Instruction::FunctionCall(name, args) if name == "add".to_string() => {
                let Some(Arg::Int(amount)) = args.get(0) else {
                    return Err(HandleError::WrongArgType("Int".to_string(), 0));
                };
                self.coins += amount;
                Ok(Value::IntValue(self.coins))
            }
            _ => self.return_err("Root".to_string(), instruction),
        }
    }
}

impl Drop for Root {
    fn drop(&mut self) {
        let json = match serde_json::to_string(self) {
            Ok(ok) => ok,
            Err(err) => {
                println!("Failed to save game root: {}", err);
                return;
            }
        };
        if let Err(err) = fs::write("game.save", json) {
            println!("Failed to save game root to file: {}", err);
        }
    }
}

#[derive(Clone)]
pub struct Game {
    term: Term,
    command_buffer: String,
    line_buffer: Vec<String>,
    cursor_offset: u16,
    scroll_offset: u16,
    event: Option<Event>,
    root: Root,
}

impl Game {
    pub fn new() -> io::Result<Self> {
        Ok(Self {
            term: Term::new()?,
            command_buffer: String::new(),
            line_buffer: vec![],
            cursor_offset: 0,
            scroll_offset: 0,
            event: None,
            root: Root::new(),
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
        } else {
            self.scroll_offset =
                cmp::min(self.scroll_offset, self.line_buffer.len() as u16 - height);
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
        self.command_buffer.insert(
            self.command_buffer.len() - self.cursor_offset as usize,
            char,
        );
    }

    fn remove_char_before(&mut self) {
        if self.command_buffer.len() - self.cursor_offset as usize == 0 {
            return;
        }
        self.command_buffer
            .remove(self.command_buffer.len() - self.cursor_offset as usize - 1);
    }

    fn remove_char_at(&mut self) {
        if self.cursor_offset == 0 {
            return;
        }
        self.command_buffer
            .remove(self.command_buffer.len() - self.cursor_offset as usize);
        self.cursor_offset -= 1;
    }

    fn submit_command(&mut self) {
        self.cursor_offset = 0;
        self.scroll_offset = 0;
        let command = self.command_buffer.clone();
        self.line_buffer.push(format!("~ {}", command));
        self.command_buffer.clear();

        let tokens = match Lexer::tokenize(&command) {
            Ok(ok) => ok,
            Err(err) => {
                self.line_buffer.push(format!("{}", err));
                return;
            }
        };

        let instruction = match Parser::parse(tokens) {
            Ok(ok) => ok,
            Err(err) => {
                self.line_buffer.push(format!("{}", err));
                return;
            }
        };

        let result = match self.root.handle(instruction) {
            Ok(ok) => ok,
            Err(err) => {
                self.line_buffer.push(format!("{}", err));
                return;
            }
        };

        self.line_buffer.push(format!("{}", result));
    }

    pub fn update(&mut self) -> io::Result<bool> {
        self.event = self.term.poll_event()?;

        match self.event {
            Some(Event::Key(KeyEvent { code, .. })) => match code {
                KeyCode::Esc => return Ok(true),
                KeyCode::Left => self.cursor_left(),
                KeyCode::Right => self.cursor_right(),
                KeyCode::Char(char) => self.char_at_cursor(char.clone()),
                KeyCode::Backspace => self.remove_char_before(),
                KeyCode::Delete => self.remove_char_at(),
                KeyCode::Enter => self.submit_command(),
                _ => (),
            },
            Some(Event::Mouse(MouseEvent { kind, .. })) => match kind {
                MouseEventKind::ScrollUp => self.scroll_up(),
                MouseEventKind::ScrollDown => self.scroll_down(),
                _ => (),
            },
            _ => (),
        }

        Ok(false)
    }

    pub fn fixed_update(&mut self) -> io::Result<()> {
        self.line_buffer.push("X seconds".to_string());
        Ok(())
    }

    pub fn draw(&mut self) -> io::Result<()> {
        self.term.clear()?;

        for (idx, line) in self
            .line_buffer
            .iter()
            .rev()
            .skip(self.scroll_offset as usize)
            .take(size()?.1 as usize - 2)
            .rev()
            .enumerate()
        {
            self.term
                .print_wrap(&line, 0, idx as u16, size()?.0, WrapMode::Normal)?;
        }

        /*
        if let Some(event) = &self.event {
            self.term.print(&format!("{:?}", event), 1, 1)?;
        }else {
            self.term.print("...", 1, 1)?;
        }
        */

        self.term
            .line('-', 0, size()?.1 - 2, size()?.0, false)?
            .print(&format!("~ {}", self.command_buffer), 0, size()?.1 - 1)?
            .move_cursor(
                2 + self.command_buffer.len() as u16 - self.cursor_offset,
                size()?.1 - 1,
            )?
            .flush()?;
        Ok(())
    }
}
