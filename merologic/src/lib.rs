use calimero_sdk::borsh::{BorshDeserialize, BorshSerialize};
use calimero_sdk::env::ext::{AccountId, ProposalId};
use calimero_sdk::serde::{Deserialize, Serialize};
use calimero_sdk::types::Error;
use calimero_sdk::{app, env};
use calimero_storage::collections::{UnorderedMap, Vector};


#[app::state(emits = Event)]
#[derive(Debug, PartialEq, PartialOrd, BorshSerialize, BorshDeserialize)]
#[borsh(crate = "calimero_sdk::borsh")]
pub struct AppState {
    // Need to store which player is active currently
    messages: UnorderedMap<ProposalId, Vector<Message>>,
    currently_active: u32,

}


#[derive(
    Clone, Debug, PartialEq, PartialOrd, BorshSerialize, BorshDeserialize, Serialize, Deserialize,
)]
#[borsh(crate = "calimero_sdk::borsh")]
#[serde(crate = "calimero_sdk::serde")]
pub struct Message {
    id: String,
    proposal_id: String,
    author: String,
    text: String,
    created_at: String,
}



// Here are defined the events that will be emitted by the logic
#[app::event]
pub enum Event {
    ProposalCreated { id: ProposalId },
    ApprovedProposal { id: ProposalId },
}


// Here are defined the requests that will be received by the logic
#[derive(Serialize, Deserialize, Debug)]
#[serde(crate = "calimero_sdk::serde")]
pub struct CreateProposalRequest {
    pub action_type: String,
    pub params: serde_json::Value,
}

// These are the functions that will be called by the frontend
#[app::logic]
impl AppState {

    // Constructor of the state
    #[app::init]
    pub fn init() -> AppState {
        AppState {
            messages: UnorderedMap::new(),
            currently_active: 0,
        }
    }
}





#[cfg(test)]
mod tests {
    use super::*;
    
    // Helper function to setup test environment
    fn setup() -> AppState {
        AppState::init()
    }

    #[test]
    fn test_currently_active_player() {
        let state = setup();
        assert_eq!(state.currently_active, 0); // Assuming default is 0
        
    }
}