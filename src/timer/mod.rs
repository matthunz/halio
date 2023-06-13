use std::convert::Infallible;
use taskio::Poll;

#[cfg(feature = "embedded-hal")]
mod from_hal;
#[cfg(feature = "embedded-hal")]
pub use from_hal::{from_hal, FromHal};

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
    fn wait(&mut self) -> Poll<Result<(), Infallible>>;
}
