extern crate rustupolis;

use rustupolis::tuplespace::{SimpleSpace, Space};

#[test]
fn test_start() {
    println!("testing startup");
    let t_space = SimpleSpace::new();
    println!("tuple space current size: {}", t_space.len());
    println!("startup done");
}
