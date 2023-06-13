use core::pin::Pin;
use taskio::Poll;

#[cfg(feature = "embedded-hal")]
mod from_hal;
#[cfg(feature = "embedded-hal")]
pub use from_hal::{from_hal, FromHal};

mod wait;
pub use wait::Wait;

pub trait CountDown {
    /// The unit of time used by this timer
    type Time;

    /// Starts a new count down
    fn start<T>(&mut self, count: T)
    where
        T: Into<Self::Time>;

    /// Non-blockingly "waits" until the count down finishes
    ///
    /// # Contract
    ///
    /// - If `Self: Periodic`, the timer will start a new count down right after the last one
    /// finishes.
    /// - Otherwise the behavior of calling `wait` after the last call returned `Ok` is UNSPECIFIED.
    /// Implementers are suggested to panic on this scenario to signal a programmer error.
    fn poll_wait(self: Pin<&mut Self>) -> Poll<()>;

    fn poll_wait_unpin(mut self) -> Poll<()>
    where
        Self: Sized + Unpin,
    {
        Pin::new(&mut self).poll_wait()
    }

    fn wait<T>(self, count: T) -> Wait<Self, T>
    where
        Self: Sized + Unpin,
        T: Into<Self::Time>,
    {
        Wait::new(self, count)
    }
}

impl<C> CountDown for &mut C
where
    C: ?Sized + CountDown + Unpin,
{
    type Time = C::Time;

    fn start<T>(&mut self, count: T)
    where
        T: Into<Self::Time>,
    {
        (&mut **self).start(count)
    }

    fn poll_wait(mut self: Pin<&mut Self>) -> Poll<()> {
        Pin::new(&mut **self).poll_wait()
    }
}
