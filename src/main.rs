use ic_cdk::export::Principal;
use ic_cdk_macros::{init, post_upgrade};
use crate::{auth, storage, security};

#[init]
fn init() {
    storage::initialize();
}

#[post_upgrade]
fn post_upgrade() {
    storage::load_state();
}
