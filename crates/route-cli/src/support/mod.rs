//! Shared helpers peeled from `main` — domain-grouped, not one global soup.
//! Prefer adding here over growing `main.rs`.
pub(crate) mod gates;
pub(crate) mod misc;
pub(crate) mod pavement;
pub(crate) mod print;
pub(crate) mod tier;
pub(crate) use gates::*;
pub(crate) use misc::*;
pub(crate) use pavement::*;
pub(crate) use print::*;
pub(crate) use tier::*;
