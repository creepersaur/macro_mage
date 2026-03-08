use crate::macro_handler::MacroHandler;
use colored::*;
use rdev::{Event, EventType, Key, listen};
use std::sync::{
    Arc,
    atomic::{AtomicBool, Ordering},
};
use std::thread;

mod errors;
mod lexer;
mod macro_handler;
mod value;

fn main() {
    let running = Arc::new(AtomicBool::new(true));
    let running_listener = running.clone();

    thread::spawn(move || {
        let callback = move |event: Event| {
            if let EventType::KeyPress(key) = event.event_type {
                if key == Key::F3 {
                    println!("F3 pressed, stopping macro & listener.");
                    running_listener.store(false, Ordering::SeqCst);
                }
            }
        };

        listen(callback).expect("failed to listen");
    });

    println!("{}", "-- PRESS [F3] TO QUIT MACRO ---".yellow());

    let file_path = "hello.mage";
    let text = match lexer::read_file(file_path) {
        Ok(text) => text,
        Err(err) => {
            print_error(None, err);
            return;
        }
    };
    let lines = lexer::get_file_lines(&text);
    let tokens = lexer::get_line_tokens(&lines);
    let mut handler = MacroHandler::new();
    let mut line = 0;

    while line < tokens.len() {
        if !running.load(Ordering::SeqCst) {
            break;
        }

        if let Err(e) = handler.run_line(line as i32, &tokens[line]) {
            print_error(Some(line), e);
        }
        if handler.line < 0 {
            break;
        }
        line = handler.line as usize + 1;
    }

    println!("{}", "Macro ran successfully!".green());
    std::process::exit(0);
}

fn print_error<E: std::fmt::Display>(line: Option<usize>, err: E) {
    if let Some(line) = line {
        eprintln!("{}", format!("Error on line {}: {}", line + 1, err).red());
    } else {
        eprintln!("{}", format!("Error: {}", err).red());
    }
}
