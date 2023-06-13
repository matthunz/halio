use super::Read;
use core::pin::Pin;
use pin_project_lite::pin_project;
use taskio::{io, ready, Poll};

pin_project! {
    pub struct Reader<R> {
        #[pin]
        read: R,
    }
}

impl<R> Reader<R> {
    pub const fn new(read: R) -> Self {
        Self { read }
    }
}

impl<R> io::Read for Reader<R>
where
    R: Read<u8>,
{
    type Error = R::Error;

    fn poll_read(self: Pin<&mut Self>, buf: &mut [u8]) -> Poll<Result<usize, Self::Error>> {
        // TODO
        let b = ready!(self.project().read.poll_read()).ok().unwrap();
        buf[0] = b;
        Poll::Ready(Ok(0))
    }
}
