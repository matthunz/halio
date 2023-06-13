use super::Reader;
use core::pin::Pin;
use taskio::Poll;

pub trait Read<W> {
    type Error;

    fn poll_read(self: Pin<&mut Self>) -> Poll<Result<W, Self::Error>>;

    fn reader(self) -> Reader<Self>
    where
        Self: Sized + Read<u8> + Unpin,
    {
        Reader::new(self)
    }
}
