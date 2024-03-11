use rust_grep::config::config::Config;
use rust_grep::search::search_builder::SearchBuilder;

use std::env;
use std::process;

fn main() {
    let config = Config::build(env::args()).unwrap_or_else(|err| {
        println!("Problem parsing arguments: {err}");
        process::exit(1);
    });

    let mut builder = SearchBuilder::new(config);

    if let Err(e) = builder.build() {
        print!("Application error: {e}");
        process::exit(1);
    };

    if let Err(e) = builder::run() {
        print!("Application error: {e}");
        process::exit(1);
    };
}
