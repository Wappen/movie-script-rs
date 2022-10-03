use std::error::Error;
use std::fmt::{Debug, Display, Formatter};
use std::io;
use std::io::Write;
use std::str::FromStr;

pub enum InputError<T: FromStr> where T::Err: Error {
    FalsePredicate,
    Parse(T::Err),
}

impl<T: FromStr> Display for InputError<T> where T::Err: Error {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match *self {
            InputError::FalsePredicate =>
                write!(f, "input predicate returned false"),
            InputError::Parse(..) =>
                write!(f, "the input could not be parsed into the desired type"),
        }
    }
}

impl<T: FromStr> Debug for InputError<T> where T::Err: Error {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match *self {
            InputError::FalsePredicate => write!(f, "FalsePredicate"),
            InputError::Parse(_) => write!(f, "Parse")
        }
    }
}

impl<T: FromStr> Error for InputError<T> where T::Err: Error {}

pub fn get_input<T, P, F>(msg: &str, pred: P, on_err: F) -> T where
    T: FromStr,
    T::Err: Error,
    P: Fn(&T) -> bool,
    F: Fn(&InputError<T>) {
    loop {
        let mut buffer = String::new();

        print!("{}", msg);
        io::stdout().flush().unwrap();
        let _n = io::stdin().read_line(&mut buffer);

        match buffer.trim().parse::<T>() {
            Ok(input) => {
                if pred(&input) {
                    return input;
                } else {
                    on_err(&InputError::FalsePredicate)
                }
            }
            Err(e) => {
                on_err(&InputError::Parse(e))
            }
        }
    }
}