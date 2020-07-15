extern crate chip8_assembler;

use std::process;

fn main() {
	let options = chip8_assembler::options::get_program_options();
	let input_data = chip8_assembler::load_file(&options.arg_input).unwrap_or_else(
		|e| {
			println!("Could Not Load Input File: {:?}", e);
			process::exit(1);
		}
	);

	match chip8_assembler::assembler::assemble(input_data) {
		Ok(data) => {
			chip8_assembler::write_to_file(data)
		},

		Err(e) => println!("{:?}", e)
	}
}