pub fn basic_search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let mut result = Vec::new();

    result.extend(contents.lines().filter(|line| line.contains(query)));

    result
}

pub fn threaded_search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let mut result = Vec::new();

    result.extend(contents.lines().filter(|line| line.contains(query)));

    result
}

pub fn fixed_string_search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let mut result = Vec::new();

    result.extend(contents.lines().filter(|line| line.contains(query)));

    result
}
