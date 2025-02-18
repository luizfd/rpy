//use crate::ir::ast::Expression;
//use crate::ir::ast::Statement;
//use crate::interpreter::interpreter::eval;

use interpreter::interpreter::{execute, ControlFlow, EnvValue};
use ir::ast::{Environment, Expression, Statement, Type};
use tc::type_checker::check_stmt;

pub mod interpreter;
pub mod ir;
pub mod tc;
fn main() -> Result<(), String> {
    let type_env = Environment::new();

    let exec_env = Environment::new();

    let file_path = Expression::CString("output.txt".to_string());

    let read_file_exp = Expression::ReadFile(Box::new(file_path));

    let assign_stmt = Statement::Assignment(
        "fileContents".to_string(),
        Box::new(read_file_exp),
        Some(Type::TString),
    );

    match check_stmt(assign_stmt.clone(), &type_env) {
        Ok(_) => println!("Type-checking passed!"),
        Err(e) => return Err(format!("Type-checking failed: {}", e)),
    }

    match execute(assign_stmt, &exec_env) {
        Ok(ControlFlow::Continue(new_env)) => {
            if let Some(EnvValue::Exp(Expression::CString(contents))) = new_env.search_frame("fileContents".to_string()) {
                println!("File contents: {}", contents);
            } else {
                return Err(String::from("Failed to retrieve file contents from environment"));
            }
        }
        Ok(ControlFlow::Return(value)) => {
            println!("Returned value: {:?}", value);
        }
        Err(e) => return Err(format!("Execution failed: {}", e)),
    }

    Ok(())

}
