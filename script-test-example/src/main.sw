script;

use std::{
    logging::log,
};

configurable {
    SECRET_NUMBER: u64 = 777
}

fn main() {
    log(SECRET_NUMBER);
}
