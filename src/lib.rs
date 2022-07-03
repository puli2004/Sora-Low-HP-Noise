#![feature(concat_idents)]
#![feature(proc_macro_hygiene)]
#[warn(stable_features)]


mod sora;

#[skyline::main(name = "smashline_test")]
pub fn main() {
    sora::install();
}
