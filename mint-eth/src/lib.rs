// Find all our documentation at https://docs.near.org
use near_sdk::{log, near, AccountId, Gas, NearToken, Promise};
pub mod external;
pub use crate::external::*;
use omni_transaction::evm::evm_transaction::EVMTransaction;
use omni_transaction::evm::utils::parse_eth_address;
use omni_transaction::transaction_builder::TransactionBuilder;
use omni_transaction::transaction_builder::TxBuilder;
use omni_transaction::types::EVM;

// Define the contract structure
#[near(contract_state)]
pub struct Contract {
    mpc_contract_id: AccountId,
}

// Define the default, which automatically initializes the contract
impl Default for Contract {
    fn default() -> Self {
        let mpc_contract_id: AccountId = "v1.signer-dev.testnet".parse().unwrap();
        Self {
            mpc_contract_id,
        }
    }
}

// Implement the contract structure
#[near]
impl Contract {
    #[payable]
    pub fn get_signature(&mut self, request: external::Request) -> Promise {
        mpc_contract::ext(self.mpc_contract_id.clone())
        .with_static_gas(Gas::from_tgas(250))
        .with_attached_deposit(NearToken::from_millinear(300))
        .sign(request)
    }

    #[payable]
    pub fn get_signature_from_payload(&mut self, request: external::Request) -> Promise {
        mpc_contract::ext(self.mpc_contract_id.clone())
        .with_static_gas(Gas::from_tgas(250))
        .with_attached_deposit(NearToken::from_millinear(300))
        .sign(request)
    }

    #[payable]
    pub fn get_signature_from_tx_info(
        &mut self, 
        to_address: String, 
        max_fee_per_gas: u128,
        max_priority_fee_per_gas: u128,
        gas_limit: u128,
        chain_id: u64,
        nonce: u64,
        amount: u128,
        path: String,
        key_version: u8,
    ) -> Promise {

        let evm_tx = TransactionBuilder::new::<EVM>()
            .nonce(nonce)
            .to(parse_eth_address(to_address.as_str()))
            .value(amount)
            .input(vec![])
            .max_priority_fee_per_gas(max_priority_fee_per_gas)
            .max_fee_per_gas(max_fee_per_gas)
            .gas_limit(gas_limit)
            .chain_id(chain_id)
            .build();
    

        let payload = evm_tx.build_for_signing();

        let request:Request = Request {
            payload,
            path,
            key_version,
        };

        mpc_contract::ext(self.mpc_contract_id.clone())
        .with_static_gas(Gas::from_tgas(250))
        .with_attached_deposit(NearToken::from_millinear(300))
        .sign(request)

}

    pub fn change_mpc_contract_id(&mut self, contract_id: AccountId) {
        self.mpc_contract_id = contract_id.clone();
        log!("New contract id is '{}'", contract_id);
    }

    pub fn get_mpc_contract_id(&self) -> AccountId {
        self.mpc_contract_id.clone()
    }
}