
use ic_cdk::{query, update, api::caller };
use crate::{MEMBERS, Member};
use candid::{Principal};

#[update]
fn add_member(new_member: Member) -> Result<(), String> {
    MEMBERS.with(|members| {
       let mut members_mut = members.borrow_mut();
        if members_mut.contains_key(&caller()) {
            Err("Member already exists".to_string())}
        else {
            members_mut.insert(caller(), new_member); Ok(())}
    })
}

#[query]
pub fn get_member(principal: Principal) -> Result<Member, String> {
    MEMBERS.with(|members| 
        match members.borrow().get(&principal) {
            Some(member) => Ok(member.clone()),
            None => Err("Member not found".to_string()),
        }
    )
}

#[update]
fn update_member(updated_member: Member) -> Result<(), String> {
    MEMBERS.with( |members|
        match members.borrow_mut().get_mut(&caller()) {
            Some(member) => { *member = updated_member; Ok(())},
            None => Err("caller is not member".to_string()),
        }
    )
}

#[update]
fn remove_member() -> Result<(), String>{
    MEMBERS.with( |members| {
        if let Some (_) = members.borrow_mut().remove(&caller()) {
            Ok(())}
        else {
            Err("caller is not a member".to_string())}
    })
}

#[query]
fn get_all_members() -> Vec<Member> {
    MEMBERS.with( |members|
        members.borrow()
               .values()
               .cloned()
               .collect()
    )
}

#[query]
fn numbers_of_members() -> usize {
    MEMBERS.with(|members| members.borrow().len())
}

