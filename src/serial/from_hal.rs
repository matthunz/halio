use super::{Read, Write};
use core::{marker::PhantomData, pin::Pin};
use embedded_hal::serial as hal;
use taskio::Poll;

pub fn from_hal<T, W>(hal: T) -> FromHal<T, W> {
    FromHal {
        hal,
        _marker: PhantomData,
    }
}

pub struct FromHal<T, W> {
    hal: T,
    _marker: PhantomData<W>,
}

impl<T: Unpin, W> Unpin for FromHal<T, W> {}

impl<W, T> Read<W> for FromHal<T, W>
where
    T: hal::Read<W> + Unpin,
{
    type Error = T::Error;

    fn poll_read(mut self: Pin<&mut Self>) -> Poll<Result<W, Self::Error>> {
        self.hal.read().into()
    }
}

impl<W, T> Write<W> for FromHal<T, W>
where
    T: hal::Write<W>,
{
    type Error = T::Error;

    fn write(&mut self, word: W) -> Poll<Result<(), Self::Error>> {
        self.hal.write(word).into()
    }

    fn flush(&mut self) -> Poll<Result<(), Self::Error>> {
        self.hal.flush().into()
    }
}
