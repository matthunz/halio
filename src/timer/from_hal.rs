use super::CountDown;
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
    C: hal::CountDown,
{
    type Time = C::Time;

    fn start<T>(&mut self, count: T)
    where
        T: Into<Self::Time>,
    {
        self.hal.start(count)
    }

    fn wait(&mut self) -> Poll<Result<(), std::convert::Infallible>> {
        self.hal.wait().map_err(|_| unreachable!()).into()
    }
}
