use crate::{mathengine::{core::Calculator, error::CalcError, token::Token::*}, operators};
use std::fmt;
#[derive(Clone, Debug, PartialEq)]
pub enum Token {
    Addition,       // +
    Subtraction,    // -
    Division,       // /
    Multiplication, // *
    Modulation,     // %
    Exponentation,  // ^

    BitwiseLeftShift,  // <<
    BitwiseRightShift, // >>
    BitwiseAnd,        // &
    BitwiseOr,         // |
    BitwiseXor,        // ^^

    LeftBracket,  // (
    RightBracket, // )

    Comma, // ,

    Assignment, // =

    UnresolvedString(Vec<u8>),
    Variable(Vec<u8>),
    FunctionName(Vec<u8>),
    FunctionArg(usize),

    Number(f64),
}

impl fmt::Display for Token {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let msg = match self {
            Addition => format!("+ "),
            Subtraction => format!("- "),
            Division => format!("/ "),
            Multiplication => format!("* "),
            Modulation => format!("% "),
            Exponentation => format!("^ "),
            BitwiseLeftShift => format!("<< "),
            BitwiseRightShift => format!(">> "),
            BitwiseAnd => format!("& "),
            BitwiseOr => format!("| "),
            BitwiseXor => format!("^^ "),
            LeftBracket => format!("( "),
            RightBracket => format!(") "),
            Comma => format!(", "),
            Assignment => format!("= "),
            UnresolvedString(name) => format!("{} ", String::from_utf8_lossy(&name)),
            Variable(name) => format!("{} ", String::from_utf8_lossy(&name)),
            FunctionName(name)=> format!("{} ", String::from_utf8_lossy(&name)),
            FunctionArg(index) => format!("[{}] ", index),
            Number(num) => format!("{} ", num),
        };
        write!(f, "{}", msg)
    }
}

pub fn print_token_list(tokens: &[Token]) {
    let mut msg = String::new();
    for token in tokens {
        msg.push_str(&format!("{}", token));
    }
    println!("{}", msg);
}

pub fn match_token(key: &[u8]) -> Option<Token> {
    let result = match key {
        b"<<" => Some(BitwiseLeftShift),
        b">>" => Some(BitwiseRightShift),
        b"^" => Some(Exponentation),
        b"+" => Some(Addition),
        b"-" => Some(Subtraction),
        b"/" => Some(Division),
        b"*" => Some(Multiplication),
        b"%" => Some(Modulation),
        b"&" => Some(BitwiseAnd),
        b"|" => Some(BitwiseOr),
        b"^^" => Some(BitwiseXor),
        b"(" => Some(LeftBracket),
        b")" => Some(RightBracket),
        b"," => Some(Comma),
        b"=" => Some(Assignment),
        _ => None,
    };
    result
}

pub fn tokenize(input: &Vec<u8>) -> Result<Vec<Token>, CalcError> {
    let mut tokens = get_tokens(input)?;
    resolve_strings(&mut tokens);
    Ok(tokens)
}

fn get_tokens(input: &[u8]) -> Result<Vec<Token>, CalcError> {
    // this will store the list of tokens, and will be returned
    let mut tokens: Vec<Token> = Vec::new();

    let is_number_part = |x: u8| (x as char).is_digit(10) || x == b'.';
    let is_string_part = |x: u8| (x as char).is_alphabetic() || x == b'_';

    // this loop parses the input bytes into a vec of raw tokens
    let mut i: usize = 0;
    let len = input.len();
    'outer: while i < len {
        // numbers
        if is_number_part(input[i]) {
            let slice_bounds = get_token_bounds(is_number_part, i, &input);
            let string_attempt = input[slice_bounds.0..slice_bounds.1].to_vec();
            match String::from_utf8(string_attempt.to_vec()).unwrap().parse::<f64>()
            {
                Ok(n) => {
                    tokens.push(Number(n));
                }
                Err(_) => return Err(CalcError::CannotParseNumber(string_attempt)),
            }
            i = slice_bounds.1;
        }
        // strings
        else if is_string_part(input[i]) {
            let slice_bounds: (usize, usize) = get_token_bounds(is_string_part, i, &input);
            // just stored as a string for now, will later be turned into a variable, function, or command
            tokens.push(UnresolvedString(
                input[slice_bounds.0..slice_bounds.1].to_vec(),
            ));
            i = slice_bounds.1;
        } else if input[i].is_ascii() {
            //operator token
            for j in (0..3).rev() {
                if i + j <= len {
                    if let Some(t) = match_token(&input[i..i + j]) {
                        tokens.push(t.clone());
                        i += j;
                        continue 'outer;
                    }
                }
            }
            let operator_string = input[i..(i + 3).clamp(1, input.len())].to_vec();
            return Err(CalcError::CannotParseOperator(operator_string));
        } else {
            return Err(CalcError::InvalidTokenSeq);
        }
    }

    // handle signs (negative, positive)
    let mut i: usize = 0;
    if tokens.len() == 0 {
        return Ok(tokens);
    }
    while i < tokens.len() - 1 {
        match tokens[i] {
            Subtraction => {
                if let Number(n) = tokens[i + 1] {
                    if i == 0 {
                        tokens[i + 1] = Number(-n);
                        tokens.remove(i);
                    } else if match tokens[i - 1] {operators!() => true, _ => false}
                    {
                        tokens[i + 1] = Number(-n);
                        tokens.remove(i);
                    } else {
                        i += 1;
                    }
                } else {
                    i += 1;
                }
            }
            Addition => {
                if let Number(n) = tokens[i + 1] {
                    if i == 0 {
                        tokens[i + 1] = Number(n);
                        tokens.remove(i);
                    } else if match tokens[i - 1] {operators!() => true, _ => false}
                    {
                        tokens[i + 1] = Number(n);
                        tokens.remove(i);
                    } else {
                        i += 1;
                    }
                } else {
                    i += 1;
                }
            }
            _ => {
                i += 1;
            }
        }
    }

    Ok(tokens)
}

fn resolve_strings(tokens: &mut Vec<Token>) {
    for i in 0..tokens.len() {
        if let UnresolvedString(name) = &tokens[i] {
            if i < tokens.len() - 1 {
                match tokens[i + 1] {
                    LeftBracket => tokens[i] = FunctionName(name.clone()),
                    _ => tokens[i] = Variable(name.clone()),
                }
            } else {
                tokens[i] = Variable(name.clone());
            }
        }
    }
}

pub trait TokenHandling {
    fn resolve_variables(&self, tokens: &mut Vec<Token>) -> Result<(), CalcError>;
}

impl TokenHandling for Calculator {
    fn resolve_variables(&self, tokens: &mut Vec<Token>) -> Result<(), CalcError> {
        for token in tokens {
            if let Variable(name) = token {
                let hash_try = self.variables.get(name);
                match hash_try {
                    Some(number) => {
                        *token = number.clone();
                    }
                    None => {
                        return Err(CalcError::VarDoesNotExist(name.to_vec()));
                    }
                }
            }
        }
        Ok(())
    }
}

fn get_token_bounds<F>(f: F, start: usize, input_chars: &[u8]) -> (usize, usize)
where
    F: Fn(u8) -> bool,
{
    let mut end: usize = start + 1;

    while end < input_chars.len() && f(input_chars[end]) {
        end += 1;
    }

    (start, end)
}
