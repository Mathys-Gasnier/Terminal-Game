use std::fmt::Display;

use crate::parser::Instruction;

#[derive(Debug, Clone)]
pub enum HandleError {
    WrongArgType(String, u16),
    NotFound(String),
}

impl Display for HandleError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::WrongArgType(expected, at) => {
                write!(f, "Expected argument of type '{}' at {}", expected, at)
            }
            Self::NotFound(str) => write!(f, "{}", str),
        }
    }
}

#[derive(Debug, Clone)]
pub enum Value {
    Null,
    IntValue(i64),
    BoolValue(bool),
}

impl Display for Value {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Null => write!(f, "Null"),
            Self::IntValue(int) => write!(f, "{}", int),
            Self::BoolValue(bool) => write!(f, "{}", bool),
        }
    }
}

pub trait GameObject {
    fn handle(&mut self, instruction: Instruction) -> Result<Value, HandleError>;
    fn return_err(
        &self,
        class_name: String,
        instruction: Instruction,
    ) -> Result<Value, HandleError> {
        match instruction {
            Instruction::FunctionCall(name, _) => Err(HandleError::NotFound(format!(
                "Did not find any functions named '{}' on {}",
                name, class_name
            ))),
            Instruction::Access(key, _) => Err(HandleError::NotFound(format!(
                "Did not find property '{}' on {}",
                key, class_name
            ))),
        }
    }
}
