//! Command handlers — domain-grouped. Dispatch stays in `run_cli`.
//!
//! Layout: see `docs/dev/cli-layout.md`.
//! Contract: each leaf is `pub(crate) fn run(ctx, ...fields) -> Result<()>`.
//! Exemplar: [`core::build`].
pub(crate) mod ctx;
pub(crate) mod analysis;
pub(crate) mod core;
pub(crate) mod data;
pub(crate) mod game;
pub(crate) mod governance;
pub(crate) mod map;
pub(crate) mod network;
pub(crate) mod optimizer;
pub(crate) mod pavement;
pub(crate) mod standards;
pub(crate) mod stop;
pub(crate) mod t1;
pub(crate) mod t2;
pub(crate) mod t3;
pub(crate) mod t4;
