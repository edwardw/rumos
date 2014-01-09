#[crate_id="std"];
#[crate_type="rlib"];
#[no_std];
#[feature(globs)];

extern mod core;

pub mod prelude;
pub mod stdio;
pub mod term;
