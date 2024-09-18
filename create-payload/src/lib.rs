// Find all our documentation at https://docs.near.org
use near_sdk::{log, near};
use omni_transaction::evm::evm_transaction::EVMTransaction;
use omni_transaction::evm::utils::parse_eth_address;
use omni_transaction::transaction_builder::TransactionBuilder;
use omni_transaction::transaction_builder::TxBuilder;
use omni_transaction::types::EVM;

// Define the contract structure
#[near(contract_state)]
pub struct Contract {}

// Define the default, which automatically initializes the contract
impl Default for Contract {
    fn default() -> Self {
        Self{}
    }
}

// Implement the contract structure
#[near]
impl Contract {
    // Public method - returns the greeting saved, defaulting to DEFAULT_GREETING
    pub fn create_payload(&self) -> Vec<u8>{
        let evm_tx = TransactionBuilder::new::<EVM>()
            .nonce(2)
            .to(parse_eth_address("e0f3b7e68151e9306727104973752a415c2bcbeb"))
            .value(5000000000000000)
            .input(vec![])
            .max_priority_fee_per_gas(21763469)
            .max_fee_per_gas(25919936145)
            .gas_limit(50000)
            .chain_id(11155111)
            .build();
        

        evm_tx.build_for_signing()
    }
}

/* 
2,
242,
131,
170,
54,
167,
2,
132,
1,
76,
21,
141,
133,
6,
8,
242,
214,
145,
130,
195,
80,
148,
224,
243,
183,
230,
129,
81,
233,
48,
103,
39,
16,
73,
115,
117,
42,
65,
92,
43,
203,
235,
135,
17,
195,
121,
55,
224,
128,
0,
128,
192
 */