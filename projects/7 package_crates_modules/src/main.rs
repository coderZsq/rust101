use std::collections::*;

// use std::io;
// use std::io::Write;
use std::io::{self, Write};

// use std::cmp::Ordering;
// use std::io;
// use std::{cmp::Ordering, io};

use rand::Rng;

fn main() {
    let select_number = rand::thread_rng().gen_range(1..101);
}

use std::collections::HashMap;

fn main_2() {
    let mut map = HashMap::new();
    map.insert(1, 2);
}

use crate::garden::vegetables::Asparagus;

pub mod garden;

fn main_1() {
    let plant = Asparagus {};
    println!("I'm growing {plant:?}!");
}
