use calimero_sdk::borsh::{de, BorshDeserialize, BorshSerialize};
use calimero_sdk::env::ext::{AccountId, ProposalId};
use calimero_sdk::serde::{Deserialize, Serialize};
use calimero_sdk::types::Error;
use calimero_sdk::app;
use calimero_storage::collections::{UnorderedMap, Vector};

#[app::state(emits = Event)]
#[derive(Debug, BorshSerialize, BorshDeserialize)]
#[borsh(crate = "calimero_sdk::borsh")]
pub struct GameState {
    // Need to store which player is active currently
    active_player: u32,
    players: Vec<Player>,
    community_cards: Vec<Card>, // Will be encrypted later
    phase: GamePhase,
    action_position: usize,
    starting_position: usize,
    pot: u64,
    current_bet: Option<u64>,
    // last_raise_position: Option<usize>,
    round_bets: Vec<u64>, //Stores the amout betted by each player in the current round
    checked_positions: Vec<usize>, //Stores the positions of players who have checked
    deck: Vec<Card>, // Will be encrypted later
}

#[derive(Debug, Clone, BorshDeserialize, BorshSerialize, Serialize, Deserialize)]
#[borsh(crate = "calimero_sdk::borsh")]
#[serde(crate = "calimero_sdk::serde")]
pub struct Player {
    public_key: String,
    chips: u64,
    cards: Vec<u32>,  // Will be encrypted later
    is_folded: bool,
    current_bet: u64,
    is_all_in: bool,
}

#[derive(Copy, Clone, Debug, BorshSerialize, BorshDeserialize, Serialize, Deserialize)]
#[borsh(crate = "calimero_sdk::borsh")]
#[serde(crate = "calimero_sdk::serde")]
pub enum Rank {
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
    Nine,
    Ten,
    Jack,
    Queen,
    King,
    Ace,
}

#[derive(Copy, Clone, Debug, BorshSerialize, BorshDeserialize, Serialize, Deserialize)]
#[borsh(crate = "calimero_sdk::borsh")]
#[serde(crate = "calimero_sdk::serde")]
pub enum Suit {
    Hearts,
    Diamonds,
    Clubs,
    Spades,
}

#[derive(Copy, Clone, Debug, BorshSerialize, BorshDeserialize, Serialize, Deserialize)]
#[borsh(crate = "calimero_sdk::borsh")]
#[serde(crate = "calimero_sdk::serde")]
pub struct Card {
    rank: Rank,
    suit: Suit,
}

pub fn init_deck() -> Vec<Card> {
    let mut deck: Vec<Card> = Vec::new();
    for suit in &[Suit::Hearts, Suit::Diamonds, Suit::Clubs, Suit::Spades] {
        for rank in &[Rank::Two, Rank::Three, Rank::Four,Rank::Five, Rank::Six, Rank::Seven, Rank::Eight, Rank::Nine, Rank::Ten, Rank::King, Rank::Queen, Rank::Jack, Rank::Ace] {
            deck.push(Card {
                rank: *rank,
                suit: *suit,
            });
        }
    }
    deck
}

#[derive(Debug, Clone, BorshDeserialize, BorshSerialize, Serialize, Deserialize)]
#[borsh(crate = "calimero_sdk::borsh")]
#[serde(crate = "calimero_sdk::serde")]
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
    Bet(u64),
    Call,
    Raise(u64),
    Fold,
    // AllIn, // Will be implemented later
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


#[derive(Debug, Clone, BorshDeserialize, BorshSerialize, Serialize, Deserialize)]
#[borsh(crate = "calimero_sdk::borsh")]
#[serde(crate = "calimero_sdk::serde")]
pub struct TestGameEvent{
    players: Vec<Player>,
    action_position: usize,
    starting_position: usize,
    pot: u64,
    current_bet: Option<u64>,
    round_bets: Vec<u64>,
    checked_positions: Vec<usize>,
    deck: Vec<Card>,
}



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

// Request for join game
#[derive(Serialize, Deserialize, Debug)]
#[serde(crate = "calimero_sdk::serde")]
pub struct JoinGameRequest {
    pub public_key: String,
}

