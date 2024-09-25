use crate::mathengine::token::{tokenize, Token::{self, *}};
use std::f64::consts::{E, PI, TAU};
use std::collections::HashMap;
use super::error::CalcError;
use super::math_function::{Function, FunctionHandling};
use super::solve::Solver;
use super::token::TokenHandling;
use super::validate::{is_valid_lhs_function, validate_token_list};

pub struct Calculator {
    pub prev_answers: Vec<Token>,
    pub variables: HashMap<Vec<u8>, Token>,
    pub functions: HashMap<Vec<u8>, Function>,
    pub use_radians: bool,
}

impl Calculator {
    pub fn new() -> Calculator {
        let mut calc_engine = Calculator {
            prev_answers: Vec::new(),
            variables: HashMap::<Vec<u8>, Token>::new(),
            functions: HashMap::<Vec<u8>, Function>::new(),
            use_radians: true,
        };
        calc_engine.prev_answers.push(Number(0.0));
        calc_engine
            .variables
            .insert("pi".as_bytes().to_vec(), Number(PI));
        calc_engine
            .variables
            .insert("e".as_bytes().to_vec(), Number(E));
        calc_engine
            .variables
            .insert("tau".as_bytes().to_vec(), Number(TAU));

        calc_engine
    }
}

pub enum EvalResult<A, F> {
    Answer(A),
    Feedback(F),
}

pub trait Calc {
    fn eval(&mut self, input: &str) -> Result<EvalResult<f64, String>, CalcError>;
}

impl Calc for Calculator {
    fn eval(&mut self, input: &str) -> Result<EvalResult<f64, String>, CalcError> {
        // remove whitespace from input
        let trimmed_input: Vec<u8> = input.chars().filter(|c| !c.is_whitespace()).collect::<String>().bytes().collect();

        // turn string input into a list of tokens
        let mut tokens = tokenize(&trimmed_input)?;

        // make sure token list is a valid equation or assignment
        validate_token_list(&tokens)?;

        // if creating/reassigning a variable/function (expression contains a '=')
        if tokens.contains(&Assignment) {
            // split the expression into the parts before and after the '='
            let parts: Vec<&[Token]> = tokens.split(|t| t == &Assignment).collect();
            let mut lhs = parts[0].to_vec();
            let mut rhs = parts[1].to_vec();

            // check if lhs is a variable
            if lhs.len() == 1 {
                if let Variable(name) = &lhs[0] {
                    // lhs is a variable, assign value to new variable
                    // resolve variables on the rhs
                    self.resolve_variables(&mut rhs)?;
                    let value;
                    match self.solve(rhs, 0) {
                        Ok(answer) => value = answer[0].clone(),
                        Err(e) => return Err(e),
                    }

                    self.variables.insert(name.clone(), value);
                    return Ok(EvalResult::Feedback("assigned value to variable".to_string()));
                } else {
                    return Err(CalcError::LhsMustBeVarOrFunc);
                }
            } else {
                // make sure lhs is valid function 
                is_valid_lhs_function(&lhs)?;
                // lhs is a function, assign value to new function
                match self.create_function(&mut lhs, &mut rhs) {
                    Ok(func) => {
                        let func_name = func.name.clone();
                        self.functions.insert(func_name, func);
                        return Ok(EvalResult::Feedback("created function".to_string()));
                    }
                    Err(e) => {
                        return Err(e);
                    }
                }
            }
        // otherwise just solve it
        } else {
            // resolve variables on the right side
            self.resolve_variables(&mut tokens)?;

            let answer;
            match self.solve(tokens, 0) {
                Ok(a) => answer = a[0].clone(),
                Err(e) => return Err(e),
            }

            if let Number(n) = answer {
                self.prev_answers[0] = Number(n);
                self.variables
                    .insert(b"ans".to_vec(), self.prev_answers[0].clone());
                return Ok(EvalResult::Answer(n));
            } else {
                return Err(CalcError::FailedToSolveEquation);
            }
        }
    }
}
