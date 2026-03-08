use crate::{errors::MacroError, lexer::Token, value::Value};
use colored::*;
use rustautogui::RustAutoGui;
use std::{collections::HashMap, thread, time::Duration};

#[allow(unused)]
pub struct MacroHandler {
    rag: RustAutoGui,
    variables: HashMap<String, Value>,
    // function_pointers: HashMap<String, usize>,
    loop_stack: Vec<(usize, i32)>,
    function_stack: Vec<usize>,
    pub line: i32,
}

impl MacroHandler {
    pub fn new() -> Self {
        Self {
            rag: RustAutoGui::new(false).unwrap(),
            variables: HashMap::new(),
            loop_stack: vec![],
            // function_pointers: HashMap::new(),
            function_stack: vec![],
            line: 0,
        }
    }

    pub fn run_line(&mut self, num: i32, line: &Vec<Token>) -> Result<(), MacroError> {
        self.line = num;

        let mut args = line.into_iter();
        let command = args.next().unwrap();

        match command {
            Token::Identifier(text) => self.run_command(text.to_string(), args.as_slice())?,

            unknown_command => {
                return Err(MacroError::InvalidCommand(unknown_command.to_string()));
            }
        }

        Ok(())
    }

    pub fn minimum_argument_count(
        name: String,
        args: &[Token],
        count: usize,
        optional: &'static str,
    ) -> Result<(), MacroError> {
        if args.len() < count {
            Err(MacroError::InvalidArgumentCount(name, optional))
        } else {
            Ok(())
        }
    }
}

