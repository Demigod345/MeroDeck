use calimero_sdk::borsh::{BorshDeserialize, BorshSerialize};
use calimero_sdk::env::ext::{AccountId, ProposalId};
use calimero_sdk::serde::{Deserialize, Serialize};
use calimero_sdk::types::Error;
use calimero_sdk::{app, env};
use calimero_storage::collections::{UnorderedMap, Vector};

#[app::state(emits = Event)]
#[derive(Debug, BorshSerialize, BorshDeserialize)]
#[borsh(crate = "calimero_sdk::borsh")]
pub struct GameState {
    // Need to store which player is active currently
    active_player: u32,
    players: Vec<Player>,
    community_cards: Vec<u32>, // Will be encrypted later
    phase: GamePhase,
    action_position: usize,
    starting_position: usize,
    pot: u64,
    current_bet: u64,
    last_raise_position: Option<usize>,
    round_bets: Vec<u64>, //Stores the amout betted by each player in the current round
    checked_positions: Vec<usize>, //Stores the positions of players who have checked
    deck: Vec<u32>, // Will be encrypted later
}

#[derive(Debug, Clone, BorshDeserialize, BorshSerialize)]
#[borsh(crate = "calimero_sdk::borsh")]
pub struct Player {
    public_key: String,
    chips: u64,
    cards: Vec<u32>,  // Will be encrypted later
    is_folded: bool,
    current_bet: u64,
    is_all_in: bool,
}

#[derive(Debug, Clone, BorshDeserialize, BorshSerialize)]
#[borsh(crate = "calimero_sdk::borsh")]
pub enum GamePhase {
    Waiting,      // Waiting for players
    PreFlop,     // Initial betting
    Flop,        // After first 3 community cards
    Turn,        // After 4th community card
    River,       // After 5th community card
    Showdown,    // Final comparison of hands
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(crate = "calimero_sdk::serde")]
pub enum PlayerAction {
    Check,
    Call,
    Raise(u64),
    Fold,
    AllIn, // Will be implemented later
}





// Okay so here are defined the structs that will be used in the logic
// #[derive(
//     Clone, Debug, PartialEq, PartialOrd, BorshSerialize, BorshDeserialize, Serialize, Deserialize,
// )]
// #[borsh(crate = "calimero_sdk::borsh")]
// #[serde(crate = "calimero_sdk::serde")]
// pub struct Message {
//     id: String,
//     proposal_id: String,
//     author: String,
//     text: String,
//     created_at: String,
// }






// Here are defined the events that will be emitted by the logic
#[app::event]
pub enum Event {
    ProposalCreated { id: ProposalId },
    ApprovedProposal { id: ProposalId },
    PlayerChanged { id: u32 },
}


// Here are defined the requests that will be received by the logic
#[derive(Serialize, Deserialize, Debug)]
#[serde(crate = "calimero_sdk::serde")]
pub struct CreateProposalRequest {
    pub action_type: String,
    pub params: serde_json::Value,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(crate = "calimero_sdk::serde")]
pub struct ChangePlayerRequest {
    pub new_player: u32,
}



// These are the functions that will be called by the frontend
#[app::logic]
impl GameState {

    // Constructor of the state
    #[app::init]
    pub fn init() -> GameState {
        GameState {
            // messages: UnorderedMap::new(),
            active_player: 0,
            players: Vec::new(),
            community_cards: Vec::new(),
            phase: GamePhase::Waiting,
            action_position: 0,
            starting_position: 0,
            pot: 0,
            current_bet: 0,
            deck: (0..52).collect(),
            last_raise_position: None,
            round_bets: Vec::new(),
            checked_positions: Vec::new(),
        }
    }

    // Function to get the currently active player
    pub fn get_active_player(&self) -> Result<u32, Error> {
        Ok(self.active_player)
    }

