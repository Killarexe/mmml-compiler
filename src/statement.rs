use crate::expression::Expression;


#[derive(Debug, Clone, Eq, PartialEq)]
pub enum Statement {
    Program {
        body: Vec<Statement>
    },
    Loop {
        times: u8,
        statements: Vec<Statement>
    },
    ExpressionStatement {
        expression: Expression
    }
}
