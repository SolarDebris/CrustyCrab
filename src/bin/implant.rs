// Ignore Warnings
#![allow(unused_variables)]
#![allow(unused_mut)]
#![allow(unused_imports)]
#![allow(unused_must_use)]
#![allow(unused_assignments)]
#![allow(unreachable_code)]
#![allow(dead_code)]

use crabby_patty_formula::*;
use std::net::SocketAddr;

fn main(){
    imp_run("tcp", SocketAddr::from(([127, 0, 0, 1], 2120)));
}
