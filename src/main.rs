use std::iter::{IntoIterator, Iterator};

use colored::{ColoredString, Colorize};

use crate::input::{get_input, InputError};
use crate::movie::Watch;

mod input;
mod movie;

fn main() {
    let movies = movie::get_movies();

    let age: i32 = {
        let age_pred = |age: &i32| *age >= 0 && *age <= 150;
        let on_error = |e: &InputError<i32>| {
            match e {
                InputError::FalsePredicate => {
                    println!("Try again! The age must be between 0 and 150.")
                }
                InputError::Parse(_) => println!("Try again! The age must be a number."),
            };
        };

        get_input("What is your age? ", age_pred, on_error)
    };

    println!();
    println!("Amazing! Here is a list of movies you may watch:");

    let mut watchable_movies: Vec<(movie::Movie, i32)> =
        movies.into_iter().filter(|(_, fsk)| age >= *fsk).collect();

    // Sort alphabetically by title
    watchable_movies.sort_by(|(m1, _), (m2, _)| m1.title.partial_cmp(&m2.title).unwrap());

    for (index, (movie, fsk)) in (&watchable_movies).iter().enumerate() {
        let colored_fsk = colorize_fsk(fsk);
        let title = movie.title.bold();
        let director = &movie.director;
        println!("{0}. {title} by {director}, FSK {colored_fsk}", index + 1);
    }

    println!();
    let index: usize = {
        let index_pred = |index: &usize| *index > 0 && *index <= watchable_movies.len();
        let on_error = |e: &InputError<usize>| match e {
            InputError::FalsePredicate => println!("Try again! The index must be in range."),
            InputError::Parse(_) => println!("Try again! The index must be a number."),
        };

        get_input(
            "Select a movie by typing in it's number: ",
            index_pred,
            on_error,
        )
    };

    let (selected_movie, _) = watchable_movies.get(index - 1).unwrap();

    println!();
    selected_movie.watch();
}

fn colorize_fsk(fsk: &i32) -> ColoredString {
    match fsk {
        0 => fsk.to_string().white(),
        6 => fsk.to_string().yellow(),
        12 => fsk.to_string().green(),
        16 => fsk.to_string().blue(),
        18 => fsk.to_string().red(),
        _ => fsk.to_string().normal(),
    }
}
