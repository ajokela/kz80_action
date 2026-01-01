// Lexer/Tokenizer for Action! language

use crate::token::{Token, TokenInfo};
use crate::error::{CompileError, Result};

pub struct Lexer<'a> {
    source: &'a str,
    chars: std::iter::Peekable<std::str::Chars<'a>>,
    line: usize,
    column: usize,
    current_char: Option<char>,
}

impl<'a> Lexer<'a> {
    pub fn new(source: &'a str) -> Self {
        let mut chars = source.chars().peekable();
        let current_char = chars.next();
        Lexer {
            source,
            chars,
            line: 1,
            column: 1,
            current_char,
        }
    }

    fn advance(&mut self) {
        if let Some(c) = self.current_char {
            if c == '\n' {
                self.line += 1;
                self.column = 1;
            } else {
                self.column += 1;
            }
        }
        self.current_char = self.chars.next();
    }

    fn peek(&mut self) -> Option<char> {
        self.chars.peek().copied()
    }

    fn skip_whitespace(&mut self) {
        while let Some(c) = self.current_char {
            if c == ' ' || c == '\t' || c == '\r' {
                self.advance();
            } else {
                break;
            }
        }
    }

    fn skip_comment(&mut self) {
        // Action! comments start with ; and go to end of line
        while let Some(c) = self.current_char {
            if c == '\n' {
                break;
            }
            self.advance();
        }
    }

    fn read_number(&mut self) -> Result<Token> {
        let start_col = self.column;
        let mut num_str = String::new();
        let mut is_hex = false;

        // Check for hex prefix $
        if self.current_char == Some('$') {
            is_hex = true;
            self.advance();
        }

        while let Some(c) = self.current_char {
            if is_hex {
                if c.is_ascii_hexdigit() {
                    num_str.push(c);
                    self.advance();
                } else {
                    break;
                }
            } else {
                if c.is_ascii_digit() {
                    num_str.push(c);
                    self.advance();
                } else {
                    break;
                }
            }
        }

        let value = if is_hex {
            i32::from_str_radix(&num_str, 16).map_err(|_| CompileError::LexerError {
                line: self.line,
                column: start_col,
                message: format!("Invalid hex number: ${}", num_str),
            })?
        } else {
            num_str.parse::<i32>().map_err(|_| CompileError::LexerError {
                line: self.line,
                column: start_col,
                message: format!("Invalid number: {}", num_str),
            })?
        };

        Ok(Token::Number(value))
    }

    fn read_string(&mut self) -> Result<Token> {
        let start_col = self.column;
        self.advance(); // Skip opening quote
        let mut s = String::new();

        while let Some(c) = self.current_char {
            if c == '"' {
                self.advance(); // Skip closing quote
                return Ok(Token::String(s));
            } else if c == '\n' {
                return Err(CompileError::LexerError {
                    line: self.line,
                    column: start_col,
                    message: "Unterminated string literal".to_string(),
                });
            } else {
                s.push(c);
                self.advance();
            }
        }

        Err(CompileError::LexerError {
            line: self.line,
            column: start_col,
            message: "Unterminated string literal".to_string(),
        })
    }

    fn read_char_literal(&mut self) -> Result<Token> {
        let start_col = self.column;
        self.advance(); // Skip opening quote

        let c = self.current_char.ok_or_else(|| CompileError::LexerError {
            line: self.line,
            column: start_col,
            message: "Empty character literal".to_string(),
        })?;

        self.advance();

        if self.current_char != Some('\'') {
            return Err(CompileError::LexerError {
                line: self.line,
                column: start_col,
                message: "Character literal must be single character".to_string(),
            });
        }
        self.advance(); // Skip closing quote

        Ok(Token::Char(c))
    }

