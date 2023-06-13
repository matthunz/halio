use core::marker::PhantomData;
use embedded_hal::serial as hal;

mod read;
pub use read::Read;
use taskio::Poll;

mod write;
pub use write::Write;

pub fn from_hal<T, W>(hal: T) -> FromHal<T, W> {
    FromHal { hal, _marker: PhantomData }
}

pub struct FromHal<T, W> {
    hal: T,
    _marker: PhantomData<W>,
}

impl<W, T> Read<W> for FromHal<T, W>
where
    T: hal::Read<W>,
{
    type Error = T::Error;

    fn read(&mut self) -> Poll<Result<W, Self::Error>> {
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
