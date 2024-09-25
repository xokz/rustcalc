use super::error::CalcError;
use crate::mathengine::token::Token::{self, *};

// just all of the operator tokens as a macro so I dont have to type all of them every time
#[macro_export]
macro_rules! operators {
    () => {
        Assignment
            | Addition
            | Subtraction
            | Division
            | Multiplication
            | Modulation
            | Exponentation
            | BitwiseLeftShift
            | BitwiseRightShift
            | BitwiseAnd
            | BitwiseOr
            | BitwiseXor
            
    };
}

pub fn validate_token_list(tokens: &Vec<Token>) -> Result<(), CalcError> {
    // make sure input is not empty
    if tokens.len() == 0 {
        return Err(CalcError::NoInput);
    }

    // make sure there are one or less '='
    let mut equal_count = 0;
    for token in tokens {
        if token == &Assignment {
            equal_count += 1;
        }
    }
    if equal_count > 1 {
        return Err(CalcError::TooManyAssignmentOps);
    }

    // make sure first and last tokens are valid
    match tokens[0] {
        Comma | RightBracket | operators!() => return Err(CalcError::InvalidFirstToken(tokens[0].clone())),
        _ => (),
    }
    match tokens[tokens.len() - 1] {
        Comma | LeftBracket | FunctionName(_) | operators!() => {
            return Err(CalcError::InvalidLastToken(tokens[tokens.len()-1].clone()))
        }
        _ => (),
    }

    // make sure all tokens are proceeded by a valid token
    for i in 0..tokens.len() - 1 {
        if !is_next_token_valid(&tokens[i], &tokens[i + 1]) {
            return Err(CalcError::InvalidTokenSeq);
        }
    }

    // make sure all brackets are closed
    let mut bracket_depth: i32 = 0;
    for token in tokens {
        match token {
            LeftBracket => bracket_depth += 1,
            RightBracket => bracket_depth -= 1,
            _ => (),
        }
    }
    if bracket_depth != 0 {
        return Err(CalcError::ImbalancedBrackets);
    }
    bracket_depth = 0;

    // make sure commas are only found inside of functions
    let mut in_function_brackets: bool = false;
    for token in tokens {
        match token {
            FunctionName(_) => {
                in_function_brackets = true;
            }
            LeftBracket => {
                if in_function_brackets {
                    bracket_depth += 1;
                }
            }
            RightBracket => {
                if in_function_brackets {
                    bracket_depth -= 1;
                    if bracket_depth == 0 {
                        in_function_brackets = false;
                    }
                }
            }
            Comma => {
                if !in_function_brackets {
                    return Err(CalcError::MisplacedComma);
                }
            }
            _ => {}
        }
    }

    // token list is valid and ready for solving
    Ok(())
}

fn is_next_token_valid(current: &Token, next: &Token) -> bool {
    match current {
        // current token is a number or var
        Number(_) | Variable(_) => {
            // if next token matches any of these, its invalid
            match next {
                Number(_) => return false,
                Variable(_) => return false,
                FunctionName(_) => return false,
                LeftBracket => return false,
                _ => return true,
            };
        }

        // current token is an operator
        operators!() => {
            // if next token matches any of these, its invalid
            match next {
                operators!() => return false,
                RightBracket => return false,
                _ => return true,
            };
        }

        // current token is a '('
        LeftBracket => {
            // if next token matches any of these, its invalid
            match next {
                operators!() => return false,
                _ => return true,
            };
        }

        // current token is a ')'
        RightBracket => {
            // if next token matches any of these, its invalid
            match next {
                Number(_) => return false,
                Variable(_) => return false,
                FunctionName(_) => return false,
                LeftBracket => return false,
                _ => return true,
            };
        }

        // current token is a function
        FunctionName(_) => {
            // the only valid next token is a left bracket
            match next {
                LeftBracket => return true,
                _ => return false,
            };
        }

        _ => return false,
    };
}

pub fn is_valid_lhs_function(tokens: &Vec<Token>) -> Result<(), CalcError> {
    // make sure first token is a function name that is not reserved
    match &tokens[0] {
        FunctionName(name) => match &name[..] {
            b"sin" => return Err(CalcError::FuncHardcodedReassignAttempt(b"sin".to_vec())),
            b"cos" => return Err(CalcError::FuncHardcodedReassignAttempt(b"cos".to_vec())),
            b"tan" => return Err(CalcError::FuncHardcodedReassignAttempt(b"tan".to_vec())),
            b"asin" => return Err(CalcError::FuncHardcodedReassignAttempt(b"asin".to_vec())),
            b"acos" => return Err(CalcError::FuncHardcodedReassignAttempt(b"acos".to_vec())),
            b"atan" => return Err(CalcError::FuncHardcodedReassignAttempt(b"atan".to_vec())),
            b"sqrt" => return Err(CalcError::FuncHardcodedReassignAttempt(b"sqrt".to_vec())),
            b"fact" => return Err(CalcError::FuncHardcodedReassignAttempt(b"fact".to_vec())),
            b"log" => return Err(CalcError::FuncHardcodedReassignAttempt(b"log".to_vec())),
            b"ln" => return Err(CalcError::FuncHardcodedReassignAttempt(b"ln".to_vec())),
            _ => (),
        },
        _ => return Err(CalcError::FuncNoName),
    }

    // args must be encased in brackets
    if &tokens[1] != &LeftBracket {
        return Err(CalcError::FuncArgsNotInBrackets);
    }
    if &tokens[tokens.len() - 1] != &RightBracket {
        return Err(CalcError::FuncArgsNotInBrackets);
    }

    // contents of brackets must be variables and commas in alternating order
    // eg. func(a,b,c)
    let mut prev_token_was_arg: bool = false;

    for token in &tokens[2..tokens.len() - 1] {
        if prev_token_was_arg {
            match token {
                Comma => {
                    prev_token_was_arg = false;
                    continue;
                }
                _ => return Err(CalcError::FuncExpectedComma),
            }
        } else {
            match token {
                Variable(_) => {
                    prev_token_was_arg = true;
                    continue;
                }
                _ => return Err(CalcError::FuncExpectedArg),
            }
        }
    }

    Ok(())
}
