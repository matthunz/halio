use core::marker::PhantomData;
use embedded_hal::serial as hal;
use taskio::Poll;

pub trait Read<W> {
    type Error;

    fn read(&mut self) -> Poll<Result<W, Self::Error>>;
}
