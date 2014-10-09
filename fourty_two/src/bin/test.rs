#![feature(phase)]
#[phase(plugin, link)]
//note: the name of the crate is unrelated with the macro's name
extern crate fourty_two;

fn main() {
    println!("oh a int {}", fourty_two! {});

}
