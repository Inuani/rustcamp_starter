use::ic_cdk::{query, update};
use candid::Principal;
use crate::{LEDGER, Account};

#[query]
fn token_name() -> String {
    "AlienLX".to_string()
}

#[query]
fn token_symbol() -> String {
    "ALX".to_string()
}

#[update]
fn mint(principal: Principal, amount: u64) {
    LEDGER.with(|ledger| {
        let mut ledger = ledger.borrow_mut();
        
        let default_account = Account {
            owner: principal,
            subaccount: None,  //Some(Account::get_default_subaccount()),
        };

        let current_balance = *ledger.get(&default_account).unwrap_or(&0);

        ledger.insert(default_account, current_balance + amount);
    })
}

#[update]
fn transfer(from: Account, to: Account, amount: u64) -> Result<(), String> {
    LEDGER.with(|ledger| {
        let mut ledger = ledger.borrow_mut();

        let balance_from = ledger.get(&from).cloned().ok_or("insufficient funds")?;

        if balance_from < amount {
            return Err("insufficient funds".to_string())}

        let new_balance_from = balance_from - amount;
        let new_balance_to = ledger.get(&to).cloned().unwrap_or(0) + amount;

        ledger.insert(from, new_balance_from);
        ledger.insert(to, new_balance_to);
        Ok(())
    })
}

#[query]
fn balance_of(account: Account) -> u64 {
    LEDGER.with(|ledger|
        ledger.borrow()
            .get(&account)
            .cloned()
            .unwrap_or(0)
    )
}

#[query]
fn total_supply() -> u64 {
    LEDGER.with(|ledger|
        ledger.borrow()
            .values()
            .sum()
    )
}