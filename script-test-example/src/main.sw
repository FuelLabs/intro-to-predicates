/* ANCHOR: all */
// ANCHOR: program_type
script;
// ANCHOR_END: program_type

use std::{
    logging::log,
};

configurable {
    SECRET_NUMBER: u64 = 777
}

fn main() -> bool {
    log(SECRET_NUMBER);
    predicate
}
/* ANCHOR_END: all */