use crate::ml::api::default_api::{init as init_ml};
use  crate::web::common::guard::init_admin;
use ic_cdk::{init, post_upgrade, pre_upgrade};


#[init]
fn init() {
    init_ml();
    init_admin();
}
#[pre_upgrade]
fn pre_upgrade() {
    ic_cdk::println!("pre_upgrade:");
}

#[post_upgrade]
fn post_upgrade() {
    ic_cdk::println!("post_upgrade:");
}
