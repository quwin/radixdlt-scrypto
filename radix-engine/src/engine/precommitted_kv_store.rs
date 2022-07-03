use crate::engine::{AddressPath, InMemoryChildren, REValue};
use sbor::rust::cell::{Ref, RefMut};
use sbor::rust::collections::*;
use sbor::rust::vec::Vec;
use scrypto::values::ScryptoValue;

#[derive(Debug)]
pub struct PreCommittedKeyValueStore {
    pub store: HashMap<Vec<u8>, (ScryptoValue, InMemoryChildren)>,
}

impl PreCommittedKeyValueStore {
    pub fn new() -> Self {
        PreCommittedKeyValueStore {
            store: HashMap::new(),
        }
    }

    pub unsafe fn get_child(&self, path: &[AddressPath]) -> Ref<REValue> {
        let (first, rest) = path.split_first().unwrap();
        let key = match first {
            AddressPath::Key(key) => key,
            _ => panic!("Unexpected"),
        };
        let (_, children) = self.store.get(key).unwrap();
        children.get_child(rest)
    }

    pub unsafe fn get_child_mut(&mut self, path: &[AddressPath]) -> RefMut<REValue> {
        let (first, rest) = path.split_first().unwrap();
        let key = match first {
            AddressPath::Key(key) => key,
            _ => panic!("Unexpected"),
        };
        let (_, children) = self.store.get_mut(key).unwrap();
        children.get_child_mut(rest)
    }

    pub fn all_descendants(&self) -> Vec<AddressPath> {
        let mut descendants = Vec::new();
        for (_, (_value, children)) in &self.store {
            descendants.extend(children.all_descendants())
        }
        descendants
    }

    pub fn put(
        &mut self,
        key: Vec<u8>,
        value: ScryptoValue,
        values: HashMap<AddressPath, REValue>,
    ) {
        if let Some((ref mut cur_value, children)) = self.store.get_mut(&key) {
            *cur_value = value;
            children.insert_children(values);
        } else {
            self.store
                .insert(key, (value, InMemoryChildren::with_values(values)));
        }
    }

    pub fn get(&self, key: &[u8]) -> Option<&(ScryptoValue, InMemoryChildren)> {
        self.store.get(key)
    }
}
