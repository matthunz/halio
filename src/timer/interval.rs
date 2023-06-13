use super::CountDown;
use core::pin::Pin;
use taskio::{Poll, Stream};

pub struct Interval<C, T> {
    count_down: C,
    count: Option<T>,
}

impl<C, T> Interval<C, T> {
    pub const fn new(count_down: C, count: T) -> Self {
        Self {
            count_down,
            count: Some(count),
        }
    }
}

impl<C, T> Stream for Interval<C, T>
where
    C: CountDown + Unpin,
    T: Into<C::Time> + Unpin,
{
    type Item = ();

    fn poll_next(mut self: Pin<&mut Self>) -> Poll<Option<Self::Item>> {
        if let Some(count) = self.count.take() {
            self.count_down.start(count);
        }

        (&mut self.count_down).poll_wait_unpin().map(Some)
    }
}
