// Parser for Action! language

use crate::token::{Token, TokenInfo};
use crate::ast::*;
use crate::error::{CompileError, Result};

pub struct Parser {
    tokens: Vec<TokenInfo>,
    pos: usize,
}

impl Parser {
    pub fn new(tokens: Vec<TokenInfo>) -> Self {
        Parser { tokens, pos: 0 }
    }

    fn current(&self) -> &Token {
        if self.pos < self.tokens.len() {
            &self.tokens[self.pos].token
        } else {
            &Token::Eof
        }
    }

    fn current_line(&self) -> usize {
        if self.pos < self.tokens.len() {
            self.tokens[self.pos].line
        } else {
            0
        }
    }

    fn advance(&mut self) {
        if self.pos < self.tokens.len() {
            self.pos += 1;
        }
    }

    fn skip_newlines(&mut self) {
        while self.current() == &Token::Newline {
            self.advance();
        }
    }

    fn expect(&mut self, expected: Token) -> Result<()> {
        self.skip_newlines();
        if self.current() == &expected {
            self.advance();
            Ok(())
        } else {
            Err(CompileError::UnexpectedToken {
                expected: format!("{:?}", expected),
                found: format!("{:?}", self.current()),
            })
        }
    }

    fn expect_identifier(&mut self) -> Result<String> {
        self.skip_newlines();
        if let Token::Identifier(name) = self.current().clone() {
            self.advance();
            Ok(name)
        } else {
            Err(CompileError::UnexpectedToken {
                expected: "identifier".to_string(),
                found: format!("{:?}", self.current()),
            })
        }
    }

    // Parse data type
    fn parse_type(&mut self) -> Result<DataType> {
        self.skip_newlines();
        let base_type = match self.current() {
            Token::Byte => { self.advance(); DataType::Byte }
            Token::Card => { self.advance(); DataType::Card }
            Token::Int => { self.advance(); DataType::Int }
            Token::Char_ => { self.advance(); DataType::Char }
            _ => {
                return Err(CompileError::ParserError {
                    line: self.current_line(),
                    message: format!("Expected type, found {:?}", self.current()),
                });
            }
        };

        // Check for ARRAY
        self.skip_newlines();
        if self.current() == &Token::Array {
            self.advance();
            self.skip_newlines();

            // Optional array size in parentheses
            let size = if self.current() == &Token::LeftParen {
                self.advance();
                let size = self.parse_number()?;
                self.expect(Token::RightParen)?;
                size as usize
            } else {
                256 // Default array size
            };

            Ok(match base_type {
                DataType::Byte | DataType::Char => DataType::ByteArray(size),
                DataType::Card => DataType::CardArray(size),
                DataType::Int => DataType::IntArray(size),
                _ => base_type,
            })
        } else {
            Ok(base_type)
        }
    }

    fn parse_number(&mut self) -> Result<i32> {
        self.skip_newlines();
        if let Token::Number(n) = self.current() {
            let n = *n;
            self.advance();
            Ok(n)
        } else {
            Err(CompileError::UnexpectedToken {
                expected: "number".to_string(),
                found: format!("{:?}", self.current()),
            })
        }
    }

    // Parse primary expression (atoms)
    fn parse_primary(&mut self) -> Result<Expression> {
        self.skip_newlines();
        match self.current().clone() {
            Token::Number(n) => {
                self.advance();
                Ok(Expression::Number(n))
            }
            Token::String(s) => {
                self.advance();
                Ok(Expression::String(s))
            }
            Token::Char(c) => {
                self.advance();
                Ok(Expression::Char(c))
            }
            Token::Identifier(name) => {
                self.advance();
                self.skip_newlines();

                // Check for array access or function call
                match self.current() {
                    Token::LeftBracket => {
                        self.advance();
                        let index = self.parse_expression()?;
                        self.expect(Token::RightBracket)?;
                        Ok(Expression::ArrayAccess {
                            array: name,
                            index: Box::new(index),
                        })
                    }
                    Token::LeftParen => {
                        self.advance();
                        let args = self.parse_argument_list()?;
                        self.expect(Token::RightParen)?;
                        Ok(Expression::FunctionCall { name, args })
                    }
                    _ => Ok(Expression::Variable(name)),
                }
            }
            Token::LeftParen => {
                self.advance();
                let expr = self.parse_expression()?;
                self.expect(Token::RightParen)?;
                Ok(expr)
            }
            Token::At => {
                self.advance();
                let name = self.expect_identifier()?;
                Ok(Expression::AddressOf(name))
            }
            Token::Caret => {
                self.advance();
                let expr = self.parse_primary()?;
                Ok(Expression::Dereference(Box::new(expr)))
            }
            Token::Minus => {
                self.advance();
                let expr = self.parse_unary()?;
                Ok(Expression::Negate(Box::new(expr)))
            }
            Token::Not => {
                self.advance();
                let expr = self.parse_unary()?;
                Ok(Expression::Not(Box::new(expr)))
            }
            _ => Err(CompileError::ParserError {
                line: self.current_line(),
                message: format!("Unexpected token in expression: {:?}", self.current()),
            }),
        }
    }

