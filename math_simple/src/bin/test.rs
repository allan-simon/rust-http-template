#![feature(phase)]
#[phase(plugin, link)]
extern crate mathexpr;

fn main() {
    println!("oh a int {}", math_expr! {one plus two});

}
