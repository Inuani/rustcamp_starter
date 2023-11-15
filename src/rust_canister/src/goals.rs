use ic_cdk::{query, update };
use crate::GOALS;

#[query]
fn get_goals() -> Vec<String> {
    GOALS.with(|goals| goals.borrow().clone())
}

#[update]
fn add_goal(new_goal: String) {
    GOALS.with(|goals| goals.borrow_mut().push(new_goal));
}