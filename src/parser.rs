use std::fmt::Display;

use crate::lexer::Token;

#[derive(Debug)]
pub enum Arg {
    Int(i64),
}

#[derive(Debug)]
pub enum Instruction {
    Access(String, Option<Box<Instruction>>),
    FunctionCall(String, Vec<Arg>),
}

#[derive(Debug)]
pub enum ParserError {
    UnexpectedToken(Option<Token>),
    ExpectedToken,
    Expected(Token, Option<Token>),
}

impl Display for ParserError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::ExpectedToken => write!(f, "Expected a token but found none"),
            Self::UnexpectedToken(token) => write!(
                f,
                "Unexpected token {}",
                token
                    .clone()
                    .map_or("none".to_string(), |token| format!("'{}'", token.text()))
            ),
            Self::Expected(expected, got) => write!(
                f,
                "Expected '{}' but got {}",
                expected.text(),
                got.clone()
                    .map_or("none".to_string(), |token| format!("'{}'", token.text()))
            ),
        }
    }
}

pub struct Parser {
    tokens: Vec<Token>,
    pointer: u16,
}

impl Parser {
    pub fn parse(tokens: Vec<Token>) -> Result<Instruction, ParserError> {
        let mut parser = Self { tokens, pointer: 0 };

        parser.parse_instruction()
    }

    fn peek(&self) -> Option<Token> {
        self.tokens.get(self.pointer as usize).cloned()
    }

    fn consume(&mut self) -> Option<Token> {
        let token = self.tokens.get(self.pointer as usize);
        self.pointer += 1;
        token.cloned()
    }

    fn parse_instruction(&mut self) -> Result<Instruction, ParserError> {
        let token = self.consume().ok_or(ParserError::ExpectedToken)?;
        match token {
            Token::Keyword(keyword) => match self.peek() {
                Some(Token::OpenParen) => {
                    self.pointer += 1;
                    let mut args: Vec<Arg> = vec![];
                    while let Some(token) = self.peek() {
                        if let Token::CloseParen = token {
                            break;
                        }
                        self.pointer += 1;
                        let arg = match token {
                            Token::Int(int) => Ok(Arg::Int(int)),
                            _ => Err(ParserError::UnexpectedToken(Some(token))),
                        }?;
                        args.push(arg);
                        let Some(Token::Coma) = self.peek() else {
                            break;
                        };
                        self.pointer += 1;
                    }
                    let Some(Token::CloseParen) = self.peek() else {
                        return Err(ParserError::Expected(Token::CloseParen, self.peek()));
                    };
                    self.pointer += 1;
                    Ok(Instruction::FunctionCall(keyword, args))
                }
                Some(Token::Dot) => {
                    self.pointer += 1;
                    Ok(Instruction::Access(
                        keyword,
                        Some(Box::new(self.parse_instruction()?)),
                    ))
                }
                None => Ok(Instruction::Access(keyword, None)),
                _ => Err(ParserError::UnexpectedToken(self.peek())),
            },
            _ => Err(ParserError::UnexpectedToken(Some(token))),
        }
    }
}