    fn parse_unary(&mut self) -> Result<Expression> {
        self.skip_newlines();
        match self.current() {
            Token::Minus => {
                self.advance();
                let expr = self.parse_unary()?;
                Ok(Expression::Negate(Box::new(expr)))
            }
            Token::Not => {
                self.advance();
                let expr = self.parse_unary()?;
                Ok(Expression::Not(Box::new(expr)))
            }
            _ => self.parse_primary(),
        }
    }

    // Parse multiplication/division
    fn parse_multiplicative(&mut self) -> Result<Expression> {
        let mut left = self.parse_unary()?;

        loop {
            self.skip_newlines();
            match self.current() {
                Token::Star => {
                    self.advance();
                    let right = self.parse_unary()?;
                    left = Expression::Multiply(Box::new(left), Box::new(right));
                }
                Token::Slash => {
                    self.advance();
                    let right = self.parse_unary()?;
                    left = Expression::Divide(Box::new(left), Box::new(right));
                }
                Token::Mod => {
                    self.advance();
                    let right = self.parse_unary()?;
                    left = Expression::Modulo(Box::new(left), Box::new(right));
                }
                _ => break,
            }
        }

        Ok(left)
    }

    // Parse addition/subtraction
    fn parse_additive(&mut self) -> Result<Expression> {
        let mut left = self.parse_multiplicative()?;

        loop {
            self.skip_newlines();
            match self.current() {
                Token::Plus => {
                    self.advance();
                    let right = self.parse_multiplicative()?;
                    left = Expression::Add(Box::new(left), Box::new(right));
                }
                Token::Minus => {
                    self.advance();
                    let right = self.parse_multiplicative()?;
                    left = Expression::Subtract(Box::new(left), Box::new(right));
                }
                _ => break,
            }
        }

        Ok(left)
    }

    // Parse shift operations
    fn parse_shift(&mut self) -> Result<Expression> {
        let mut left = self.parse_additive()?;

        loop {
            self.skip_newlines();
            match self.current() {
                Token::Lsh => {
                    self.advance();
                    let right = self.parse_additive()?;
                    left = Expression::LeftShift(Box::new(left), Box::new(right));
                }
                Token::Rsh => {
                    self.advance();
                    let right = self.parse_additive()?;
                    left = Expression::RightShift(Box::new(left), Box::new(right));
                }
                _ => break,
            }
        }

        Ok(left)
    }

    // Parse comparison operations
    fn parse_comparison(&mut self) -> Result<Expression> {
        let mut left = self.parse_shift()?;

        loop {
            self.skip_newlines();
            match self.current() {
                Token::Equal => {
                    self.advance();
                    let right = self.parse_shift()?;
                    left = Expression::Equal(Box::new(left), Box::new(right));
                }
                Token::NotEqual => {
                    self.advance();
                    let right = self.parse_shift()?;
                    left = Expression::NotEqual(Box::new(left), Box::new(right));
                }
                Token::Less => {
                    self.advance();
                    let right = self.parse_shift()?;
                    left = Expression::Less(Box::new(left), Box::new(right));
                }
                Token::LessEqual => {
                    self.advance();
                    let right = self.parse_shift()?;
                    left = Expression::LessEqual(Box::new(left), Box::new(right));
                }
                Token::Greater => {
                    self.advance();
                    let right = self.parse_shift()?;
                    left = Expression::Greater(Box::new(left), Box::new(right));
                }
                Token::GreaterEqual => {
                    self.advance();
                    let right = self.parse_shift()?;
                    left = Expression::GreaterEqual(Box::new(left), Box::new(right));
                }
                _ => break,
            }
        }

        Ok(left)
    }

