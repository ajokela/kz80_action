// Token types for Action! language

#[derive(Debug, Clone, PartialEq)]
pub enum Token {
    // Literals
    Number(i32),           // Decimal or hex number
    String(String),        // String literal
    Char(char),            // Character literal
    Identifier(String),    // Variable/procedure name

    // Type keywords
    Byte,                  // BYTE - 8-bit unsigned
    Card,                  // CARD - 16-bit unsigned (cardinal)
    Int,                   // INT - 16-bit signed
    Char_,                 // CHAR - character type
    Array,                 // ARRAY keyword

    // Control flow keywords
    If,                    // IF
    Then,                  // THEN
    Else,                  // ELSE
    ElseIf,                // ELSEIF
    Fi,                    // FI (end if)
    While,                 // WHILE
    Do,                    // DO
    Od,                    // OD (end do)
    For,                   // FOR
    To,                    // TO
    Step,                  // STEP
    Until,                 // UNTIL
    Exit,                  // EXIT (break)
    Return,                // RETURN

    // Procedure/function keywords
    Proc,                  // PROC
    Func,                  // FUNC
    Module,                // MODULE

    // Operators
    Plus,                  // +
    Minus,                 // -
    Star,                  // *
    Slash,                 // /
    Mod,                   // MOD
    Lsh,                   // LSH (left shift)
    Rsh,                   // RSH (right shift)

    // Comparison operators
    Equal,                 // =
    NotEqual,              // <> or #
    Less,                  // <
    LessEqual,             // <=
    Greater,               // >
    GreaterEqual,          // >=

    // Logical operators
    And,                   // AND
    Or,                    // OR
    Xor,                   // XOR
    Not,                   // NOT

    // Bitwise operators
    BitAnd,                // &
    BitOr,                 // %
    BitXor,                // !

    // Assignment
    Assign,                // = (context-dependent)

    // Punctuation
    LeftParen,             // (
    RightParen,            // )
    LeftBracket,           // [
    RightBracket,          // ]
    Comma,                 // ,
    Semicolon,             // ; (also comment start in some contexts)
    Colon,                 // :
    At,                    // @ (address-of)
    Caret,                 // ^ (pointer dereference)

    // Special
    Eof,                   // End of file
    Newline,               // End of line
}

#[derive(Debug, Clone)]
pub struct TokenInfo {
    pub token: Token,
    pub line: usize,
    pub column: usize,
}

impl TokenInfo {
    pub fn new(token: Token, line: usize, column: usize) -> Self {
        TokenInfo { token, line, column }
    }
}
