
#![allow(unused)]
fn main() {
    uniffi_macros::include_scaffolding!("math");

    fn add(a: u32, b: u32) -> u32 {
        a + b
    }
}
