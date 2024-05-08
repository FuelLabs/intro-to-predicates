 /* ANCHOR: all */ // ANCHOR: predicate
predicate;
// ANCHOR_END: predicate

// ANCHOR: import_parent
use std::{
    b512::B512,
    constants::ZERO_B256,
    ecr::ec_recover_address,
    tx::{
        tx_id,
        tx_witness_data,
        tx_witnesses_count,
    },
};
// ANCHOR_END: import_parent

// ANCHOR: configurable
configurable {
    REQUIRED_SIGNATURES: u64 = 0,
    SIGNERS: [Address; 3] = [
        Address::from(0x0000000000000000000000000000000000000000000000000000000000000000),
        Address::from(0x0000000000000000000000000000000000000000000000000000000000000000),
        Address::from(0x0000000000000000000000000000000000000000000000000000000000000000),
    ],
}
// ANCHOR_END: configurable

// ANCHOR: signature_verification_parent
fn verify_signature(i: u64) -> u64 {
    // Discard any out of bounds signatures
    if (i >= tx_witnesses_count()) {
        return 0;
    }

    let tx_hash = tx_id();

    let mut j = 0;

    // ANCHOR: verification_loop
    while j < 3 {
        let current_signature = tx_witness_data::<B512>(j);

        let current_address = ec_recover_address(current_signature, tx_hash).unwrap();

        if current_address == SIGNERS[i] {
            return 1;
        }

        j += 1;
    }
    return 0;
    // ANCHOR_END: verification_loop
}
// ANCHOR_END: signature_verification_parent

// ANCHOR: main
fn main() -> bool {
    let mut valid_signatures = 0;

    // Verifiying each potential signature 
    valid_signatures = verify_signature(0);
    valid_signatures = valid_signatures + verify_signature(1);
    valid_signatures = valid_signatures + verify_signature(2);

    if valid_signatures >= REQUIRED_SIGNATURES {
        return true;
    }
    return false;
}
// ANCHOR_END: main /* ANCHOR_END: all */ 