    pub fn set_active_player(&mut self, request: ChangePlayerRequest) -> Result<(), Error> {
        
        //Parsing the request
        let new_player = request.new_player;
        
        self.active_player = new_player;

        // env::emit(&Event::PlayerChanged { id: new_player });
        app::emit!(Event::PlayerChanged { id: new_player });
        Ok(())
    }

    pub fn join_game(&mut self, public_key: String) -> Result<(), Error> {
        // Check if player already exists
        for player in self.players.iter() {
            if player.public_key == public_key {
                return Err(Error::msg("Player already exists"));
            }
        }

        if self.players.len() >=9 {
            return Err(Error::msg("Game is full"));
        }

        // Add player to the game
        self.players.push(Player {
            public_key: public_key,
            chips: 1000,
            cards: Vec::new(),
            is_folded: false,
            current_bet: 0,
            is_all_in: false,
        });

        Ok(())
    }

    pub fn process_action(&mut self, player_index: usize, action: PlayerAction) -> Result<(), Error> {
        let player = &mut self.players[player_index];

        // match action {
        //     PlayerAction::Check => {
        //         if self.current_bet > 0 {
        //             return Err("Cannot check when there's a bet".to_string());
        //         }
        //         self.checked_positions.push(player_index);
        //     },
        //     PlayerAction::Bet(amount) => {
        //         if self.checked_positions.contains(&player_index) {
        //             // Player can still bet after checking
        //             self.checked_positions.retain(|&x| x != player_index);
        //         }
        //         self.handle_bet(player_index, amount)?;
        //         self.last_raise_position = Some(player_index);
        //     },
        //     PlayerAction::Call => self.handle_call(player_index)?,
        //     PlayerAction::Fold => self.handle_fold(player_index)?,
        // }

        Ok(())
    }



    // Creating a new proposal
    // pub fn create_new_proposal(
    //     &mut self,
    //     request: CreateProposalRequest,
    // ) -> Result<ProposalId, Error> {

    //     // Logs into the node?
    //     env::log("Starting create_new_proposal");
    //     env::log(&format!("Request type: {}", request.action_type));



    //     let proposal_id = match request.action_type.as_str() {
    //         "ExternalFunctionCall" => {
    //             env::log("Processing ExternalFunctionCall");

    //             // Parsing happens in this specific format
    //             let receiver_id = request.params["receiver_id"]
    //                 .as_str()
    //                 .ok_or_else(|| Error::msg("receiver_id is required"))?;
    //             let method_name = request.params["method_name"]
    //                 .as_str()
    //                 .ok_or_else(|| Error::msg("method_name is required"))?;
    //             let args = request.params["args"]
    //                 .as_str()
    //                 .ok_or_else(|| Error::msg("args is required"))?;
    //             let deposit = request.params["deposit"]
    //                 .as_str()
    //                 .ok_or_else(|| Error::msg("deposit is required"))?
    //                 .parse::<u128>()?;
    //             let gas = request.params["gas"]
    //                 .as_str()
    //                 .map(|g| g.parse::<u64>())
    //                 .transpose()?
    //                 .unwrap_or(30_000_000_000_000);

    //             env::log(&format!(
    //                 "Parsed values: receiver_id={}, method_name={}, args={}, deposit={}, gas={}",
    //                 receiver_id, method_name, args, deposit, gas
    //             ));
                
    //             // What specifically is external? and propose()
    //             Self::external()
    //                 .propose()
    //                 .external_function_call(
    //                     receiver_id.to_string(),
    //                     method_name.to_string(),
    //                     args.to_string(),
    //                     deposit,
    //                     gas,
    //                 )
    //                 .send()
    //         }
    //         "Transfer" => {
    //             env::log("Processing Transfer");
    //             let receiver_id = request.params["receiver_id"]
    //                 .as_str()
    //                 .ok_or_else(|| Error::msg("receiver_id is required"))?;
    //             let amount = request.params["amount"]
    //                 .as_str()
    //                 .ok_or_else(|| Error::msg("amount is required"))?
    //                 .parse::<u128>()?;

