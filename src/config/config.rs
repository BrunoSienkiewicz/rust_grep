use super::flag_config::FlagConfig;

#[derive(Debug)]
pub struct Config {
    pub query: String,
    pub file_path: String,
    pub flag_config: FlagConfig,
}

impl Config {
    pub fn build(mut args: impl Iterator<Item = String>) -> Result<Config, &'static str> {
        args.next();

        let query = match args.next() {
            Some(arg) => arg,
            None => return Err("Query not found"),
        };

        let flag_config = match FlagConfig::build(args) {
            Ok(flag_config) => flag_config,
            Err(e) => return Err(e),
        };

        let file_path = match args.next() {
            Some(arg) => arg,
            None => return Err("File path not found"),
        };

        Ok(Config {
            query,
            file_path,
            flag_config,
        })
    }
}
