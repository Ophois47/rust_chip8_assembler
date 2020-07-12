mod lexer;
mod parser;
mod semantics;
mod codegenerator;

use codegenerator::CodeGenerator;

use std::io::Error;

// Consume Input Data and Assemble Code
pub fn assemble(input_data: Vec<u8>) -> Result<Vec<u8>, Error> {
	/* Transform Input Data to Tokens */
	let tokens = lexer::tokenize(&input_data[..]).unwrap();
	/* Transform Tokens Into Expressions */
	let exprs = parser::parse(tokens).unwrap();

	/* Generate OpCodes From Expressions */
	let codegen = CodeGenerator::new();
	let opcodes = codegen.generate(exprs);

	Ok(opcodes)
}