// RUN COMMAND
impl MacroHandler {
    pub fn run_command(&mut self, command: String, args: &[Token]) -> Result<(), MacroError> {
        ////////////////////////////////////
        // MOUSE
        ////////////////////////////////////

        if command.eq_ignore_ascii_case("setmouse") {
            Self::minimum_argument_count(command.clone(), &args, 2, "2-3")?;

            return self
                .rag
                .move_mouse_to(
                    self.parse_uint_optional(&args[0])?,
                    self.parse_uint_optional(&args[1])?,
                    if args.len() >= 3 {
                        self.parse_float_optional(&args[2])?.unwrap_or(0.0)
                    } else {
                        0.0
                    },
                )
                .map_err(|source| MacroError::AutoGuiError { source });
        }

        if command.eq_ignore_ascii_case("movemouse") {
            Self::minimum_argument_count(command.clone(), &args, 2, "2-3")?;

            return self
                .rag
                .move_mouse(
                    self.parse_int(&args[0])?,
                    self.parse_int(&args[1])?,
                    if args.len() >= 3 {
                        self.parse_float_optional(&args[2])?.unwrap_or(0.0)
                    } else {
                        0.0
                    },
                )
                .map_err(|source| MacroError::AutoGuiError { source });
        }

        if command.eq_ignore_ascii_case("click") {
            if args.len() == 0 {
                return self
                    .rag
                    .click(rustautogui::MouseClick::LEFT)
                    .map_err(|source| MacroError::AutoGuiError { source });
            }

            let button = self.parse_string_optional(&args[0])?;

            if let None = button {
                return self
                    .rag
                    .click(rustautogui::MouseClick::LEFT)
                    .map_err(|source| MacroError::AutoGuiError { source });
            }

            let button = button.unwrap();

            return if button.eq_ignore_ascii_case("left") {
                self.rag
                    .click(rustautogui::MouseClick::LEFT)
                    .map_err(|source| MacroError::AutoGuiError { source })
            } else if button.eq_ignore_ascii_case("right") {
                self.rag
                    .click(rustautogui::MouseClick::RIGHT)
                    .map_err(|source| MacroError::AutoGuiError { source })
            } else if button.eq_ignore_ascii_case("middle") {
                self.rag
                    .click(rustautogui::MouseClick::MIDDLE)
                    .map_err(|source| MacroError::AutoGuiError { source })
            } else {
                return Err(MacroError::UnknownMouseButton(button));
            };
        }

        if command.eq_ignore_ascii_case("doubleclick") {
            Self::minimum_argument_count(command.clone(), &args, 1, "1")?;

            return self
                .rag
                .double_click()
                .map_err(|source| MacroError::AutoGuiError { source });
        }

        if command.eq_ignore_ascii_case("mousedown") {
            if args.len() == 0 {
                return self
                    .rag
                    .click_down(rustautogui::MouseClick::LEFT)
                    .map_err(|source| MacroError::AutoGuiError { source });
            }

            let button = self.parse_string(&args[0])?;

            return if button.eq_ignore_ascii_case("left") {
                self.rag
                    .click_down(rustautogui::MouseClick::LEFT)
                    .map_err(|source| MacroError::AutoGuiError { source })
            } else if button.eq_ignore_ascii_case("right") {
                self.rag
                    .click_down(rustautogui::MouseClick::RIGHT)
                    .map_err(|source| MacroError::AutoGuiError { source })
            } else if button.eq_ignore_ascii_case("middle") {
                self.rag
                    .click_down(rustautogui::MouseClick::MIDDLE)
                    .map_err(|source| MacroError::AutoGuiError { source })
            } else {
                return Err(MacroError::UnknownMouseButton(button));
            };
        }

        if command.eq_ignore_ascii_case("mouseup") {
            if args.len() == 0 {
                return self
                    .rag
                    .click_up(rustautogui::MouseClick::LEFT)
                    .map_err(|source| MacroError::AutoGuiError { source });
            }

            let button = self.parse_string(&args[0])?;

            return if button.eq_ignore_ascii_case("left") {
                self.rag
                    .click_up(rustautogui::MouseClick::LEFT)
                    .map_err(|source| MacroError::AutoGuiError { source })
            } else if button.eq_ignore_ascii_case("right") {
                self.rag
                    .click_up(rustautogui::MouseClick::RIGHT)
                    .map_err(|source| MacroError::AutoGuiError { source })
            } else if button.eq_ignore_ascii_case("middle") {
                self.rag
                    .click_up(rustautogui::MouseClick::MIDDLE)
                    .map_err(|source| MacroError::AutoGuiError { source })
            } else {
                return Err(MacroError::UnknownMouseButton(button));
            };
        }

        if command.eq_ignore_ascii_case("dragto") {
            Self::minimum_argument_count(command.clone(), &args, 2, "2-3")?;

            return self
                .rag
                .drag_mouse_to(
                    self.parse_uint_optional(&args[0])?,
                    self.parse_uint_optional(&args[1])?,
                    if args.len() >= 3 {
                        self.parse_float_optional(&args[2])?.unwrap_or(0.0)
                    } else {
                        0.0
                    },
                )
                .map_err(|source| MacroError::AutoGuiError { source });
        }

        if command.eq_ignore_ascii_case("drag") {
            Self::minimum_argument_count(command.clone(), &args, 2, "2-3")?;

            return self
                .rag
                .drag_mouse(
                    self.parse_int(&args[0])?,
                    self.parse_int(&args[1])?,
                    if args.len() >= 3 {
                        self.parse_float_optional(&args[2])?.unwrap_or(0.0)
                    } else {
                        0.0
                    },
                )
                .map_err(|source| MacroError::AutoGuiError { source });
        }

        ////////////////////////////////////
        // VARIABLES
        ////////////////////////////////////

        if command.eq_ignore_ascii_case("set") {
            Self::minimum_argument_count(command.clone(), &args, 2, "2")?;

            if let Token::Variable(var) = &args[0] {
                self.variables
                    .insert(var.clone(), self.parse_token(&args[1])?);
                return Ok(());
            }

            return Err(MacroError::ExpectedToken("variable"));
        }

        if command.eq_ignore_ascii_case("increment") {
            Self::minimum_argument_count(command.clone(), &args, 1, "1")?;

            if let Token::Variable(var) = &args[0] {
                self.variables
                    .insert(var.clone(), Value::Int(self.parse_int(&args[0])? + 1));
                return Ok(());
            }

            return Err(MacroError::ExpectedToken("variable"));
        }

        if command.eq_ignore_ascii_case("decrement") {
            Self::minimum_argument_count(command.clone(), &args, 1, "1")?;

            if let Token::Variable(var) = &args[0] {
                self.variables
                    .insert(var.clone(), Value::Int(self.parse_int(&args[0])? - 1));
                return Ok(());
            }

            return Err(MacroError::ExpectedToken("variable"));
        }

        if command.eq_ignore_ascii_case("print") {
            for i in args {
                print!("{} ", self.parse_token(i)?.to_string())
            }
            print!("\n");
            return Ok(());
        }

        ////////////////////////////////////
        // CONTROL
        ////////////////////////////////////

        if command.eq_ignore_ascii_case("wait") {
            Self::minimum_argument_count(command.clone(), &args, 1, "1")?;

            thread::sleep(Duration::from_secs_f32(self.parse_float(&args[0])?));

            return Ok(());
        }

        if command.eq_ignore_ascii_case("loop") {
            if args.len() > 0 {
                self.loop_stack
                    .push((self.line as usize, self.parse_uint(&args[0])? as i32));
            } else {
                self.loop_stack.push((self.line as usize, -1));
            }

            return Ok(());
        }

        if command.eq_ignore_ascii_case("end") {
            if self.loop_stack.len() > 0 {
                let last_idx = self.loop_stack.len() - 1;

                if self.loop_stack[last_idx].1 < 0 {
                    self.line = self.loop_stack[last_idx].0 as i32;
                    return Ok(());
                }

                self.loop_stack[last_idx].1 -= 1;

                if self.loop_stack[last_idx].1 == 0 {
                    self.loop_stack.pop();
                } else {
                    self.line = self.loop_stack[last_idx].0 as i32;
                }
            }

            return Ok(());
        }

        if command.eq_ignore_ascii_case("exit") {
            self.line = -1;
            return Ok(());
        }

        // KEYBOARD

        if command.eq_ignore_ascii_case("write") {
            Self::minimum_argument_count(command.clone(), &args, 1, "1")?;

            let text = self.parse_string(&args[0])?;

            return self
                .rag
                .keyboard_input(&text)
                .map_err(|source| MacroError::AutoGuiError { source });
        }

        if command.eq_ignore_ascii_case("press") {
            Self::minimum_argument_count(command.clone(), &args, 1, "1")?;

            let text = self.parse_string(&args[0])?;

            return self
                .rag
                .keyboard_command(&text)
                .map_err(|source| MacroError::AutoGuiError { source });
        }

        if command.eq_ignore_ascii_case("hold") {
            Self::minimum_argument_count(command.clone(), &args, 2, "2")?;

            let key = self.parse_string(&args[0])?;

            self.rag
                .key_down(&key)
                .map_err(|source| MacroError::AutoGuiError { source })?;

            thread::sleep(Duration::from_secs_f32(self.parse_float(&args[1])?));

            return self
                .rag
                .key_up(&key)
                .map_err(|source| MacroError::AutoGuiError { source });
        }

        if command.eq_ignore_ascii_case("keydown") {
            Self::minimum_argument_count(command.clone(), &args, 1, "1")?;

            let button = self.parse_string(&args[0])?;

            return self
                .rag
                .key_down(&button)
                .map_err(|source| MacroError::AutoGuiError { source });
        }

        if command.eq_ignore_ascii_case("keyup") {
            Self::minimum_argument_count(command.clone(), &args, 1, "1")?;

            let button = self.parse_string(&args[0])?;

            return self
                .rag
                .key_up(&button)
                .map_err(|source| MacroError::AutoGuiError { source });
        }

        if command.eq_ignore_ascii_case("find") {
            Self::minimum_argument_count(command.clone(), &args, 3, "3")?;

            let file_path = &self.parse_string(&args[0])?;

            let precision = self.parse_float(&args[1])?;

            let x_var = if let Token::Variable(var) = &args[2] {
                var
            } else {
                return Err(MacroError::ExpectedToken("variable"));
            };

            let y_var = if let Token::Variable(var) = &args[3] {
                var
            } else {
                return Err(MacroError::ExpectedToken("variable"));
            };

            println!("{}", format!("Trying to find `{file_path}` (precision: {precision}).\n\tGoing to assign to: ({x_var}, {y_var})").black());

            if let Err(err) = self.rag.prepare_template_from_file(
                file_path,                         // template_path: &str path to the image file on disk
                None, // region: Option<(u32, u32, u32, u32)>  region of monitor to search (x, y, width, height)
                rustautogui::MatchMode::Segmented, // match_mode: rustautogui::MatchMode search mode (Segmented or FFT)
            ) {
                println!("Error: {}", format!("{}", err).red());
            }

            if let Ok(Some(locations)) = self.rag.find_image_on_screen(precision) {
                self.variables
                    .insert(x_var.clone(), Value::Int(locations[0].0 as i32));
                self.variables
                    .insert(y_var.clone(), Value::Int(locations[0].1 as i32));
            }

            return Ok(());
        }

        Err(MacroError::InvalidCommand(command))
    }
}

