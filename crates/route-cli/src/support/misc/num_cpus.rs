//! Helper `num_cpus`.
#[allow(unused_imports)]
use crate::*;

pub(crate) fn num_cpus() -> usize {
    std::thread::available_parallelism()
        .map(|n| n.get())
        .unwrap_or(4)
}
