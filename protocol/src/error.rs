use std::io::Error;

#[derive(Debug, thiserror::Error)]
pub enum NetheriteError {
    #[error("IO error: `{0}`")]
    IOErr(#[from] Error),
    #[error("Double login - player with your address is already logged in!")]
    DoubleLogin
}