// Request for processing action
#[derive(Serialize, Deserialize, Debug)]
#[serde(crate = "calimero_sdk::serde")]
pub struct ProcessActionRequest {
    pub action: PlayerAction,
    pub player_index: usize,
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
            current_bet: None,
            deck: init_deck(),
            // last_raise_position: None,
            round_bets: Vec::new(),
            checked_positions: Vec::new(),
        }
    }

    // Sample function
    pub fn get_active_player(&self) -> Result<u32, Error> {
        Ok(self.active_player)
    }

    //Sample function
    pub fn set_active_player(&mut self, request: ChangePlayerRequest) -> Result<(), Error> {
        
        //Parsing the request
        let new_player = request.new_player;
        
        self.active_player = new_player;

        // env::emit(&Event::PlayerChanged { id: new_player });
        app::emit!(Event::PlayerChanged { id: new_player });
        Ok(())
    }


    // For joining the game
    pub fn join_game(&mut self, request: JoinGameRequest) -> Result<(), Error> {
        // Check if player already exists
        let public_key = request.public_key;

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

        self.round_bets.push(0);

        Ok(())
    }

    pub fn process_action(&mut self, request: ProcessActionRequest) -> Result<(), Error> {
        
        let player_index = request.player_index;
        let action = request.action;
        if player_index != self.action_position {
            return Err(Error::msg("Not your turn"));
        }

        let player = &mut self.players[player_index];
        if player.is_folded {
            return Err(Error::msg("Player is already folded"));
        }

        match action {
            PlayerAction::Check => {
                if self.current_bet.is_some() {
                    return Err(Error::msg("Cannot check when there's a bet"));
                }
                self.checked_positions.push(player_index);
                // This looks good
            },
            PlayerAction::Fold => {
                
                player.is_folded = true;
            },
            PlayerAction::Bet(amount) => {
                // Can only be done if no one has betted yet
                if self.current_bet.is_some() {
                    return Err(Error::msg("Cannot bet when there's a bet"));
                }


                // Not managing all in for now
                if amount > player.chips {
                    return Err(Error::msg("Insufficient chips"));
                }

                player.chips -= amount;
                player.current_bet += amount;
                self.current_bet = Some(amount);
                self.round_bets[player_index] = amount;

                
            },
            PlayerAction::Call => {
                // Call only if there is a bet or the rounds
                let current_bet = self.current_bet.ok_or_else(|| Error::msg("No bet to call"))?;
                let amount_to_call = current_bet - self.round_bets[player_index];

                if amount_to_call > player.chips {
                    return Err(Error::msg("Insufficient chips"));
                }

                player.chips -= amount_to_call;
                player.current_bet += amount_to_call;
                self.round_bets[player_index] = current_bet;

            }

            PlayerAction::Raise(new_bet) => {

                // Raise only if there is a bet or the rounds
                if self.current_bet.is_none() {
                    return Err(Error::msg("Cannot raise without a bet"));
                }

                // can raise only if his previous bet is zero (either not betted or checked)
                // for simplicity, only one raise per round
                if self.round_bets[player_index] != 0 {
                    return Err(Error::msg("Already betted cannot raise"));
                }

                let amount_to_raise = new_bet - self.round_bets[player_index];

                if amount_to_raise > player.chips {
                    return Err(Error::msg("Insufficient chips"));
                }

                player.chips -= amount_to_raise;
                player.current_bet = new_bet;
                self.current_bet = Some(new_bet);
                self.round_bets[player_index] = new_bet;
                // self.last_raise_position = Some(player_index);
            }


            
        }
        self.advance_action()?;
        Ok(())
    }

    fn advance_action(&mut self) -> Result<(), Error> {

        // If bet is not None and all the players have betted the same amount then the round is complete
        if self.current_bet.is_some() {
            let all_betted = self.round_bets.iter().all(|&bet| bet == self.current_bet.unwrap());
            if all_betted {
                self.advance_phase()?;
                return Ok(());
            }
        }


        // Advance phase if everyone has checked
        if self.checked_positions.len() == self.players.len() {
            self.advance_phase()?;
            return Ok(());
        }


        let mut next_position = (self.action_position + 1) % self.players.len();
        // let mut next_position = self.action_position;
        for _ in 0..self.players.len() {
            next_position = (next_position + 1) % self.players.len();
            if !self.players[next_position].is_folded {
            break;
            }
        }
        
        // Check if betting round is complete
        // let round_complete = match self.last_raise_position {
        //     Some(raise_pos) => next_position == raise_pos,
        //     None => {
        //         // If no raises, check if everyone has acted
        //         let active_players: Vec<_> = self.players.iter()
        //             .enumerate()
        //             .filter(|(_, p)| !p.is_folded && !p.is_all_in)
        //             .map(|(i, _)| i)
        //             .collect();
                
        //         let all_checked = active_players.iter()
        //             .all(|&pos| self.checked_positions.contains(&pos));
                
        //         all_checked
        //     }
        // };

        // if round_complete {
        //     self.advance_phase()?;
        // } else {
        //     self.action_position = next_position;
        // }
        self.action_position = next_position;

        Ok(())
    }


    fn advance_phase(&mut self) -> Result<(), Error> {
        // Reset betting round state
        self.current_bet = None;
        // self.last_raise_position = None;
        self.checked_positions.clear();
        self.round_bets = vec![0; self.players.len()];

        match self.phase {
            GamePhase::PreFlop => {
                self.phase = GamePhase::Flop;
                // Deal 3 community cards
            },
            GamePhase::Flop => {
                self.phase = GamePhase::Turn;
                // Deal 1 card
            },
            GamePhase::Turn => {
                self.phase = GamePhase::River;
                // Deal 1 card
            },
            GamePhase::River => {
                self.phase = GamePhase::Showdown;
                self.handle_showdown()?;
            },
            _ => return Err(Error::msg("Invalid phase")),
        }

        // Set starting position for new phase
        self.action_position = self.starting_position;
        Ok(())
    }

    fn handle_showdown(&mut self) -> Result<(), Error> {
        // Compare hands of active players
        // Determine winner
        // Distribute pot
        Ok(())
    }

    fn get_game_state(&self) -> Result<(), Error> {
        // Getting the game state for testing

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
