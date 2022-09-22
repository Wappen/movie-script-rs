use std::collections::HashMap;
use std::fs::File;
use std::hash::{Hash, Hasher};
use std::io;
use std::io::{BufRead, Read, Write};
use std::path::Path;
use std::thread::sleep;
use std::time::Duration;

static MOVIES_DIR: &str = "res/movies";
static MOVIE_LIST: &str = "res/movies/movie_list";

pub struct Movie {
    pub id: i64,
    pub title: String,
    pub director: String,
}

pub trait Watch {
    fn watch(&self);
}

impl Watch for Movie {
    fn watch(&self) {
        println!("Watching {} by {}...", self.title, self.director);

        let path = format!("{}/{:x}.tmov", MOVIES_DIR, self.id);

        let movie = match read_str(&path) {
            Ok(m) => m,
            Err(_) => {
                eprintln!("Could not load contents for movie {} at {path}", self.title);
                return;
            }
        };

        let parts: Vec<&str> = movie.split("---\n").collect();

        for part in parts.into_iter() {
            sleep(Duration::from_secs(2));
            print!("{}", part);
            io::stdout().flush().unwrap();
        }
    }
}

impl PartialEq<Self> for Movie {
    fn eq(&self, other: &Self) -> bool {
        return self.id == other.id;
    }
}

impl Eq for Movie {}

impl Hash for Movie {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.id.hash(state);
    }
}

pub fn get_movies() -> HashMap<Movie, i32> {
    let mut movies = HashMap::new();

    let lines: Vec<String> = match read_lines(MOVIE_LIST) {
        Ok(lines) => lines.filter_map(|line_str| {
            match line_str {
                Ok(str) => Some(str),
                Err(_) => {
                    eprintln!("Interrupted while reading movie list {}", MOVIE_LIST);
                    return None;
                }
            }
        }).collect(),
        Err(_) => {
            eprintln!("Could not read movie list {}", MOVIE_LIST);
            return movies;
        }
    };

    for line in lines {
        if line.is_empty() {
            continue;
        }

        let parts: Vec<&str> = line.split(",").collect();
        let id = match i64::from_str_radix(parts[0].trim(), 16) {
            Ok(i) => i,
            Err(_) => {
                eprintln!("Invalid ID found for movie!");
                continue;
            }
        };

        let title = parts[1].trim().to_string();
        let director = parts[2].trim().to_string();
        let fsk: i32 = match parts[3].trim().parse() {
            Ok(fsk) => fsk,
            Err(_) => {
                eprintln!("Invalid FSK value for movie '{}'", title);
                continue;
            }
        };

        movies.insert(Movie { id, title, director }, fsk);
    }

    return movies;
}

fn read_str<P>(filename: P) -> io::Result<String>
    where P: AsRef<Path>, {
    let file = File::open(filename)?;
    let mut out = String::new();
    io::BufReader::new(file).read_to_string(&mut out)?;
    Ok(out)
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
    where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
