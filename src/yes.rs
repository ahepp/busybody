use crate::busybody::Result;

pub fn yes() -> Result<()> {
    Ok(())
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
}
