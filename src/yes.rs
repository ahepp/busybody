use crate::busybody::Result;

pub fn yes() -> Result<()> {
    do_yes(&mut std::io::stdout())
}

fn do_yes(writer: &mut dyn std::io::Write) -> Result<()> {
    loop {
        say_yes(writer)?
    }
}

fn say_yes(writer: &mut dyn std::io::Write) -> Result<()> {
    Ok(writer.write_all(b"y\n")?)
}

#[cfg(test)]
mod tests {
    use super::*;

    use std::io::Cursor;

    #[test]
    fn say_yes_says_yes_once() {
        let mut writer = Vec::new();
        say_yes(&mut writer).unwrap();
        assert_eq!(writer, b"y\n");
    }

    #[test]
    fn do_yes_returns_err_when_buffer_full() {
        let mut writer = Cursor::new([0; 100]);
        match do_yes(&mut writer) {
            Err(_) => {}
            _ => panic!(),
        }
    }
}
