#[crate_id="arch"];
#[crate_type="rlib"];
#[no_std];
#[feature(asm, globs)];

extern mod core;
extern mod std;

pub mod cpu;
pub mod drivers;
