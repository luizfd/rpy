//use crate::ir::ast::Expression;
//use crate::ir::ast::Statement;
//use crate::interpreter::interpreter::eval;

use interpreter::interpreter::{execute, ControlFlow};
use ir::ast::{Environment, Expression, Statement};

pub mod interpreter;
pub mod ir;
pub mod tc;

fn main() -> Result<(), String> {

    let exec_env = Environment::new();

    //let value_to_print = Expression::CString("hello, world!".to_string());

    let value_to_print = Expression::Add(
        Box::new(Expression::CReal(3.15)),
        Box::new(Expression::CReal(2.1)),
    );
    
    let print_stmt = Statement::Print(Box::new(value_to_print));

    match execute(print_stmt, &exec_env) {
        Ok(ControlFlow::Continue(_)) => println!("Print statement executed successfully"),
        Ok(ControlFlow::Return(_)) => println!("Unexpected return from print statement"),
        Err(e) => return Err(format!("Execution failed: {}", e)),
        
    }


    Ok(())
}
