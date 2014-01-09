#[crate_id="arch"];
#[crate_type="rlib"];
#[no_std];
#[feature(asm)];

extern mod core;

pub mod cpu;
pub mod drivers;
