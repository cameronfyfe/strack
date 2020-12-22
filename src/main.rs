use strack;
use std::env::args;

fn main() {
    strack::run(args().skip(1).collect());
}
