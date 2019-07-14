#[derive(Debug)]
pub enum TransportError {
    Io(std::io::Error),
    Codec(kafka_api::CodecError),
    TooLarge,
}

impl TransportError {
    pub fn broken_pipe() -> Self {
        TransportError::Io(futures::io::ErrorKind::BrokenPipe.into())
    }
}
