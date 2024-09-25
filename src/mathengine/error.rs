use std::fmt;
use super::token::Token;

#[derive(Clone, Debug)]
pub enum CalcError {
    FuncNoName,
    FuncIncorrectArgCount(usize),
    FuncDoesNotExist(Vec<u8>),
    FuncArgsNotInBrackets,
    FuncExpectedComma,
    FuncExpectedArg,
    FuncHardcodedReassignAttempt(Vec<u8>),
    CannotParseNumber(Vec<u8>),
    CannotParseOperator(Vec<u8>),
    VarDoesNotExist(Vec<u8>),
    NoInput,
    TooManyAssignmentOps,
    InvalidFirstToken(Token),
    InvalidLastToken(Token),
    InvalidTokenSeq,
    ImbalancedBrackets,
    MisplacedComma,
    LhsMustBeVarOrFunc,
    FailedToSolveEquation,
    TooMuchRecursion,
}

impl fmt::Display for CalcError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let msg = match self {
            CalcError::FuncNoName => format!("function has no name"),
            CalcError::FuncIncorrectArgCount(num) => format!("function recieved {num} args"),
            CalcError::FuncDoesNotExist(name) => format!("function \"{}\" does not exist", String::from_utf8_lossy(&name)),
            CalcError::FuncArgsNotInBrackets => format!("function args must be inside function brackets"),
            CalcError::FuncExpectedComma => format!("function expected comma"),
            CalcError::FuncExpectedArg => format!("function expected argument"),
            CalcError::FuncHardcodedReassignAttempt(name) => format!("cannot reassign base function \"{}\"", String::from_utf8_lossy(&name)),
            CalcError::CannotParseNumber(name) => format!("string \"{}\" cannot be parsed as a number", String::from_utf8_lossy(&name)),
            CalcError::CannotParseOperator(name) => format!("string \"{}\" cannot be parsed as an operator", String::from_utf8_lossy(&name)),
            CalcError::VarDoesNotExist(name) => format!("variable \"{}\" does not exist", String::from_utf8_lossy(&name)),
            CalcError::NoInput => format!(""),
            CalcError::TooManyAssignmentOps => format!("too many assignment (=) operators"),
            CalcError::InvalidFirstToken(token) => format!("{:?} cannot be the first token", token),
            CalcError::InvalidLastToken(token) => format!("{:?} cannot be the last token", token),
            CalcError::InvalidTokenSeq => format!("invalid token sequence"),
            CalcError::ImbalancedBrackets => format!("imbalanced brackets"),
            CalcError::MisplacedComma => format!("misplaced comma"),
            CalcError::LhsMustBeVarOrFunc => format!("LHS must be a variable or a function"),
            CalcError::FailedToSolveEquation => format!("failed to solve expression"),
            CalcError::TooMuchRecursion => format!("exceeded recursion depth limit"),
        };
        write!(f, "{}", msg)
    }
}