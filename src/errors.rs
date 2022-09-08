type Code = i32;

pub const ACCOUNT_ALREADY_EXISTS: Code = 1;
pub const INVALID_CREDENTIAL: Code = 2;

pub struct Error {
    msg: String,
    code: Code,
}

impl Error {
    pub fn new(msg: &str, code: Code) -> Self {
        Self {
            msg: msg.to_owned(),
            code,
        }
    }
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.msg)
    }
}
