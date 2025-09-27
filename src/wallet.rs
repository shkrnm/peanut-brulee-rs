use std::collections::HashMap;
use crate::user::User;

pub struct WalletSystem {
    pub users: HashMap<String, User>,
}

impl WalletSystem {
    pub fn new() -> Self {
        WalletSystem {
            users: HashMap::new(),
        }
    }

    pub fn create_account(&mut self) -> String {
        let user = User::new();
        let address = user.address.clone();
        self.users.insert(address.clone(), user);
        address
    }

    pub fn get_user(&self, address: &str) -> Option<&User> {
        self.users.get(address)
    }

    pub fn list_addresses(&self) -> Vec<String> {
        self.users.keys().cloned().collect()
    }
}