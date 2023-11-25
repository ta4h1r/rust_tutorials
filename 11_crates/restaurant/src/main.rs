// Hide warnings
#![allow(unused_variables)]
#![allow(dead_code)]
#![allow(unused_assignments)]
#![allow(unused_imports)]

use std::collections::HashMap;
use std::fmt::Result;
use std::io::Result as IoResult;   // type alias

fn main() {
    let mut map = HashMap::new();
    map.insert(1, 2);
}