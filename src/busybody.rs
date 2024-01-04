type Result<T> = core::result::Result<T, Box<dyn std::error::Error>>;

pub fn run(args: Vec<String>) -> Result<()> {
    match args.first() {
        Some(name) => match name.as_str() {
            _ => Ok(()),
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
}
