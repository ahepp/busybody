use crate::yes;

type Result<T> = core::result::Result<T, Box<dyn std::error::Error>>;

enum Cmd {
    Yes,
}

pub fn run(args: Vec<String>) -> Result<()> {
    match parse(args.first())? {
        Cmd::Yes => yes::yes(),
    }
    Ok(())
}

fn parse(maybe: Option<&String>) -> Result<Cmd> {
    match maybe {
        Some(name) => match name.as_str() {
            "yes" => Ok(Cmd::Yes),
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

    #[test]
    fn parse_yes_returns_yes_cmd() {
        match parse(Some(&"yes".to_string())) {
            Ok(Cmd::Yes) => {}
            _ => panic!(),
        }
    }
}
