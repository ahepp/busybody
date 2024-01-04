use std::path::Path;

pub type Result<T> = core::result::Result<T, Box<dyn std::error::Error>>;

pub enum Cmd {
    Yes,
}

pub fn run(
    args: Vec<String>,
    dispatch: impl Fn(Cmd) -> Result<()>,
) -> Result<()> {
    dispatch(parse(args.first())?)
}

fn parse(maybe: Option<&String>) -> Result<Cmd> {
    match maybe {
        None => Err("called with no name".into()),
        Some(name) => match Path::new(name).file_name() {
            None => Err("path has no filename".into()),
            Some(path) => match path.to_str() {
                None => Err("filename cannot be expressed as str".into()),
                Some(string) => match string {
                    "yes" => Ok(Cmd::Yes),
                    _ => Err("failed to parse name".into()),
                },
            },
        },
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

    fn null_dispatch(_: Cmd) -> Result<()> {
        Err("null dispatch".into())
    }

    #[test]
    fn run_returns() {
        let _ = run(vec![], null_dispatch);
    }

    #[test]
    fn run_no_args_returns_err() {
        take_error(run(vec![], null_dispatch));
    }

    #[test]
    fn run_gibberish_returns_err() {
        take_error(run(vec!["gibberish".to_string()], null_dispatch));
    }

    #[test]
    fn parse_yes_returns_yes_cmd() {
        match parse(Some(&"yes".to_string())) {
            Ok(Cmd::Yes) => {}
            _ => panic!(),
        }
    }

    #[test]
    fn run_yes_triggers_dispatch() {
        let dispatch = |cmd: Cmd| match cmd {
            Cmd::Yes => Ok(()),
        };
        run(vec!["yes".to_string()], dispatch).unwrap();
    }

    #[test]
    fn parse_yes_with_leading_path_returns_yes_cmd() {
        match parse(Some(&"/all/kinds/.././of/stuff/yes".to_string())) {
            Ok(Cmd::Yes) => {}
            _ => panic!(),
        }
    }
}
