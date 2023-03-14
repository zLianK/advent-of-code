use std::io;

pub struct Reader {}

impl Reader {
    // Read a line from stdin and returns it
    pub fn read_line() -> Result<String, String> {
        let mut buffer = String::new();
        match io::stdin().read_line(&mut buffer) {
            Ok(_) => Ok(buffer.trim().to_string()),
            Err(error) => Err(error.to_string()),
        }
    }
}
