use ic_cdk::{query, update, export_candid};
use std::cell::RefCell;

mod goals;

thread_local!{

    //This is a string literal. In Rust, string literals have a 'static lifetime because they are embedded directly in the final binary and are always available during the program execution.
    pub static NAME: &'static str = "AlienChainDao";

    //In Rust, static variables are global and remain in memory for the lifetime of the running program.
    // Static variables in Rust are immutable by default. If you need to modify a static variable, you must use a type that supports interior mutability, like RefCell or Mutex.
    // RefCell doesn't provide thread safety
    // RefCell provides "interior mutability".  even if RefCell is accessed through an immutable reference, you can still change the data inside it. 
    pub static MANIFESTO: RefCell<String> = RefCell::new("Go look for aliens in the ICPverse".to_string());

    pub static GOALS: RefCell<Vec<String>> = RefCell::new(vec![]);
}


#[query]
fn get_name() -> &'static str {
    NAME.with(|name| *name)
}

#[query]
fn get_manifesto() -> String {
    MANIFESTO.with(|manifesto| manifesto.borrow().clone())
}

#[update]
fn set_manifesto(new_manifesto: String) {
    MANIFESTO.with(|manifesto| *manifesto.borrow_mut() = new_manifesto);
}


export_candid!();
