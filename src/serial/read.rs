use taskio::Poll;

pub trait Read<W> {
    type Error;

    fn read(&mut self) -> Poll<Result<W, Self::Error>>;
}
