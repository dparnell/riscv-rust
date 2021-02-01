/// Allows the host application to provide services to the emulation
pub trait Host {
    fn load(&self, address: u64) -> u8;
    fn store(&self, address: u64, value: u8);
}

pub struct DummyHost {}

impl DummyHost {
    pub fn new() -> Self {
        DummyHost{}
    }
}

impl Host for DummyHost {
    fn load(&self, _address: u64) -> u8 {
        0
    }

    fn store(&self, _address: u64, _value: u8) {
        // do nothing
    }
}