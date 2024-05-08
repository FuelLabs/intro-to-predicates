 /* ANCHOR: all */ predicate;

use std::{logging::log,};

configurable {
    SECRET_NUMBER: u64 = 777,
}

fn main() -> bool {
    log(SECRET_NUMBER);
    return true;
} /* ANCHOR_END: all */ 
