use std::collections::{BTreeMap, HashMap, HashSet};

use blockchaintree::transaction::Transactionable;

#[derive(Default)]
pub struct TxPool {
    transactions: HashMap<[u8; 32], Box<dyn Transactionable>>,
    time_reference: BTreeMap<u64, HashSet<[u8; 32]>>,
}

impl TxPool {
    pub fn new() -> Self {
        Self {
            transactions: HashMap::new(),
            time_reference: BTreeMap::new(),
        }
    }
    pub fn add_transaction(&mut self, transaction: Box<dyn Transactionable>) -> bool {
        let transaction_hash = transaction.hash();
        match self.transactions.entry(transaction_hash) {
            std::collections::hash_map::Entry::Occupied(_) => false,
            std::collections::hash_map::Entry::Vacant(entry) => {
                self.time_reference
                    .entry(transaction.get_timestamp())
                    .or_insert_with(|| {
                        let mut to_insert = HashSet::new();
                        to_insert.insert(transaction_hash);
                        to_insert
                    });
                entry.insert(transaction);

                true
            }
        }
    }
}
