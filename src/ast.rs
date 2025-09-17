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
    Block {
        body: Vec<Statement>,
    },
    Breakloop,
    Say(Vec<Expression>),
    Ask(Vec<Expression>),
    Open(Expression),
    Get(Vec<Expression>),
    Run(Vec<Expression>),
    Wait(Vec<Expression>),
    ActionDefinition {
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
    Number(f64),
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
