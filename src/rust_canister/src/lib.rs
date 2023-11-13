use ic_cdk::{query, export_candid};

#[query]
fn greet(name: String) -> String {
    format!("Hello, {}!", name)
}

#[query]
fn greet_two_times(name: String) -> String {
    format!("Hello, {}! Hello {}!", name, name)
}

export_candid!();
