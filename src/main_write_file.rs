//use crate::ir::ast::Expression;
//use crate::ir::ast::Statement;
//use crate::interpreter::interpreter::eval;

use interpreter::interpreter::execute;
use ir::ast::{Environment, Expression, Statement};

pub mod interpreter;
pub mod ir;
pub mod tc;
fn main() -> Result<(), String> {
    let exec_env = Environment::new();

    let file_path = Expression::CString("output.txt".to_string());
    let content = Expression::CString("teste de escrita".to_string());

    let write_stmt = Statement::WriteToFile(Box::new(file_path), Box::new(content));


    match execute(write_stmt, &exec_env) {
        Ok(_) => println!("File written successfully"),
        Err(e) => return Err(format!("Execution failed: {}", e)),
    }

    Ok(())
}
