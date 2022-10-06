type Code = i32;

pub const ACCOUNT_ALREADY_EXISTS: Code = 1;
pub const INVALID_CREDENTIAL: Code = 2;
pub const INVALID_TOKENER_KEY: Code = 3;
pub const FAILED_TO_SIGN_CLAIM: Code = 4;
pub const FAILED_TO_VERIFY_TOKEN: Code = 5;
pub const FAILED_TO_CONNECT_TO_DATABASE: Code = 6;
pub const FAILED_TO_GET_DATABASE_CONNECTION: Code = 7;
pub const FAILED_TO_LOAD_RECORD: Code = 8;
pub const FAILED_TO_HASH_PASSWORD: Code = 9;
pub const ACCOUNT_NOT_EXISTS: Code = 10;
pub const FAILED_TO_INSERT_ACCOUNT: Code = 11;
pub const INVALID_DATABASE_URI: Code = 12;
pub const INVALID_DATABASE_OPTIONS: Code = 13;

#[derive(Debug)]
pub struct Error {
    pub msg: String,
    pub code: Code,
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
