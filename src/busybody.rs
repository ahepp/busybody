type Result<T> = core::result::Result<T, Box<dyn std::error::Error>>;

pub fn run(args: Vec<String>) -> Result<()> {
    parse(args.first())
}

fn parse(maybe: Option<&String>) -> Result<()> {
    match maybe {
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

    fn take_error(result: Result<()>) {
        match result {
            Err(_) => {}
            _ => panic!(),
        }
    }

    #[test]
    fn run_returns() {
        let _ = run(vec![]);
    }

    #[test]
    fn run_no_args_returns_err() {
        take_error(run(vec![]));
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
        take_error(run(vec!["gibberish".to_string()]));
    }
}
