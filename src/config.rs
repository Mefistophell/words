#[derive(Debug)]
pub struct Config {
    pub action: String,
    pub language: String,
}

impl Config {
    pub fn new<T: Iterator<Item=String>>(mut args: T) -> Result<Config, &'static str> {
        args.next();

        let language = match args.next() {
            Some(arg) => arg,
            None => return Err("Didn't get the language"),
        };

        let action = match args.next() {
            Some(arg) => arg,
            None => return Err("Didn't get the action"),
        };

        Ok(Config { action, language })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::env;

    #[test]
    fn config_test() {
        let action = String::from("add");
        let language = String::from("sv");

        let args2 = env::args().chain(vec![language, action]);
        let config = Config::new(args2).unwrap();
        assert_eq!(config.action, "add");
        assert_eq!(config.language, "sv");
    }
}
