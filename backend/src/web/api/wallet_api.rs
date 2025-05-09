use crate::web::models::wallet_model::Wallet;
use crate::web::services::wallet_service::ExtendWalletService;
use crate::web::services::wallet_service::WalletService;
use ic_cdk::update;
use  crate::web::common::guard::is_admin;

#[update(guard = "is_admin")]
fn create_wallet(data: Wallet) {
    Wallet::is_exist(data.principal_id.clone().unwrap());
}
