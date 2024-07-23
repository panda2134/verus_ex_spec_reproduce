use crate::consts::*;

pub struct Device {
    buf: [u8; 8192]
}

impl Device {
    pub fn read(&self, out: &mut [u8; BLKSIZE]) {
    }
}
