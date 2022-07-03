use sbor::rust::vec::Vec;
use sbor::*;
use scrypto::buffer::*;
use scrypto::crypto::*;

#[derive(Debug, Clone, Hash, TypeId, Encode, Decode, PartialEq, Eq)]
pub struct PhysicalSubstateId(pub Hash, pub u32);

#[derive(Clone, Debug, Encode, Decode, TypeId)]
pub struct Substate {
    pub value: Vec<u8>,
    pub phys_id: PhysicalSubstateId,
}

#[derive(Debug)]
pub struct SubstateIdGenerator {
    tx_hash: Hash,
    count: u32,
}

impl SubstateIdGenerator {
    pub fn new(tx_hash: Hash) -> Self {
        Self { tx_hash, count: 0 }
    }

    pub fn next(&mut self) -> PhysicalSubstateId {
        let value = self.count;
        self.count = self.count + 1;
        PhysicalSubstateId(self.tx_hash.clone(), value)
    }
}

/// A ledger stores all transactions and substates.
pub trait ReadableSubstateStore {
    fn get_substate(&self, address: &[u8]) -> Option<Substate>;
    fn get_space(&mut self, address: &[u8]) -> Option<PhysicalSubstateId>;

    fn get_decoded_substate<A: Encode, T: Decode>(&self, address: &A) -> Option<T> {
        self.get_substate(&scrypto_encode(address))
            .map(|s| scrypto_decode(&s.value).unwrap())
    }

    fn get_epoch(&self) -> u64;
}

pub trait WriteableSubstateStore {
    fn put_substate(&mut self, address: &[u8], substate: Substate);
    fn put_space(&mut self, address: &[u8], phys_id: PhysicalSubstateId);
    fn set_epoch(&mut self, epoch: u64);
}

pub trait QueryableSubstateStore {
    fn get_substates(&self, address: &[u8]) -> Vec<(Vec<u8>, Vec<u8>)>;
}
