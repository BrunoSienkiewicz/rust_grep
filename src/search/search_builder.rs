use super::search_functions::{basic_search, fixed_string_search, threaded_search};
use crate::config::config::Config;
use std::error::Error;

type SearchFn<'a> = fn(&str, &'a str) -> Vec<&'a str>;

pub struct SearchBuilder<'a> {
    config: Config,
    basic_search: SearchFn<'a>,
    threaded_search: SearchFn<'a>,
    fixed_string_search: SearchFn<'a>,
}

impl SearchBuilder<'_> {
    pub fn new(config: Config) -> SearchBuilder {
        SearchBuilder {
            config: Config,
            basic_search,
            threaded_search,
            fixed_string_search,
        }
    }
}

impl SearchBuilder<'_> {
    pub fn build(&self) -> Result<(), Box<dyn Error>> {
        Ok(())
    }
}

impl SearchBuilder<'_> {
    pub fn run(&self) -> Result<(), Box<dyn Error>> {
        Ok(())
    }
}
