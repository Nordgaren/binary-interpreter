use std::fs;
use std::io::Cursor;
use std::path::PathBuf;
use crate::error::*;

pub struct BinaryInterpreter {
    cursor: Cursor<Vec<u8>>
}

impl BinaryInterpreter {
    pub fn new(bytes: &[u8]) -> Result<BinaryInterpreter, InterpreterError>  {
        let cursor = Cursor::new(bytes.to_owned());
        Ok(BinaryInterpreter { cursor })
    }

    pub fn from_path(path: PathBuf) -> Result<BinaryInterpreter, InterpreterError> {
        let file = fs::read(path)?;
        BinaryInterpreter::new(file.as_slice())
    }


}