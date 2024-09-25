use crate::mathengine::token::{self, Token::{self, *}};

use super::{core::Calculator, error::CalcError, math_function::FunctionHandling};

pub trait Solver {
    fn solve(&mut self, equation: Vec<Token>, depth: i32) -> Result<Vec<Token>, CalcError>;
    fn solve_rec(&mut self, token_list: &mut Vec<Token>, depth: i32) -> Result<Vec<Token>, CalcError>;
}

impl Solver for Calculator {
    fn solve(&mut self, equation: Vec<Token>, depth: i32) -> Result<Vec<Token>, CalcError> {
        if depth > 32 {
            return Err(CalcError::TooMuchRecursion);
        }
        if equation.len() == 0 {
            let mut v = Vec::new();
            v.push(Number(0.0));
            return Ok(v);
        }
        let mut token_list = equation.clone();
        let answer = self.solve_rec(&mut token_list, depth + 1);
        answer
    }

    fn solve_rec(&mut self, token_list: &mut Vec<Token>, depth: i32) -> Result<Vec<Token>, CalcError> {
        if depth > 32 {
            return Err(CalcError::TooMuchRecursion);
        }
        let mut i = 0;
        while i < token_list.len() {
            match &token_list[i] {
                Token::LeftBracket => {
                    let right_bracket_index =
                        {
                            match get_matching_bracket_index(&token_list[i + 1..]) {
                                Some(index) => index,
                                None => return Err(CalcError::ImbalancedBrackets)
                            }
                        }
                        + i + 1;
                    match self.solve_rec(&mut token_list[i + 1..right_bracket_index].into(), depth + 1) {
                        Ok(answer) => token_list.splice(i..right_bracket_index + 1, answer),
                        Err(e) => return Err(e),
                    };
                    i = 0;
                }
                Token::FunctionName(func_name) => {
                    let right_bracket_index =
                    {
                        match get_matching_bracket_index(&token_list[i + 2..]) {
                            Some(index) => index,
                            None => return Err(CalcError::ImbalancedBrackets)
                        }
                    }
                        + i + 1;
                    match self
                        .solve_function(&func_name, &token_list[i + 2..right_bracket_index + 1], depth + 1)
                    {
                        Ok(answer) => {
                            token_list.splice(i..right_bracket_index + 2, Vec::from([answer]))
                        }
                        Err(e) => return Err(e),
                    };
                    i = 0;
                }
                _ => {
                    i += 1;
                }
            }
        }
        i = 0;
        while i < token_list.len() {
            match token_list[i] {
                Token::Exponentation => {
                    if let Token::Number(a) = token_list[i - 1] {
                        if let Token::Number(b) = token_list[i + 1] {
                            let answer = a.powf(b);
                            token_list[i - 1] = Token::Number(answer);
                            token_list.remove(i);
                            token_list.remove(i);
                            i = 0;
                        }
                    }
                }
                _ => {
                    i += 1;
                }
            }
        }
        i = 0;
        while i < token_list.len() {
            match token_list[i] {
                Token::Multiplication => {
                    if let Token::Number(a) = token_list[i - 1] {
                        if let Token::Number(b) = token_list[i + 1] {
                            let answer = a * b;
                            token_list[i - 1] = Token::Number(answer);
                            token_list.remove(i);
                            token_list.remove(i);
                            i = 0;
                        }
                    }
                }
                Token::Division => {
                    if let Token::Number(a) = token_list[i - 1] {
                        if let Token::Number(b) = token_list[i + 1] {
                            let answer = a / b;
                            token_list[i - 1] = Token::Number(answer);
                            token_list.remove(i);
                            token_list.remove(i);
                            i = 0;
                        }
                    }
                }
                Token::Modulation => {
                    if let Token::Number(a) = token_list[i - 1] {
                        if let Token::Number(b) = token_list[i + 1] {
                            let answer = a % b;
                            token_list[i - 1] = Token::Number(answer);
                            token_list.remove(i);
                            token_list.remove(i);
                            i = 0;
                        }
                    }
                }
                _ => {
                    i += 1;
                }
            }
        }
        i = 0;
        while i < token_list.len() {
            match token_list[i] {
                Token::BitwiseAnd => {
                    if let Token::Number(a) = token_list[i - 1] {
                        if let Token::Number(b) = token_list[i + 1] {
                            let answer = (a as i64) & (b as i64);
                            token_list[i - 1] = Token::Number(answer as f64);
                            token_list.remove(i);
                            token_list.remove(i);
                            i = 0;
                        }
                    }
                }
                Token::BitwiseOr => {
                    if let Token::Number(a) = token_list[i - 1] {
                        if let Token::Number(b) = token_list[i + 1] {
                            let answer = (a as i64) | (b as i64);
                            token_list[i - 1] = Token::Number(answer as f64);
                            token_list.remove(i);
                            token_list.remove(i);
                            i = 0;
                        }
                    }
                }
                Token::BitwiseXor => {
                    if let Token::Number(a) = token_list[i - 1] {
                        if let Token::Number(b) = token_list[i + 1] {
                            let answer = (a as i64) ^ (b as i64);
                            token_list[i - 1] = Token::Number(answer as f64);
                            token_list.remove(i);
                            token_list.remove(i);
                            i = 0;
                        }
                    }
                }
                Token::BitwiseLeftShift => {
                    if let Token::Number(a) = token_list[i - 1] {
                        if let Token::Number(b) = token_list[i + 1] {
                            let answer = (a as i64) << (b as i64);
                            token_list[i - 1] = Token::Number(answer as f64);
                            token_list.remove(i);
                            token_list.remove(i);
                            i = 0;
                        }
                    }
                }
                Token::BitwiseRightShift => {
                    if let Token::Number(a) = token_list[i - 1] {
                        if let Token::Number(b) = token_list[i + 1] {
                            let answer = (a as i64) >> (b as i64);
                            token_list[i - 1] = Token::Number(answer as f64);
                            token_list.remove(i);
                            token_list.remove(i);
                            i = 0;
                        }
                    }
                }
                _ => {
                    i += 1;
                }
            }
        }
        i = 0;
        while i < token_list.len() {
            match token_list[i] {
                Token::Addition => {
                    if let Token::Number(a) = token_list[i - 1] {
                        if let Token::Number(b) = token_list[i + 1] {
                            let answer = a + b;
                            token_list[i - 1] = Token::Number(answer);
                            token_list.remove(i);
                            token_list.remove(i);
                            i = 0;
                        }
                    }
                }
                Token::Subtraction => {
                    if let Token::Number(a) = token_list[i - 1] {
                        if let Token::Number(b) = token_list[i + 1] {
                            let answer = a - b;
                            token_list[i - 1] = Token::Number(answer);
                            token_list.remove(i);
                            token_list.remove(i);
                            i = 0;
                        }
                    }
                }
                _ => {
                    i += 1;
                }
            }
        }

        Ok(token_list.to_vec())
    }
}

pub fn get_matching_bracket_index(token_list: &[Token]) -> Option<usize> {
    let mut depth = 1;
    for i in 0..token_list.len() {
        match token_list[i] {
            Token::LeftBracket => depth += 1,
            Token::RightBracket => depth -= 1,
            _ => (),
        }
        if depth == 0 {
            return Some(i);
        }
    }

    None
}
