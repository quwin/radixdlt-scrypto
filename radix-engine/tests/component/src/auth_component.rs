use scrypto::prelude::*;

blueprint! {
    struct AuthComponent {
        some_non_fungible: NonFungibleAddress,
    }

    impl AuthComponent {
        pub fn create_component(some_non_fungible: NonFungibleAddress) -> ComponentId {
            Self {
                some_non_fungible
            }
            .instantiate_with_auth(component_authorization! {
                "get_secret" => any_of!(0),
                "update_auth" => any_of!(0),
            })
        }

        pub fn get_secret(&self) -> String {
            "Secret".to_owned()
        }
        
        pub fn update_auth(&mut self, some_non_fungible: NonFungibleAddress) {
            self.some_non_fungible = some_non_fungible;
        }
    }
}