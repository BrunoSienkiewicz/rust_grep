#[derive(Debug)]
pub struct FlagConfig {
    // query config
    pub case_sensitive: bool,
    pub invert_match: bool,
    pub recursive: bool,
    pub word_regexp: bool,

    // search config
    pub basic_search: bool,
    pub threaded_search: bool,
    pub fixed_string_search: bool,

    // output config
    pub count: bool,
    pub line_number: bool,
    pub only_matching: bool,
}

impl FlagConfig {
    pub fn build(mut args: impl Iterator<Item = String>) -> Result<FlagConfig, &'static str> {
        let mut flag_config = FlagConfig {
            case_sensitive: true,
            invert_match: false,
            recursive: false,
            word_regexp: false,
            basic_search: true,
            threaded_search: false,
            fixed_string_search: false,
            count: false,
            line_number: false,
            only_matching: false,
        };

        while let Some(arg) = args.next() {
            match arg.as_str() {
                "-i" => flag_config.case_sensitive = false,
                "-v" => flag_config.invert_match = true,
                "-r" => flag_config.recursive = true,
                "-w" => flag_config.word_regexp = true,
                "-F" => {
                    flag_config.basic_search = false;
                    flag_config.fixed_string_search = true;
                }
                "-n" => flag_config.line_number = true,
                "-c" => flag_config.count = true,
                "-o" => flag_config.only_matching = true,
                _ => return Err("Invalid flag"),
            }
        }

        Ok(flag_config)
    }
}
