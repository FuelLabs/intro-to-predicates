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
    return true;
}
/* ANCHOR_END: all */