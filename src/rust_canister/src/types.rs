use::candid::{CandidType, Deserialize, Principal};

#[derive(CandidType, Deserialize, Clone)]
pub struct Member {
    pub name: String,
    pub age: u8
}

pub type Blob = Vec<u8>;

pub type Subaccount = Blob;

#[derive(CandidType, Deserialize, Clone)]
pub struct Account {
    pub owner: Principal,
    pub subaccount: Option<Subaccount>,
}

#[derive(CandidType, Deserialize, Clone, PartialEq)]
pub enum Status {
    Open,
    Accepted,
    Rejected,
}

#[derive(CandidType, Deserialize, Clone)]
pub enum CreateProposalOk {
    ProposalCreated(u64),
}

#[derive(CandidType, Deserialize, Clone)]
pub struct Proposal {
    pub id: u64,
    pub status: Status,
    pub manifest: String,
    pub votes: i64,
    pub voters: Vec<Principal>,
}

#[derive(CandidType, Deserialize, Clone)]
pub enum CreateProposalErr {
    NotDAOMember,
    NotEnoughTokens,
}

#[derive(CandidType, Deserialize, Clone)]
pub enum VoteErr {
    ProposalNotFound,
    AlreadyVoted,
    ProposalEnded,
}

#[derive(CandidType, Deserialize, Clone)]
pub enum VoteOk {
    ProposalAccepted,
    ProposalRefused,
    ProposalOpen,
}