//use crate::ir::ast::Expression;
//use crate::ir::ast::Statement;
//use crate::interpreter::interpreter::eval;

use interpreter::interpreter::{execute, ControlFlow};
use ir::ast::{Environment, Expression, Statement};

pub mod interpreter;
pub mod ir;
pub mod tc;

fn main() -> Result<(), String> {
    let exec_env: Environment<_> = Environment::new();

    let read_string_stmt = Statement::Print(Box::new(Expression::ReadString));
    match execute(read_string_stmt, &exec_env) {
        Ok(ControlFlow::Continue(_)) => println!("Statement executed successfully"),
        Ok(ControlFlow::Return(_)) => println!("Unexpected return from statement"),
        Err(e) => return Err(format!("Execution failed: {}", e)),
    }

    let read_int_stmt = Statement::Print(Box::new(Expression::ReadInt));
    match execute(read_int_stmt, &exec_env) {
        Ok(ControlFlow::Continue(_)) => println!("statement executed successfully"),
        Ok(ControlFlow::Return(_)) => println!("Unexpected return from statement"),
        Err(e) => return Err(format!("Execution failed: {}", e)),
    }

    let read_float_stmt = Statement::Print(Box::new(Expression::ReadFloat));
    match execute(read_float_stmt, &exec_env) {
        Ok(ControlFlow::Continue(_)) => println!("statement executed successfully"),
        Ok(ControlFlow::Return(_)) => println!("Unexpected return from statement"),
        Err(e) => return Err(format!("Execution failed: {}", e)),
    }
    
    Ok(())

}
