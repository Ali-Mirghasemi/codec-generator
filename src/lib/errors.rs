

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct Error {
    pub kind:           ErrorKind,
    pub message:        Option<String>,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum ErrorKind {
    InvalidByteOrder,
}

impl Error {

    pub fn new(kind: ErrorKind, msg: &str) -> Error {
        Self { 
            kind, 
            message: Some(msg.to_owned()),
        }
    }
}