// PARSING
#[allow(unused)]
impl MacroHandler {
    pub fn parse_token(&self, token: &Token) -> Result<Value, MacroError> {
        match token {
            &Token::Int(x) => Ok(Value::Int(x)),
            &Token::Float(x) => Ok(Value::Float(x)),
            Token::String(x) => Ok(Value::String(x.clone())),
            &Token::Bool(x) => Ok(Value::Bool(x)),
            &Token::NIL => Ok(Value::NIL),

            Token::Variable(k) => self
                .variables
                .get(k)
                .ok_or(MacroError::UndefinedVariable(k.clone()))
                .cloned(),

            _ => Err(MacroError::UnknownToken(token.to_string())),
        }
    }

    pub fn parse_uint(&self, token: &Token) -> Result<u32, MacroError> {
        match self.parse_token(token)? {
            Value::Int(x) => x.try_into().map_err(|_| MacroError::ExpectedUnsigned),
            Value::Float(x) => (x as i32)
                .try_into()
                .map_err(|_| MacroError::ExpectedUnsigned),

            _ => Err(MacroError::ParseIntError(token.to_string())),
        }
    }

    pub fn parse_uint_optional(&self, token: &Token) -> Result<Option<u32>, MacroError> {
        match self.parse_token(token)? {
            Value::Int(x) => Ok(Some(
                x.try_into().map_err(|_| MacroError::ExpectedUnsigned)?,
            )),
            Value::Float(x) => Ok(Some(
                (x as i32)
                    .try_into()
                    .map_err(|_| MacroError::ExpectedUnsigned)?,
            )),
            Value::NIL => Ok(None),

            _ => Err(MacroError::ParseIntError(token.to_string())),
        }
    }

