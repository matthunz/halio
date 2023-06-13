use super::CountDown;
use core::pin::Pin;
use taskio::{Poll, Task};

pub struct Wait<C, T> {
    count_down: C,
    count: Option<T>,
}

impl<C, T> Wait<C, T> {
    pub const fn new(count_down: C, count: T) -> Self {
        Self {
            count_down,
            count: Some(count),
        }
    }
}

impl<C, T> Task for Wait<C, T>
where
    C: CountDown + Unpin,
    T: Into<C::Time> + Unpin,
{
    type Output = ();

    fn poll(mut self: Pin<&mut Self>) -> Poll<Self::Output> {
        if let Some(count) = self.count.take() {
            self.count_down.start(count);
        }

        (&mut self.count_down).poll_wait_unpin()
    }
}
