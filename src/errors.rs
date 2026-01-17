use rustautogui::errors::AutoGuiError;
use thiserror::Error;

#[allow(unused)]
#[derive(Error, Debug)]
pub enum MacroError {
    #[error("Unable to read macro file: {path}.")]
    UnableToReadFile {
        path: String,
        #[source]
        source: std::io::Error,
    },

	#[error("AutoGuiError: {source}")]
	AutoGuiError {
		#[source]
		source: AutoGuiError
	},

    #[error("Undefined variable `{0}`.")]
    UndefinedVariable(String),

    #[error("Unknown mouse button `{0}`. Expected string (left, middle, right).")]
    UnknownMouseButton(String),

    #[error("Expected token type `{0}`")]
    ExpectedToken(&'static str),

    #[error("Invalid Command. Cannot run `{0}` as command or function.")]
    InvalidCommand(String),

    #[error("Invalid argument count. `{0}` expects {1} arguments.")]
    InvalidArgumentCount(String, &'static str),

    #[error("Unknown token `{0}`. Cannot parse.")]
    UnknownToken(String),

    #[error("Cannot parse `{0}` to integer.")]
    ParseIntError(String),

    #[error("Cannot parse `{0}` to float.")]
    ParseFloatError(String),

    #[error("Cannot parse `{0}` to string.")]
    ParseStringError(String),

    #[error("Expected unsigned (positive) number.")]
    ExpectedUnsigned,
}