    // Parse logical AND
    fn parse_and(&mut self) -> Result<Expression> {
        let mut left = self.parse_comparison()?;

        loop {
            self.skip_newlines();
            if self.current() == &Token::And {
                self.advance();
                let right = self.parse_comparison()?;
                left = Expression::And(Box::new(left), Box::new(right));
            } else {
                break;
            }
        }

        Ok(left)
    }

    // Parse logical OR/XOR
    fn parse_or(&mut self) -> Result<Expression> {
        let mut left = self.parse_and()?;

        loop {
            self.skip_newlines();
            match self.current() {
                Token::Or => {
                    self.advance();
                    let right = self.parse_and()?;
                    left = Expression::Or(Box::new(left), Box::new(right));
                }
                Token::Xor => {
                    self.advance();
                    let right = self.parse_and()?;
                    left = Expression::Xor(Box::new(left), Box::new(right));
                }
                _ => break,
            }
        }

        Ok(left)
    }

    fn parse_expression(&mut self) -> Result<Expression> {
        self.parse_or()
    }

    fn parse_argument_list(&mut self) -> Result<Vec<Expression>> {
        let mut args = Vec::new();
        self.skip_newlines();

        if self.current() == &Token::RightParen {
            return Ok(args);
        }

        args.push(self.parse_expression()?);

        while self.current() == &Token::Comma {
            self.advance();
            args.push(self.parse_expression()?);
        }

        Ok(args)
    }

    // Parse variable declaration
    fn parse_var_decl(&mut self) -> Result<Variable> {
        let data_type = self.parse_type()?;
        let name = self.expect_identifier()?;

        let initial_value = if self.current() == &Token::Equal {
            self.advance();
            Some(self.parse_expression()?)
        } else {
            None
        };

        Ok(Variable {
            name,
            data_type,
            initial_value,
        })
    }

    // Parse statement
    fn parse_statement(&mut self) -> Result<Option<Statement>> {
        self.skip_newlines();

        match self.current().clone() {
            Token::Eof | Token::Od | Token::Fi | Token::Until => {
                Ok(None)
            }

            // Variable declaration
            Token::Byte | Token::Card | Token::Int | Token::Char_ => {
                let var = self.parse_var_decl()?;
                Ok(Some(Statement::VarDecl(var)))
            }

            // IF statement
            Token::If => {
                self.advance();
                let condition = self.parse_expression()?;
                self.skip_newlines();

                // THEN is optional in some Action! variants
                if self.current() == &Token::Then {
                    self.advance();
                }

                let then_block = self.parse_block()?;

                let else_block = if self.current() == &Token::Else {
                    self.advance();
                    Some(self.parse_block()?)
                } else {
                    None
                };

                self.expect(Token::Fi)?;
                Ok(Some(Statement::If {
                    condition,
                    then_block,
                    else_block,
                }))
            }

            // WHILE statement
            Token::While => {
                self.advance();
                let condition = self.parse_expression()?;
                self.expect(Token::Do)?;
                let body = self.parse_block()?;
                self.expect(Token::Od)?;
                Ok(Some(Statement::While { condition, body }))
            }

            // FOR statement
            Token::For => {
                self.advance();
                let var = self.expect_identifier()?;
                self.expect(Token::Equal)?;
                let start = self.parse_expression()?;
                self.expect(Token::To)?;
                let end = self.parse_expression()?;

                let step = if self.current() == &Token::Step {
                    self.advance();
                    Some(self.parse_expression()?)
                } else {
                    None
                };

                self.expect(Token::Do)?;
                let body = self.parse_block()?;
                self.expect(Token::Od)?;

                Ok(Some(Statement::For {
                    var,
                    start,
                    end,
                    step,
                    body,
                }))
            }

            // EXIT
            Token::Exit => {
                self.advance();
                Ok(Some(Statement::Exit))
            }

            // RETURN
            Token::Return => {
                self.advance();
                self.skip_newlines();

                // Check if there's a return value
                let value = match self.current() {
                    Token::Newline | Token::Eof | Token::Od | Token::Fi => None,
                    _ => Some(self.parse_expression()?),
                };

                Ok(Some(Statement::Return(value)))
            }

            // Assignment or procedure call
            Token::Identifier(name) => {
                self.advance();
                self.skip_newlines();

                match self.current() {
                    // Array assignment
                    Token::LeftBracket => {
                        self.advance();
                        let index = self.parse_expression()?;
                        self.expect(Token::RightBracket)?;
                        self.expect(Token::Equal)?;
                        let value = self.parse_expression()?;
                        Ok(Some(Statement::ArrayAssignment {
                            array: name,
                            index,
                            value,
                        }))
                    }
                    // Assignment
                    Token::Equal => {
                        self.advance();
                        let value = self.parse_expression()?;
                        Ok(Some(Statement::Assignment { target: name, value }))
                    }
                    // Procedure call
                    Token::LeftParen => {
                        self.advance();
                        let args = self.parse_argument_list()?;
                        self.expect(Token::RightParen)?;
                        Ok(Some(Statement::ProcCall { name, args }))
                    }
                    // Bare procedure call (no parens)
                    _ => {
                        Ok(Some(Statement::ProcCall { name, args: vec![] }))
                    }
                }
            }

            // Pointer dereference assignment
            Token::Caret => {
                self.advance();
                let pointer = self.parse_primary()?;
                self.expect(Token::Equal)?;
                let value = self.parse_expression()?;
                Ok(Some(Statement::PointerAssignment { pointer, value }))
            }

            Token::Newline => {
                self.advance();
                self.parse_statement()
            }

            _ => Err(CompileError::ParserError {
                line: self.current_line(),
                message: format!("Unexpected token: {:?}", self.current()),
            }),
        }
    }

