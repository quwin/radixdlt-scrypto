use sbor::rust::cell::{Ref, RefMut};
use sbor::rust::collections::*;
use sbor::rust::vec::Vec;
use scrypto::values::ScryptoValue;
use crate::engine::{AddressPath, InMemoryChildren, REValue};

#[derive(Debug)]
pub struct PreCommittedKeyValueStore {
    pub store: HashMap<Vec<u8>, ScryptoValue>,
    pub children: InMemoryChildren,
}

impl PreCommittedKeyValueStore {
    pub fn new() -> Self {
        PreCommittedKeyValueStore {
            store: HashMap::new(),
            children: InMemoryChildren::new(),
        }
    }

    pub unsafe fn get_child(&self, path: &[AddressPath]) -> Ref<REValue> {
        self.children.get_child(path)
    }

    pub unsafe fn get_child_mut(&mut self, path: &[AddressPath]) -> RefMut<REValue> {
        self.children.get_child_mut(path)
    }

    pub fn all_descendants(&self) -> Vec<AddressPath> {
        self.children.all_descendants()
    }

    pub fn put(&mut self, key: Vec<u8>, value: ScryptoValue) {
        self.store.insert(key, value);
    }

    pub fn get(&self, key: &[u8]) -> Option<ScryptoValue> {
        self.store.get(key).cloned()
    }
}
