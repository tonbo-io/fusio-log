use thiserror::Error;

#[derive(Debug, Error)]
#[error("log error")]
pub enum LogError {
    #[error("IO error: {0}")]
    IO(#[from] std::io::Error),
    #[error("S3 error: {0}")]
    S3Error(#[from] fusio::Error),
    #[error("encode error: {message}")]
    Encode { message: String },
    #[error("decode error: {message}")]
    Decode { message: String },
    #[error("recover error: bad data")]
    BadData,
    #[error("recover error: checksum does not match")]
    Checksum,
}

pub(crate) fn parse_fusio_error(err: fusio::Error) -> LogError {
    match err {
        fusio::Error::Io(error) => LogError::IO(error),
        fusio::Error::S3Error(_) => LogError::S3Error(err),
        _ => todo!(),
    }
}
