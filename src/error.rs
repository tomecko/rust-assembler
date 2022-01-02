#[derive(thiserror::Error, Debug)]
pub enum Foo {
    #[error("bar")]
    Bar,
}

#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("invalid register index: {0}")]
    InvalidRegisterIndex(i64),
    #[error("missing input")]
    MissingInput,
    #[error("missing mnemonic")]
    MissingMnemonic,
    #[error("unknown mnemonic: {0}")]
    UnknownMnemonic(String),
    #[error("wrong number of arguments, expected: {expected}, received: {received}")]
    WrongNumberOfArguments { received: usize, expected: usize },
    #[error("IO error: {0}")]
    IOError(#[from] std::io::Error),
    #[error("generic error: {0}")]
    GenericError(#[from] anyhow::Error),
}
