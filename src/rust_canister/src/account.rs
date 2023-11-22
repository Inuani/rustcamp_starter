use candid::Principal;
use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};
use crate::{Account, Subaccount};

impl Account {
    
    pub fn get_default_subaccount() -> Subaccount {
        vec![0; 32] // Creates a Vec<u8> with 32 zeros
    }

        pub fn accounts_equal(&self, other: &Account) -> bool {
            let default_subaccount = Self::get_default_subaccount();

            let lhs_subaccount = self.subaccount.as_ref().unwrap_or(&default_subaccount);
            let rhs_subaccount = other.subaccount.as_ref().unwrap_or(&default_subaccount);
    
            self.owner == other.owner && lhs_subaccount == rhs_subaccount
        }

        pub fn account_belongs_to_principal(&self, principal: &Principal) -> bool {
            &self.owner == principal
        }

        pub fn accounts_hash(&self) -> u32 {
            let mut hasher = DefaultHasher::new();
    
            self.owner.hash(&mut hasher);
            self.subaccount.as_ref().unwrap_or(&Self::get_default_subaccount()).hash(&mut hasher);
    
            hasher.finish() as u32
        }
}

impl PartialEq for Account {
    fn eq(&self, other: &Self) -> bool {
        Account::accounts_equal(self, other)
    }
}

impl Eq for Account {}

impl Hash for Account {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.accounts_hash().hash(state);
    }
}