    fn read_identifier(&mut self) -> Token {
        let mut ident = String::new();

        while let Some(c) = self.current_char {
            if c.is_ascii_alphanumeric() || c == '_' {
                ident.push(c);
                self.advance();
            } else {
                break;
            }
        }

        // Check for keywords (case-insensitive in Action!)
        match ident.to_uppercase().as_str() {
            "BYTE" => Token::Byte,
            "CARD" => Token::Card,
            "INT" => Token::Int,
            "CHAR" => Token::Char_,
            "ARRAY" => Token::Array,
            "IF" => Token::If,
            "THEN" => Token::Then,
            "ELSE" => Token::Else,
            "ELSEIF" => Token::ElseIf,
            "FI" => Token::Fi,
            "WHILE" => Token::While,
            "DO" => Token::Do,
            "OD" => Token::Od,
            "FOR" => Token::For,
            "TO" => Token::To,
            "STEP" => Token::Step,
            "UNTIL" => Token::Until,
            "EXIT" => Token::Exit,
            "RETURN" => Token::Return,
            "PROC" => Token::Proc,
            "FUNC" => Token::Func,
            "MODULE" => Token::Module,
            "MOD" => Token::Mod,
            "LSH" => Token::Lsh,
            "RSH" => Token::Rsh,
            "AND" => Token::And,
            "OR" => Token::Or,
            "XOR" => Token::Xor,
            "NOT" => Token::Not,
            _ => Token::Identifier(ident),
        }
    }

    fn next_token(&mut self) -> Result<Option<TokenInfo>> {
        self.skip_whitespace();

        let line = self.line;
        let column = self.column;

        let c = match self.current_char {
            Some(c) => c,
            None => return Ok(Some(TokenInfo::new(Token::Eof, line, column))),
        };

        let token = match c {
            // Comments
            ';' => {
                self.skip_comment();
                return self.next_token();
            }

            // Newlines (significant in Action!)
            '\n' => {
                self.advance();
                Token::Newline
            }

            // Numbers
            '$' => self.read_number()?,
            '0'..='9' => self.read_number()?,

            // Strings
            '"' => self.read_string()?,

            // Character literals
            '\'' => self.read_char_literal()?,

            // Identifiers and keywords
            'a'..='z' | 'A'..='Z' | '_' => self.read_identifier(),

            // Single-character operators
            '+' => { self.advance(); Token::Plus }
            '-' => { self.advance(); Token::Minus }
            '*' => { self.advance(); Token::Star }
            '/' => { self.advance(); Token::Slash }
            '(' => { self.advance(); Token::LeftParen }
            ')' => { self.advance(); Token::RightParen }
            '[' => { self.advance(); Token::LeftBracket }
            ']' => { self.advance(); Token::RightBracket }
            ',' => { self.advance(); Token::Comma }
            ':' => { self.advance(); Token::Colon }
            '@' => { self.advance(); Token::At }
            '^' => { self.advance(); Token::Caret }
            '&' => { self.advance(); Token::BitAnd }
            '%' => { self.advance(); Token::BitOr }
            '!' => { self.advance(); Token::BitXor }
            '#' => { self.advance(); Token::NotEqual }

            // Multi-character operators
            '=' => { self.advance(); Token::Equal }
            '<' => {
                self.advance();
                match self.current_char {
                    Some('>') => { self.advance(); Token::NotEqual }
                    Some('=') => { self.advance(); Token::LessEqual }
                    _ => Token::Less
                }
            }
            '>' => {
                self.advance();
                match self.current_char {
                    Some('=') => { self.advance(); Token::GreaterEqual }
                    _ => Token::Greater
                }
            }

            _ => {
                return Err(CompileError::LexerError {
                    line,
                    column,
                    message: format!("Unexpected character: '{}'", c),
                });
            }
        };

        Ok(Some(TokenInfo::new(token, line, column)))
    }

    pub fn tokenize(&mut self) -> Result<Vec<TokenInfo>> {
        let mut tokens = Vec::new();

        loop {
            match self.next_token()? {
                Some(token_info) => {
                    let is_eof = token_info.token == Token::Eof;
                    tokens.push(token_info);
                    if is_eof {
                        break;
                    }
                }
                None => break,
            }
        }

        Ok(tokens)
    }
}
