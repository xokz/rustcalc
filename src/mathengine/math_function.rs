use std::f64::consts::PI;

use crate::mathengine::token::Token::*;

use super::{
    core::Calculator,
    error::CalcError,
    solve::Solver,
    token::{Token, TokenHandling},
};

#[derive(Clone)]
pub struct Function {
    pub name: Vec<u8>,
    pub arg_count: usize,
    pub func: Vec<Token>,
}

impl Function {
    fn new() -> Function {
        Function {
            name: Vec::new(),
            arg_count: 0,
            func: Vec::new(),
        }
    }
}

pub trait FunctionHandling {
    fn create_function(
        &mut self,
        lhs: &mut Vec<Token>,
        rhs: &mut Vec<Token>,
    ) -> Result<Function, CalcError>;
    fn solve_function(&mut self, name: &[u8], args: &[Token], depth: i32) -> Result<Token, CalcError>;
}

impl FunctionHandling for Calculator {
    fn create_function(
        &mut self,
        lhs: &mut Vec<Token>,
        rhs: &mut Vec<Token>,
    ) -> Result<Function, CalcError> {
        let mut arg_count: usize = 0;
        let mut func = Function::new();
        for i in 0..lhs.len() {
            if let Variable(arg_name) = &lhs[i] {
                // replace cooresponding rhs variable(s) with a function argument index
                for j in 0..rhs.len() {
                    if let Variable(var_name) = &rhs[j] {
                        if arg_name == var_name {
                            rhs[j] = FunctionArg(arg_count);
                        }
                    }
                }

                arg_count += 1;
            }
        }

        self.resolve_variables(rhs)?;

        // assign proper values to func
        if let FunctionName(func_name) = &lhs[0] {
            func.name = func_name.clone();
        } else {
            return Err(CalcError::FuncNoName);
        }
        func.arg_count = arg_count;
        func.func = rhs.to_vec();

        Ok(func)
    }

    fn solve_function(&mut self, name: &[u8], arg_slice: &[Token], depth: i32) -> Result<Token, CalcError> {
        let mut args: Vec<f64> = Vec::new();

        for arg in arg_slice.split(|t| t == &Comma) {
            if let Number(n) = self.solve(arg.to_vec(), depth + 1)?[0].clone() {
                args.push(n);
            }
        }

        let angle_mode = if self.use_radians { 1.0 } else { PI / 180.0 };
        match name {
            b"sin" => return Ok(Number((args[0] * angle_mode).sin())),
            b"cos" => return Ok(Number((args[0] * angle_mode).cos())),
            b"tan" => return Ok(Number((args[0] * angle_mode).tan())),
            b"asin" => return Ok(Number((args[0] * angle_mode).asin())),
            b"acos" => return Ok(Number((args[0] * angle_mode).acos())),
            b"atan" => return Ok(Number((args[0] * angle_mode).atan())),
            b"sqrt" => return Ok(Number((args[0]).sqrt())),
            b"log" => return Ok(Number((args[0]).log(10.0))),
            b"ln" => return Ok(Number((args[0]).log2())),
            _ => match self.functions.get(name) {
                Some(function) => {
                    if function.arg_count != args.len() {
                        return Err(CalcError::FuncIncorrectArgCount(function.arg_count));
                    }
                    let mut expr = function.func.clone();
                    for token in &mut expr {
                        if let FunctionArg(index) = token {
                            *token = Number(args[*index]);
                        }
                    }
                    match self.solve(expr, depth + 1) {
                        Ok(answer) => return Ok(answer[0].clone()),
                        Err(e) => return Err(e),
                    };
                }
                None => return Err(CalcError::FuncDoesNotExist(name.to_vec())),
            },
        }
    }
}
