mod args;

use std::thread;
use std::time::Duration;
use jokeapi_rs::Joke;
use clap::Parser;
use args::JokerArgs;
use enigo::{Enigo, Keyboard, Settings};

fn main() {
    let args = JokerArgs::parse();
    let out;
    let delay = args.delay;
    let mut enigo = Enigo::new(&Settings::default()).unwrap();

    match args.category.as_str() {
        "all" => out = Joke::new().fetch().joke(),
        "programming" => out = Joke::new().categories(vec!["programming".to_string()]).fetch().joke(),
        "dark" => out = Joke::new().categories(vec!["dark".to_string()]).fetch().joke(),
        "pun" => out = Joke::new().categories(vec!["pun".to_string()]).fetch().joke(),
        "misc" => out = Joke::new().categories(vec!["misc".to_string()]).fetch().joke(),
        _ => return
    }
    
    println!("{}", out);
    thread::sleep(Duration::from_secs(delay));
    let _ = enigo.text(&out);
}
