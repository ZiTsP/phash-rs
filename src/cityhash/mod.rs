pub struct Cityhash {
}

mod cityhash32;

pub use Hash16;
pub use Hash32;
pub use Hash64;
pub use compress;


impl Hash16 for Cityhash {
    fn to_16bit(text: &str) -> u16 {
        compress::hash32_to_hash16((cityhash32::to_hash(text)))
    }
}

impl Hash32 for Cityhash {
    fn to_32bit(text: &str) -> u32 {
        cityhash32::to_hash(text)
    }
}
