#[crate_id="extra"];
#[crate_type="rlib"];
#[no_std];
#[feature(globs)];

extern mod std;

pub mod prelude;
pub mod stdio;
pub mod term;
