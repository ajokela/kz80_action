// Error types for the Action! compiler

use thiserror::Error;

#[derive(Error, Debug)]
pub enum CompileError {
    #[error("Lexer error at line {line}, column {column}: {message}")]
    LexerError {
        line: usize,
        column: usize,
        message: String,
    },

    #[error("Parser error at line {line}: {message}")]
    ParserError {
        line: usize,
        message: String,
    },

    #[error("Unexpected token: expected {expected}, found {found}")]
    UnexpectedToken {
        expected: String,
        found: String,
    },

    #[error("Undefined variable: {name}")]
    UndefinedVariable {
        name: String,
    },

    #[error("Undefined procedure: {name}")]
    UndefinedProcedure {
        name: String,
    },

    #[error("Type mismatch: expected {expected}, found {found}")]
    TypeMismatch {
        expected: String,
        found: String,
    },

    #[error("Code generation error: {message}")]
    CodeGenError {
        message: String,
    },

    #[error("Internal compiler error: {message}")]
    InternalError {
        message: String,
    },
}

pub type Result<T> = std::result::Result<T, CompileError>;