    //             Self::external()
    //                 .propose()
    //                 .transfer(AccountId(receiver_id.to_string()), amount)
    //                 .send()
    //         }
    //         "SetContextValue" => {
    //             env::log("Processing SetContextValue");
    //             let key = request.params["key"]
    //                 .as_str()
    //                 .ok_or_else(|| Error::msg("key is required"))?
    //                 .as_bytes()
    //                 .to_vec()
    //                 .into_boxed_slice();
    //             let value = request.params["value"]
    //                 .as_str()
    //                 .ok_or_else(|| Error::msg("value is required"))?
    //                 .as_bytes()
    //                 .to_vec()
    //                 .into_boxed_slice();

    //             Self::external()
    //                 .propose()
    //                 .set_context_value(key, value)
    //                 .send()
    //         }
    //         "SetNumApprovals" => Self::external()
    //             .propose()
    //             .set_num_approvals(
    //                 request.params["num_approvals"]
    //                     .as_u64()
    //                     .ok_or(Error::msg("num_approvals is required"))? as u32,
    //             )
    //             .send(),
    //         "SetActiveProposalsLimit" => Self::external()
    //             .propose()
    //             .set_active_proposals_limit(
    //                 request.params["active_proposals_limit"]
    //                     .as_u64()
    //                     .ok_or(Error::msg("active_proposals_limit is required"))?
    //                     as u32,
    //             )
    //             .send(),
    //         "DeleteProposal" => Self::external()
    //             .propose()
    //             .delete(ProposalId(
    //                 hex::decode(
    //                     request.params["proposal_id"]
    //                         .as_str()
    //                         .ok_or_else(|| Error::msg("proposal_id is required"))?,
    //                 )?
    //                 .try_into()
    //                 .map_err(|_| Error::msg("Invalid proposal ID length"))?,
    //             ))
    //             .send(),
    //         _ => return Err(Error::msg("Invalid action type")),
    //     };

    //     env::emit(&Event::ProposalCreated { id: proposal_id });

    //     let old = self.messages.insert(proposal_id, Vector::new())?;
    //     if old.is_some() {
    //         return Err(Error::msg("proposal already exists"));
    //     }

    //     Ok(proposal_id)
    // }

    // pub fn approve_proposal(&self, proposal_id: ProposalId) -> Result<(), Error> {
    //     // fixme: should we need to check this?
    //     // self.messages
    //     //     .get(&proposal_id)?
    //     //     .ok_or(Error::msg("proposal not found"))?;

    //     Self::external().approve(proposal_id);

    //     env::emit(&Event::ApprovedProposal { id: proposal_id });

    //     Ok(())
    // }

    // pub fn get_proposal_messages(&self, proposal_id: ProposalId) -> Result<Vec<Message>, Error> {
    //     let Some(msgs) = self.messages.get(&proposal_id)? else {
    //         return Ok(vec![]);
    //     };

    //     let entries = msgs.entries()?;

    //     Ok(entries.collect())
    // }

    // pub fn send_proposal_messages(
    //     &mut self,
    //     proposal_id: ProposalId,
    //     message: Message,
    // ) -> Result<(), Error> {
    //     let mut messages = self.messages.get(&proposal_id)?.unwrap_or_default();

    //     messages.push(message)?;

    //     self.messages.insert(proposal_id, messages)?;

    //     Ok(())
    // }
}


// Here private functions can be defined
impl GameState {

}

// #[cfg(test)]
// mod tests {
//     use super::*;
    
//     // Helper function to setup test environment
//     fn setup() -> AppState {
//         AppState::init()
//     }

//     #[test]
//     fn test_currently_active_player() {
//         let state = setup();
//         assert_eq!(state.active_player, 0); // Assuming default is 0
        
//     }
// }
