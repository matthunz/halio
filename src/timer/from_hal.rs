use super::CountDown;
use core::pin::Pin;
use embedded_hal::timer as hal;
use taskio::Poll;

pub fn from_hal<C>(hal: C) -> FromHal<C> {
    FromHal { hal }
}

pub struct FromHal<C> {
    hal: C,
}

impl<C> CountDown for FromHal<C>
where
    C: hal::CountDown + Unpin,
{
    type Time = C::Time;

    fn start<T>(&mut self, count: T)
    where
        T: Into<Self::Time>,
    {
        self.hal.start(count)
    }

    fn poll_wait(mut self: Pin<&mut Self>) -> Poll<()> {
        match self.hal.wait() {
            Ok(()) => Poll::Ready(()),
            Err(nb::Error::Other(_void)) => unreachable!(),
            Err(nb::Error::WouldBlock) => Poll::Pending,
        }
    }
}
