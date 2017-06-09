#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
    }
}

pub trait Hash16 {
    fn to_16bit(str: &str) -> u16;
}

pub trait Hash32 {
    fn to_32bit(str: &str) -> u32;
}

pub trait Hash64 {
    fn to_64bit(str: &str) -> u64;
}

pub mod compress{
    pub fn hash64_to_hash32(i: u64) -> u32 {
        ((i >> 32) ^ (i & 0x00000000ffffffff)) as u32
    }

    pub fn hash32_to_hash16(i: u32) -> u16 {
        ((i >> 16) ^ (i & 0x0000ffff)) as u16
    }

    pub fn hash32_to_hash24(i: u32) -> u32 {
        ((i >> 8) ^ (i & 0x00ffffff))
    }
}

// pub use self::hash::traits::{Hash16, Hash32, Hash64};
// pub use self::hash::compress;
// pub mod hash;

pub use self::cityhash::Cityhash;
pub mod cityhash;
