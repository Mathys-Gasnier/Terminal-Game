use std::fmt::Display;

#[derive(Debug, Clone)]
pub enum Token {
    Keyword(String),
    Int(i64),
    OpenParen,
    CloseParen,
    Dot,
    Coma,
}

impl Token {
    pub fn text(&self) -> String {
        match self {
            Token::Keyword(keyword) => keyword.clone(),
            Token::Int(int) => int.to_string(),
            Token::OpenParen => "(".to_string(),
            Token::CloseParen => ")".to_string(),
            Token::Dot => ".".to_string(),
            Token::Coma => ",".to_string(),
        }
    }
}

#[derive(Debug)]
pub enum LexerError {
    Unknown(u16, char),
    NumberParseError(u16, String),
}

impl Display for LexerError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            LexerError::Unknown(idx, char) => write!(f, "Unexpected char '{}' at {}", char, idx),
            LexerError::NumberParseError(idx, number) => {
                write!(f, "Number '{}' failed to parse at {}", number, idx)
            }
        }
    }
}

pub struct Lexer {
    source: String,
    pointer: u16,
}

impl Lexer {
    pub fn tokenize(source: &str) -> Result<Vec<Token>, LexerError> {
        let mut lexer = Lexer {
            source: source.to_string(),
            pointer: 0,
        };

        lexer.tokens()
    }

    fn peek(&self) -> Option<char> {
        self.source.chars().nth(self.pointer as usize)
    }

    fn consume(&mut self) -> Option<char> {
        let char = self.source.chars().nth(self.pointer as usize);
        self.pointer += 1;
        char
    }

    fn tokens(&mut self) -> Result<Vec<Token>, LexerError> {
        let mut tokens = vec![];

        while let Some(char) = self.consume() {
            if char.is_whitespace() {
                continue;
            } else if char.is_alphabetic() {
                let mut buffer = String::from(char);
                while let Some(char) = self.peek() {
                    if !char.is_alphanumeric() && char != '_' {
                        break;
                    }
                    self.pointer += 1;
                    buffer.push(char);
                }
                tokens.push(Token::Keyword(buffer));
            } else if char.is_ascii_digit() || char == '-' {
                let number_start = self.pointer - 1;
                let mut buffer = String::from(char);
                while let Some(char) = self.peek() {
                    if !char.is_numeric() && char != '_' {
                        break;
                    }
                    self.pointer += 1;
                    buffer.push(char);
                }
                let int = buffer
                    .parse::<i64>()
                    .map_err(|_| LexerError::NumberParseError(number_start, buffer))?;
                tokens.push(Token::Int(int));
            } else if char == '(' {
                tokens.push(Token::OpenParen);
            } else if char == ')' {
                tokens.push(Token::CloseParen);
            } else if char == '.' {
                tokens.push(Token::Dot);
            } else if char == ',' {
                tokens.push(Token::Coma);
            } else {
                return Err(LexerError::Unknown(self.pointer - 1, char));
            }
        }

        Ok(tokens)
    }
}
