use std::fmt;
use std::error::Error;

use quote;

pub type MacroResult<T> = Result<T, MacroError>;

#[derive(Debug, Clone)]
pub enum MacroError {
    BodyError(quote::Tokens, String),
    FieldError(quote::Tokens, String),
    TypeError(quote::Tokens, String),
    Unaccessible,
}

impl fmt::Display for MacroError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            MacroError::BodyError(ref t, ref s) => {
                write!(f, "BodyError:\n\t{}\n because:\n \t{}", t, s)
            }
            MacroError::FieldError(ref t, ref s) => {
                write!(f, "FieldError:\n\t{}\n because:\n \t{}. Must be Scalar of Fixed Array with the maximum length of 4",
                       t,
                       s)
            }
            MacroError::TypeError(ref t, ref s) => {
                write!(f, "TypeError:\n\t{}\n because:\n \t{}. Is not an valid type.",
                       t,
                       s)
            }
            MacroError::Unaccessible => write!(f, "This path should have been unaccessible"),
        }
    }
}

impl Error for MacroError {
    fn description(&self) -> &str {
        match *self {
            MacroError::BodyError(_, _) => "Errors involving the whole structure",
            MacroError::FieldError(_, _) => "Errors involving the fields of a structure or enum",
            MacroError::TypeError(_, _) => "Errors involving the type conversion",
            MacroError::Unaccessible => "Errors from what should be impossible paths",
        }
    }

    fn cause(&self) -> Option<&Error> {
        match self {
            _ => None,
        }
    }
}
