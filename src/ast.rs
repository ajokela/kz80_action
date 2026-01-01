// Abstract Syntax Tree types for Action! language

#[derive(Debug, Clone, PartialEq)]
pub enum DataType {
    Byte,           // 8-bit unsigned (0-255)
    Card,           // 16-bit unsigned (0-65535)
    Int,            // 16-bit signed (-32768 to 32767)
    Char,           // Character (same as Byte)
    ByteArray(usize),  // BYTE ARRAY with size
    CardArray(usize),  // CARD ARRAY with size
    IntArray(usize),   // INT ARRAY with size
    Pointer(Box<DataType>),  // Pointer to another type
}

impl DataType {
    pub fn size(&self) -> usize {
        match self {
            DataType::Byte | DataType::Char => 1,
            DataType::Card | DataType::Int => 2,
            DataType::ByteArray(n) => *n,
            DataType::CardArray(n) => n * 2,
            DataType::IntArray(n) => n * 2,
            DataType::Pointer(_) => 2,
        }
    }

    pub fn is_word(&self) -> bool {
        matches!(self, DataType::Card | DataType::Int | DataType::Pointer(_))
    }
}

#[derive(Debug, Clone)]
pub struct Variable {
    pub name: String,
    pub data_type: DataType,
    pub initial_value: Option<Expression>,
}

#[derive(Debug, Clone)]
pub struct Parameter {
    pub name: String,
    pub data_type: DataType,
}

#[derive(Debug, Clone)]
pub enum Expression {
    // Literals
    Number(i32),
    String(String),
    Char(char),

    // Variables
    Variable(String),
    ArrayAccess {
        array: String,
        index: Box<Expression>,
    },

    // Unary operations
    Negate(Box<Expression>),
    Not(Box<Expression>),
    AddressOf(String),           // @variable
    Dereference(Box<Expression>), // ^pointer

    // Binary operations
    Add(Box<Expression>, Box<Expression>),
    Subtract(Box<Expression>, Box<Expression>),
    Multiply(Box<Expression>, Box<Expression>),
    Divide(Box<Expression>, Box<Expression>),
    Modulo(Box<Expression>, Box<Expression>),
    LeftShift(Box<Expression>, Box<Expression>),
    RightShift(Box<Expression>, Box<Expression>),

    // Comparison operations
    Equal(Box<Expression>, Box<Expression>),
    NotEqual(Box<Expression>, Box<Expression>),
    Less(Box<Expression>, Box<Expression>),
    LessEqual(Box<Expression>, Box<Expression>),
    Greater(Box<Expression>, Box<Expression>),
    GreaterEqual(Box<Expression>, Box<Expression>),

    // Logical operations
    And(Box<Expression>, Box<Expression>),
    Or(Box<Expression>, Box<Expression>),
    Xor(Box<Expression>, Box<Expression>),

    // Bitwise operations
    BitAnd(Box<Expression>, Box<Expression>),
    BitOr(Box<Expression>, Box<Expression>),
    BitXor(Box<Expression>, Box<Expression>),

    // Function call
    FunctionCall {
        name: String,
        args: Vec<Expression>,
    },
}

#[derive(Debug, Clone)]
pub enum Statement {
    // Variable declaration
    VarDecl(Variable),

    // Assignment
    Assignment {
        target: String,
        value: Expression,
    },
    ArrayAssignment {
        array: String,
        index: Expression,
        value: Expression,
    },
    PointerAssignment {
        pointer: Expression,
        value: Expression,
    },

    // Control flow
    If {
        condition: Expression,
        then_block: Vec<Statement>,
        else_block: Option<Vec<Statement>>,
    },
    While {
        condition: Expression,
        body: Vec<Statement>,
    },
    For {
        var: String,
        start: Expression,
        end: Expression,
        step: Option<Expression>,
        body: Vec<Statement>,
    },
    Until {
        condition: Expression,
        body: Vec<Statement>,
    },

    // Flow control
    Exit,
    Return(Option<Expression>),

    // Procedure call
    ProcCall {
        name: String,
        args: Vec<Expression>,
    },

    // Block of statements
    Block(Vec<Statement>),
}

#[derive(Debug, Clone)]
pub struct Procedure {
    pub name: String,
    pub params: Vec<Parameter>,
    pub return_type: Option<DataType>,  // None for PROC, Some for FUNC
    pub locals: Vec<Variable>,
    pub body: Vec<Statement>,
}

#[derive(Debug, Clone)]
pub struct Program {
    pub globals: Vec<Variable>,
    pub procedures: Vec<Procedure>,
}

impl Program {
    pub fn new() -> Self {
        Program {
            globals: Vec::new(),
            procedures: Vec::new(),
        }
    }
}
