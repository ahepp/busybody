use crate::busybody::Result;

pub fn yes() -> Result<()> {
    do_yes(&mut std::io::stdout())
}

fn do_yes(writer: &mut dyn std::io::Write) -> Result<()> {
    Ok(writer.write_all(b"y\n")?)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn yes_returns() {
        let _ = yes();
    }

    #[test]
    fn yes_returns_ok() {
        match yes() {
            Ok(_) => {}
            _ => panic!(),
        }
    }

    #[test]
    fn do_yes_says_yes_once() {
        let mut writer = Vec::new();
        do_yes(&mut writer).unwrap();
        assert_eq!(writer, b"y\n");
    }
}
