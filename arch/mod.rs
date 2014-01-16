#[crate_id="arch"];
#[crate_type="rlib"];
#[no_std];
#[feature(asm, globs)];

extern mod std;
extern mod extra;

pub mod cpu;
pub mod drivers;
