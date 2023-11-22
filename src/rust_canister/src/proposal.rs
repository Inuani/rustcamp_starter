use::ic_cdk::{query, update, api::caller};
use crate::{PROPOSALS, LEDGER, NEXT_PROPOSAL_ID, Status, Proposal, VoteOk, VoteErr, CreateProposalErr, CreateProposalOk, member::get_member, Account};
use CreateProposalErr::{NotDAOMember, NotEnoughTokens};
use CreateProposalOk::ProposalCreated;
use VoteErr::{ProposalNotFound, AlreadyVoted, ProposalEnded};
use Status::{Open, Accepted, Rejected};
use VoteOk::{ProposalAccepted, ProposalRefused, ProposalOpen};


#[update(name = "createProposal")]
fn create_proposal(manifest: String) -> Result<CreateProposalOk, CreateProposalErr> {
    let caller_id = caller();
    let member = get_member(caller_id).map_err(|_err| NotDAOMember)?;

    let caller_account = Account {
        owner: caller_id,
        subaccount: None,
    };

    let sufficient_balance = LEDGER.with(|ledger| {
        let mut ledger = ledger.borrow_mut();
        let cur_balance = *ledger.get(&caller_account).unwrap_or(&0);
        if cur_balance >= 1
        {
            ledger.insert(caller_account, cur_balance - 1);
            true
        }
        else {
            false}
    });

    if !sufficient_balance {
        return Err(NotEnoughTokens);}

    let proposal_id = NEXT_PROPOSAL_ID.with(|id| {
        let mut id = id.borrow_mut();
        let proposal_id = *id;
        *id += 1;
        proposal_id
    });

    let new_proposal = Proposal {
        id: proposal_id,
        status: Open,
        manifest,
        votes: 0,
        voters: Vec::new(),
    };

    PROPOSALS.with(|proposals| {
        proposals.borrow_mut().insert(proposal_id, new_proposal);
    });

    Ok(ProposalCreated(proposal_id))
}

#[update]
fn vote(id: u64, up_vote: bool) -> Result<VoteOk, VoteErr> {
    PROPOSALS.with(|proposals| {
        let mut proposals = proposals.borrow_mut();
        if let Some(proposal) = proposals.get_mut(&id)
        {
            if proposal.status != Open {
                return Err(ProposalEnded);}
            
            if proposal.voters.contains(&caller()) {
                return Err(AlreadyVoted);}

            let vote_value = if up_vote { 1 } else { -1 };
            proposal.votes += vote_value;

            proposal.status = if proposal.votes >= 100 {
                Accepted
            } else if proposal.votes <= -100 {
                Rejected
            } else {
                Open}; 
            
            proposal.voters.push(caller());
            
            match proposal.status {
                Accepted => Ok(ProposalAccepted),
                Rejected => Ok(ProposalRefused),
                Open => Ok(ProposalOpen)
            }
        }   
        else {
            return Err(ProposalNotFound);}
    })
}
        

#[query]
fn get_proposal(id: u64) -> Option<Proposal> {
    PROPOSALS.with(|proposals|
        proposals.borrow().get(&id).cloned()
    )
}