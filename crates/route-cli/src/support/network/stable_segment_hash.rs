//! Helper `stable_segment_hash`.
#[allow(unused_imports)]
use crate::*;

pub(crate) fn stable_segment_hash(value: &str) -> u64 {
    let mut hash = 0xcbf29ce484222325u64;
    for byte in value.as_bytes() {
        hash ^= u64::from(*byte);
        hash = hash.wrapping_mul(0x100000001b3);
    }
    hash
}

