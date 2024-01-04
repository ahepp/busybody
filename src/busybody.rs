type Result<T> = core::result::Result<T, Box<dyn std::error::Error>>;

pub fn run(args: Vec<String>) -> Result<()> {
    match args.first() {
        Some(name) => match name.as_str() {
            "yes" => Ok(()),
            _ => Err("failed to parse name".into()),
        },
        None => Err("called with no name".into()),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn run_returns() {
        let _ = run(vec![]);
    }

    #[test]
    fn run_no_args_returns_err() {
        match run(vec![]) {
            Err(_) => {}
            _ => panic!(),
        }
    }

    #[test]
    fn run_yes_returns_ok() {
        match run(vec!["yes".to_string()]) {
            Ok(_) => {}
            _ => panic!(),
        }
    }

    #[test]
    fn run_gibberish_returns_err() {
        match run(vec!["gibberish".to_string()]) {
            Err(_) => {}
            _ => panic!(),
        }
    }
}
