use crate::token::Token;

#[derive(Clone, PartialEq, Eq, Debug)]
pub enum Expression {
    ShortCommand {
        command: Token
    },
    Command {
        command: Token,
        argument: u8
    }
}
