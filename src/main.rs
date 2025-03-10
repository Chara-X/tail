use std::{env, io};
fn main() {
    tail::File::open(&env::args().last().unwrap())
        .unwrap()
        .tail(io::stdout())
        .unwrap();
}