    pub fn parse_int(&self, token: &Token) -> Result<i32, MacroError> {
        match self.parse_token(token)? {
            Value::Int(x) => Ok(x),
            Value::Float(x) => Ok(x as i32),

            _ => Err(MacroError::ParseIntError(token.to_string())),
        }
    }

    pub fn parse_int_optional(&self, token: &Token) -> Result<Option<i32>, MacroError> {
        match self.parse_token(token)? {
            Value::Int(x) => Ok(Some(x)),
            Value::Float(x) => Ok(Some(x as i32)),
            Value::NIL => Ok(None),

            _ => Err(MacroError::ParseIntError(token.to_string())),
        }
    }

    pub fn parse_float(&self, token: &Token) -> Result<f32, MacroError> {
        match self.parse_token(token)? {
            Value::Int(x) => Ok(x as f32),
            Value::Float(x) => Ok(x),

            _ => Err(MacroError::ParseIntError(token.to_string())),
        }
    }

    pub fn parse_float_optional(&self, token: &Token) -> Result<Option<f32>, MacroError> {
        match self.parse_token(token)? {
            Value::Int(x) => Ok(Some(x as f32)),
            Value::Float(x) => Ok(Some(x)),
            Value::NIL => Ok(None),

            _ => Err(MacroError::ParseIntError(token.to_string())),
        }
    }

    pub fn parse_string(&self, token: &Token) -> Result<String, MacroError> {
        match self.parse_token(token)? {
            Value::Int(x) => Ok(x.to_string()),
            Value::Float(x) => Ok(x.to_string()),
            Value::String(x) => Ok(x),
            Value::Bool(x) => Ok(x.to_string()),

            _ => Err(MacroError::ParseStringError(token.to_string())),
        }
    }

    pub fn parse_string_optional(&self, token: &Token) -> Result<Option<String>, MacroError> {
        match self.parse_token(token)? {
            Value::Int(x) => Ok(Some(x.to_string())),
            Value::Float(x) => Ok(Some(x.to_string())),
            Value::String(x) => Ok(Some(x)),
            Value::Bool(x) => Ok(Some(x.to_string())),
            Value::NIL => Ok(None),

            _ => Err(MacroError::ParseStringError(token.to_string())),
        }
    }
}
