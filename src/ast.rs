#[derive(Debug, Clone, PartialEq)]
pub enum Statement {
    Assignment {
        variable: String,
        value: Expression,
    },
    PropertyAssignment {
        object: Box<Expression>,
        property: String,
        value: Expression,
    },
    If {
        condition: Expression,
        then_stmt: Box<Statement>,
        else_stmt: Option<Box<Statement>>,
    },
    Loop {
        body: Vec<Statement>,
    },
    ForEach {
        variables: Vec<String>,
        iterable: Expression,
        body: Vec<Statement>,
    },
    Increment {
        variable: String,
        amount: Expression,
    },
    Decrement {
        variable: String,
        amount: Expression,
    },
    Block {
        body: Vec<Statement>,
    },
    Breakloop,
    Open(Expression),
    FunctionDefinition {
        name: String,
        params: Vec<String>,
        body: Vec<Statement>,
    },
    Give(Expression),
    Expression(Expression),
}

#[derive(Debug, Clone, PartialEq)]
pub enum InterpolationPart {
    Text(String),
    Variable(String),
    Expression(Expression),
}

#[derive(Debug, Clone, PartialEq)]
pub enum Expression {
    Number(f64, bool), // value, was_float_literal
    String(String),
    InterpolatedString(Vec<InterpolationPart>),
    Boolean(bool),
    Variable(String),
    List(Vec<Expression>),

    BinaryOp {
        left: Box<Expression>,
        op: BinaryOperator,
        right: Box<Expression>,
    },

    FunctionCall {
        name: String,
        args: Vec<Expression>,
    },

    PropertyAccess {
        object: Box<Expression>,
        property: String,
    },

    ObjectLiteral {
        pairs: Vec<(String, Expression)>,
    },
}

#[derive(Debug, Clone, PartialEq)]
pub enum BinaryOperator {
    Add,
    Subtract,
    Multiply,
    Divide,
    IntegerDivide,
    Modulo,
    LessThan,
    LessThanOrEqual,
    GreaterThan,
    GreaterThanOrEqual,
    Equal,
    NotEqual,
    And,
    Or,
}

pub type Program = Vec<Statement>;
