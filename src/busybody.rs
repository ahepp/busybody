type Result<T> = core::result::Result<T, Box<dyn std::error::Error>>;

pub fn run() -> Result<()> {
    Err("failed to run".into())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn run_returns() {
        let _ = run();
    }

    #[test]
    fn run_returns_err() {
        match run() {
            Err(_) => {}
            _ => panic!(),
        }
    }
}
