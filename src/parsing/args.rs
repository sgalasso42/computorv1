use clap::{Arg, App};

#[derive(Debug, PartialEq)]
pub struct Config {
    pub equation: String,
}

impl Config {
    pub fn new() -> Self {
        let matches = App::new("computorv1")
            .version("0.1.0")
            .author("Simon Galasso <simon.galasso@hotmail.fr>")
            .about("Solve a polynomial equation")
            .arg(Arg::with_name("equation")
                .required(true)
                .help("the formatted equation"))
            .get_matches();
        return Self {
            equation: matches.value_of("equation").unwrap().to_string()
        }
    }
}