    fn parse_block(&mut self) -> Result<Vec<Statement>> {
        let mut statements = Vec::new();
        self.skip_newlines();

        loop {
            match self.current() {
                Token::Od | Token::Fi | Token::Else | Token::ElseIf | Token::Until | Token::Eof | Token::Return => {
                    break;
                }
                _ => {
                    if let Some(stmt) = self.parse_statement()? {
                        statements.push(stmt);
                    } else {
                        break;
                    }
                }
            }
            self.skip_newlines();
        }

        Ok(statements)
    }

    // Parse procedure/function
    fn parse_procedure(&mut self) -> Result<Procedure> {
        let is_func = self.current() == &Token::Func;
        self.advance();

        let return_type = if is_func {
            Some(self.parse_type()?)
        } else {
            None
        };

        let name = self.expect_identifier()?;

        // Parse parameters
        let params = if self.current() == &Token::LeftParen {
            self.advance();
            let params = self.parse_parameter_list()?;
            self.expect(Token::RightParen)?;
            params
        } else {
            Vec::new()
        };

        self.skip_newlines();

        // Parse locals and body
        let mut locals = Vec::new();
        let mut body = Vec::new();

        // Parse local variable declarations first
        loop {
            self.skip_newlines();
            match self.current() {
                Token::Byte | Token::Card | Token::Int | Token::Char_ => {
                    let var = self.parse_var_decl()?;
                    locals.push(var);
                }
                _ => break,
            }
        }

        // Parse body until RETURN
        body = self.parse_block()?;

        // Handle RETURN at end
        self.skip_newlines();
        if self.current() == &Token::Return {
            if let Some(stmt) = self.parse_statement()? {
                body.push(stmt);
            }
        }

        Ok(Procedure {
            name,
            params,
            return_type,
            locals,
            body,
        })
    }

    fn parse_parameter_list(&mut self) -> Result<Vec<Parameter>> {
        let mut params = Vec::new();
        self.skip_newlines();

        if self.current() == &Token::RightParen {
            return Ok(params);
        }

        loop {
            let data_type = self.parse_type()?;
            let name = self.expect_identifier()?;
            params.push(Parameter { name, data_type });

            self.skip_newlines();
            if self.current() == &Token::Comma {
                self.advance();
            } else {
                break;
            }
        }

        Ok(params)
    }

    pub fn parse(&mut self) -> Result<Program> {
        let mut program = Program::new();

        loop {
            self.skip_newlines();

            match self.current() {
                Token::Eof => break,

                // Global variable
                Token::Byte | Token::Card | Token::Int | Token::Char_ => {
                    let var = self.parse_var_decl()?;
                    program.globals.push(var);
                }

                // Procedure or function
                Token::Proc | Token::Func => {
                    let proc = self.parse_procedure()?;
                    program.procedures.push(proc);
                }

                Token::Module => {
                    self.advance();
                    // Skip module declaration for now
                }

                _ => {
                    return Err(CompileError::ParserError {
                        line: self.current_line(),
                        message: format!("Unexpected token at top level: {:?}", self.current()),
                    });
                }
            }
        }

        Ok(program)
    }
}
