use taskio::Poll;

pub trait Write<W> {
    type Error;

    fn write(&mut self, word: W) -> Poll<Result<(), Self::Error>>;

    fn flush(&mut self) -> Poll<Result<(), Self::Error>>;
}
