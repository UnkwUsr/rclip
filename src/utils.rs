use std::hash::Hasher;

pub fn get_hash<H: Hasher>(msg: &[u8], mut hasher: H) -> u64 {
    hasher.write(msg);
    hasher.finish()
}
