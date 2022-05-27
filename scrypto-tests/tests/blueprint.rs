#![cfg_attr(not(feature = "std"), no_std)]

use scrypto::abi::BlueprintAbi;
use scrypto::buffer::*;
use scrypto::prelude::*;
use serde::Serialize;
use serde_json::{json, to_value, Value};

blueprint! {
    struct Simple {
        state: u32,
    }

    impl Simple {
        pub fn new() -> ComponentAddress {
            Self {
                state: 0
            }
            .instantiate()
            .globalize()
        }

        pub fn get_state(&self) -> u32 {
            self.state
        }

        pub fn set_state(&mut self, new_state: u32) {
            self.state = new_state;
        }

        pub fn custom_types() -> (Decimal, PackageAddress, LazyMap<String, String>, Hash, Bucket, Proof, Vault) {
            todo!()
        }
    }
}

fn assert_json_eq<T: Serialize>(actual: T, expected: Value) {
    assert_eq!(to_value(&actual).unwrap(), expected);
}

#[test]
fn test_simple_abi() {
    let ptr = Simple_abi(core::ptr::null_mut::<u8>());
    let abi: BlueprintAbi = scrypto_consume(ptr, |slice| scrypto_decode(slice).unwrap());
    assert_json_eq(
        abi,
        json!(
            {
                "value_schema": {
                    "type": "Struct",
                    "name": "Simple",
                    "fields": {
                        "type": "Named",
                        "named": [
                            [ "state", {"type":"U32"} ]
                        ]
                    }
                },
                "functions": [
                    {
                        "name": "new",
                        "inputs":[],
                        "output":{
                            "type":"Custom",
                            "name":"ComponentAddress",
                            "generics":[]
                        }
                    },
                    {
                        "name":"custom_types",
                        "inputs":[],
                        "output":{
                            "type":"Tuple",
                            "elements":[
                                {"type":"Custom",
                                    "name":"Decimal",
                                    "generics":[]
                                },
                                {
                                    "type": "Custom",
                                    "name":"PackageAddress",
                                    "generics":[]
                                },
                                {
                                    "type":"Custom",
                                    "name":"LazyMap",
                                    "generics":[
                                        { "type":"String" },
                                        {"type":"String"}
                                    ]
                                },
                                {"type":"Custom","name":"Hash","generics":[]
                                },{"type":"Custom","name":"Bucket","generics":[]
                                },{"type":"Custom","name":"Proof","generics":[]
                                },{"type":"Custom","name":"Vault","generics":[]}
                            ]
                        }
                    },
                ],
                "methods":[
                    {"name":"get_state","inputs":[],"output":{"type":"U32"}},
                    {"name":"set_state","inputs":[{"type":"U32"}],"output":{"type":"Unit"}}
                ]
            }
        )
    );
}
