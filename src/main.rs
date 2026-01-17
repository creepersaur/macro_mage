use crate::macro_handler::MacroHandler;
use colored::*;

mod errors;
mod lexer;
mod macro_handler;
mod value;

fn main() {
    let file_path = "hello.mage";
    let text = match lexer::read_file(file_path) {
        Ok(text) => text,
        Err(err) => {
            print_error(err);
            return;
        }
    };
    let lines = lexer::get_file_lines(&text);
    let tokens = lexer::get_line_tokens(&lines);

    let mut handler = MacroHandler::new();
	let mut line = 0;
	while line < tokens.len() {
		if let Err(e) = handler.run_line(line, &tokens[line]) {
            print_error(e);
        }
		line = handler.line + 1;
	}
}

fn print_error<E: std::fmt::Display>(err: E) {
    eprintln!("{}", format!("Error: {}", err).red());
}
