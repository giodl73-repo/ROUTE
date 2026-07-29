#![allow(unused_imports)]
//! Shared helpers peeled from `main`, grouped by domain.
pub(crate) mod gates;
pub(crate) mod misc;
pub(crate) mod network;
pub(crate) mod optimizer;
pub(crate) mod pavement;
pub(crate) mod print;
pub(crate) mod tier;
pub(crate) use gates::*;
pub(crate) use misc::*;
pub(crate) use network::*;
pub(crate) use optimizer::*;
pub(crate) use pavement::*;
pub(crate) use print::*;
pub(crate) use tier::*;
