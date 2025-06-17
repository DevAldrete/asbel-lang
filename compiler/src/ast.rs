// Using String for Identifiers and TypeIdentifiers for simplicity in MVP
pub type Identifier = String;
pub type TypeIdentifier = String;

#[derive(Debug, PartialEq, Clone)]
pub enum Node {
    Statement(Statement),
    Expression(Expression),
    Program(Program),
}

#[derive(Debug, PartialEq, Clone)]
pub struct Program {
    pub body: Vec<Statement>,
}

#[derive(Debug, PartialEq, Clone)]
pub enum Statement {
    Let {
        name: Identifier,
        value: Expression,
        // type_annotation: Option<TypeIdentifier>, // Optional for MVP
    },
    FunctionDeclaration {
        name: Identifier,
        params: Vec<Parameter>,
        return_type: Option<TypeIdentifier>,
        body: BlockStatement,
    },
    ExpressionStatement {
        expression: Expression,
    },
    // ReturnStatement { // Not strictly needed for "Hello World" if last expr is implicit return
    //     value: Option<Expression>,
    // },
}

#[derive(Debug, PartialEq, Clone)]
pub enum Expression {
    Identifier(Identifier),
    LiteralInteger(i64),
    LiteralString(String),
    FunctionCall {
        function: Box<Expression>, // Usually an Identifier
        arguments: Vec<Expression>,
    },
    InfixExpression {
        left: Box<Expression>,
        operator: InfixOperator,
        right: Box<Expression>,
    },
    // PrefixExpression { // For things like -5 or !true
    //     operator: PrefixOperator,
    //     right: Box<Expression>,
    // },
    GroupedExpression(Box<Expression>),
}

#[derive(Debug, PartialEq, Clone)]
pub enum InfixOperator {
    Plus,  // +
    Minus, // -
    Star,  // *
    Slash, // /
    Eq,    // = (Used for assignment in Let, not comparison)
}

// pub enum PrefixOperator {
//     Minus, // -
//     Not,   // ! (if we add booleans)
// }

#[derive(Debug, PartialEq, Clone)]
pub struct BlockStatement {
    pub statements: Vec<Statement>,
}

#[derive(Debug, PartialEq, Clone)]
pub struct Parameter {
    pub name: Identifier,
    pub type_ann: Option<TypeIdentifier>,
}
