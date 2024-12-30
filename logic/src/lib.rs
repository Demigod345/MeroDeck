use calimero_sdk::borsh::{BorshDeserialize, BorshSerialize};
use calimero_sdk::env::ext::{AccountId, ProposalId};
use calimero_sdk::serde::{Deserialize, Serialize};
use calimero_sdk::types::Error;
use calimero_sdk::app;
// use calimero_storage::collections::{UnorderedMap, Vector};
// use rand::seq::SliceRandom;
// use rand::thread_rng;

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
    winner: Option<usize>,
}

#[derive(Debug, Clone, BorshDeserialize, BorshSerialize, Serialize, Deserialize)]
#[borsh(crate = "calimero_sdk::borsh")]
#[serde(crate = "calimero_sdk::serde")]
pub struct Player {
    public_key: String,
    chips: u64,
    cards: Vec<Card>,  // Will be encrypted later
    is_folded: bool,
    current_bet: u64,
    is_all_in: bool,
}

#[derive(Copy, Clone, Debug, BorshSerialize, BorshDeserialize, Serialize, Deserialize, PartialOrd, PartialEq, Eq, Ord)]
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
    AllFolded,   // All but one player has folded
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
    phase: GamePhase,
    players: Vec<Player>,
    action_position: usize,
    starting_position: usize,
    pot: u64,
    current_bet: Option<u64>,
    round_bets: Vec<u64>,
    checked_positions: Vec<usize>,
    community_cards: Vec<Card>,
    // deck: Vec<Card>,
    winner: Option<usize>,
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

#[derive(Serialize, Deserialize, Debug)]
#[serde(crate = "calimero_sdk::serde")]
pub struct SetPlayerCardsRequest {
    pub player_index: usize,
    pub cards: Vec<Card>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(crate = "calimero_sdk::serde")]
pub struct SetCommunityCardsRequest {
    pub cards: Vec<Card>,
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
            winner: None,
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

    pub fn start_game(&mut self) -> Result<(), Error> {
        if self.players.len() < 2 {
            return Err(Error::msg("Not enough players"));
        }

        // Reset game state
        self.community_cards.clear();
        self.phase = GamePhase::PreFlop;
        self.action_position = 0;
        self.starting_position = 0;
        self.pot = 0;
        self.current_bet = None;
        self.round_bets = vec![0; self.players.len()];
        self.checked_positions.clear();

        //Shuffle the deck
        // self.deck.shuffle(&mut thread_rng());

        // Deal 2 cards to each player
        for player in self.players.iter_mut() {
            player.cards = self.deck.drain(..2).collect();
        }

        self.community_cards = self.deck.drain(..5).collect();
        Ok(())

    }

    // Setting community cards and hole cards to known cards for testing
    pub fn set_player_cards(&mut self, request:SetPlayerCardsRequest) -> Result<(), Error> {
        let player_index = request.player_index;
        let cards = request.cards;
        if player_index >= self.players.len() {
            return Err(Error::msg("Invalid player index"));
        }

        if cards.len() != 2 {
            return Err(Error::msg("Invalid number of cards"));
        }

        self.players[player_index].cards = cards;
        Ok(())
    }

    pub fn set_community_cards(&mut self, cards: Vec<Card>) -> Result<(), Error> {
        if cards.len() != 5 {
            return Err(Error::msg("Invalid number of cards"));
        }

        self.community_cards = cards;
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

                // Check if all but one player has folded
                let active_players: Vec<_> = self.players.iter()
                    .enumerate()
                    .filter(|(_, p)| !p.is_folded)
                    .map(|(i, _)| i)
                    .collect();

                if active_players.len() == 1 {
                    // Only one player left
                    self.phase = GamePhase::AllFolded;
                    self.winner = Some(active_players[0]);
                    return Ok(());
                }
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
                self.pot += amount;

                
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
                self.pot += amount_to_call;

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
                self.pot += amount_to_raise;
                // self.last_raise_position = Some(player_index);
            }


            
        }
        self.advance_action()?;
        Ok(())
    }

    fn advance_action(&mut self) -> Result<(), Error> {

        // If bet is not None and all the players have betted the same amount then the round is complete
        if self.current_bet.is_some() {
            // let all_betted = self.round_bets.iter().all(|&bet| bet == self.current_bet.unwrap());
            let all_betted = self.players.iter().enumerate().all(|(i, player)| player.is_folded || self.round_bets[i] == self.current_bet.unwrap());
            if all_betted {
                self.advance_phase()?;
                return Ok(());
            }
        }


        // Advance phase if everyone who hasn't folded has checked
        if self.checked_positions.len() == self.players.iter().filter(|p| !p.is_folded).count() {
            self.advance_phase()?;
            return Ok(());
        }


        // let mut next_position = (self.action_position + 1) % self.players.len();
        let mut next_position = self.action_position;
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

        let mut all_folded:bool = true;
        for player in self.players.iter() {
            if !player.is_folded {
                all_folded = false;
                break;
            }
        }
        if !all_folded {
            while self.players[self.action_position].is_folded { //No but this can go in a inf loop if all the players are folded, no but that never happens
                self.action_position = (self.action_position + 1) % self.players.len();
            }
        }
        
        Ok(())
    }

    fn handle_showdown(&mut self) -> Result<(), Error> {

        // Determine winner
        let winner = self.determine_winner()?;
        self.winner = Some(winner); // Frontend will decide from this only that if winner is not none then the game is over
        // Distribute pot through proposal to external contract

        Ok(())
    }

    fn rank_to_value(rank: &Rank) -> usize {
        match rank {
            Rank::Two => 0,
            Rank::Three => 1,
            Rank::Four => 2,
            Rank::Five => 3,
            Rank::Six => 4,
            Rank::Seven => 5,
            Rank::Eight => 6,
            Rank::Nine => 7,
            Rank::Ten => 8,
            Rank::Jack => 9,
            Rank::Queen => 10,
            Rank::King => 11,
            Rank::Ace => 12,
        }
    }

    fn check_consecutive(values: &[usize]) -> bool {
        for window in values.windows(5) {
            if window.windows(2).all(|w| w[1] == w[0] + 1) {
                return true;
            }
        }
        false
    }

    fn determine_winner(&self) -> Result<usize, Error> {
        // Compare hands of active players with the community cards
        // Determine winner
        let mut scores: Vec<i32> = vec![-1; self.players.len()];
        for (i, player) in self.players.iter().enumerate() {
            if player.is_folded {
                continue;
            }
            let player_score = self.evaluate_hand(player);
            scores[i] = player_score as i32;
        }

        let max_score = scores.iter().max().ok_or_else(|| Error::msg("No active players"))?;
        let winners: Vec<usize> = scores.iter().enumerate()
            .filter(|&(_, &score)| score == *max_score)
            .map(|(index, _)| index)
            .collect();

        if winners.is_empty() {
            return Err(Error::msg("No winners found"));
        }

        Ok(winners[0]) // Return the first winner for simplicity
    }

    fn evaluate_hand(&self, player: &Player) -> usize {
        // Evaluate the hand of the player and gives a maximum possible score
        // Royal Flush: 10
        // Straight Flush: 9
        // Four of a Kind: 8
        // Full House: 7
        // Flush: 6
        // Straight: 5
        // Three of a Kind: 4
        // Two Pair: 3
        // One Pair: 2
        // High Card: 1
        let mut cards = player.cards.clone();
        cards.extend(self.community_cards.clone());

        if self.has_royal_flush(cards.clone()) {
            return 10
        } else if self.has_straight_flush(cards.clone()) {
            return 9
        } else if self.has_four_of_a_kind(cards.clone()) {
            return 8
        } else if self.has_full_house(cards.clone()) {
            return 7
        } else if self.has_flush(cards.clone()) {
            return 6
        } else if self.has_straight(cards.clone()) {
            return 5
        } else if self.has_three_of_a_kind(cards.clone()) {
            return 4
        } else if self.has_two_pair(cards.clone()) {
            return 3
        } else if self.has_one_pair(cards.clone()) {
            return 2  
        } else {
            return 1
        }
    }

    pub fn has_royal_flush(&self, cards: Vec<Card>) -> bool {
        // Check if the player has a royal flush
        self.has_straight_flush(cards.clone()) && cards.iter().any(|card| card.rank == Rank::King) && cards.iter().any(|card| card.rank == Rank::Ace)
    }

    pub fn has_straight_flush(&self, cards:Vec<Card>) -> bool {
        // Check if the player has a straight flush
        self.has_flush(cards.clone()) && self.has_straight(cards)
    }

    pub fn has_four_of_a_kind(&self, cards:Vec<Card>) -> bool {
        let mut rank_counts = [0; 13];
        for card in cards.iter() {
            rank_counts[card.rank as usize] += 1;
        }
        rank_counts.iter().any(|&count| count == 4)
    }

    pub fn has_full_house(&self, cards:Vec<Card>) -> bool {
        // Check if the player has a full house
        self.has_three_of_a_kind(cards.clone()) && self.has_one_pair(cards)
    }

    pub fn has_flush(&self, cards: Vec<Card>) -> bool {
        // Check if the player has a flush
        let mut suit_counts = [0; 4];
        for card in cards.iter() {
            suit_counts[card.suit as usize] += 1;
        }
        suit_counts.iter().any(|&count| count >= 5)
    }

    pub fn has_straight(&self, cards:Vec<Card>) -> bool {
        // Check if the player has a straight           cards is an array containing the cards of the player and the community cards
        
        let mut ranks: Vec<Rank> = cards.iter().map(|card| card.rank.clone()).collect();
        ranks.sort();
        ranks.dedup();

        let rank_values: Vec<usize> = ranks.iter().map(|rank| GameState::rank_to_value(rank)).collect();

        if rank_values.len() < 5 {
            return false;
        }

        if GameState::check_consecutive(&rank_values) {
            return true;
        }

        if rank_values.contains(&12) {
            let mut adjusted_values = rank_values.clone();
            adjusted_values.retain(|&value| value != 12); // Remove Ace (high value)
            adjusted_values.insert(0, 0); // Add Ace as the lowest value (0)
            if GameState::check_consecutive(&adjusted_values) {
                return true;
            }
        }

        false
    }

    pub fn has_three_of_a_kind(&self, cards:Vec<Card>) -> bool {
        // Check if the player has a three of a kind
        
        let mut rank_counts = [0; 13];
        for card in cards.iter() {
            rank_counts[card.rank as usize] += 1;
        }
        rank_counts.iter().any(|&count| count == 3)
    }

    pub fn has_two_pair(&self, cards:Vec<Card>) -> bool {
        // Check if the player has a two pair
        
        let mut rank_counts = [0; 13];
        for card in cards.iter() {
            rank_counts[card.rank as usize] += 1;
        }
        rank_counts.iter().filter(|&&count| count == 2).count() == 2
    }

    pub fn has_one_pair(&self, cards:Vec<Card>) -> bool {
        // Check if the player has a one pair
        
        let mut rank_counts = [0; 13];
        for card in cards.iter() {
            rank_counts[card.rank as usize] += 1;
        }
        rank_counts.iter().any(|&count| count == 2)
    }

    // fn has_high_card(&self, player: &Player) -> bool {
    //     // Check if the player has a high card
        
    //     false
    // }

    

    // a scoring for each player is to be maintained






    pub fn get_game_state(&self) -> Result<TestGameEvent, Error> {
        // Getting the game state for testing
        Ok(TestGameEvent {
            phase: self.phase.clone(),
            players: self.players.clone(),
            action_position: self.action_position,
            starting_position: self.starting_position,
            pot: self.pot,
            current_bet: self.current_bet,
            round_bets: self.round_bets.clone(),
            checked_positions: self.checked_positions.clone(),
            community_cards: self.community_cards.clone(),
            // deck: self.deck.clone(),
            winner: self.winner,
        })
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

#[cfg(test)]
mod tests {
    use super::*;
    
    // Helper function to setup test environment
    #[test]
    fn test_has_one_pair() {
        let cards = vec![
            Card { rank: Rank::Two, suit: Suit::Hearts },
            Card { rank: Rank::Two, suit: Suit::Diamonds },
            Card { rank: Rank::Three, suit: Suit::Clubs },
            Card { rank: Rank::Four, suit: Suit::Spades },
            Card { rank: Rank::Five, suit: Suit::Hearts },
            Card { rank: Rank::Six, suit: Suit::Diamonds },
            Card { rank: Rank::Seven, suit: Suit::Clubs },
        ];
        let state = GameState::init();
        // state.has_one_pair(cards)
        assert!(state.has_one_pair(cards));
    }

    #[test]
    fn test_has_two_pair() {
        let cards = vec![
            Card { rank: Rank::King, suit: Suit::Diamonds },
            Card { rank: Rank::Ten, suit: Suit::Clubs },
            Card { rank: Rank::Ace, suit: Suit::Hearts },
            Card { rank: Rank::King, suit: Suit::Hearts },
            Card { rank: Rank::Nine, suit: Suit::Spades },
            Card { rank: Rank::Ten, suit: Suit::Hearts },
            Card { rank: Rank::Jack, suit: Suit::Hearts },
        ];
        let state = GameState::init();
        // state.has_two_pair(cards)
        assert!(state.has_two_pair(cards));
    }

    #[test]
    fn test_has_three_of_a_kind() {
        let cards = vec![
            Card { rank: Rank::Two, suit: Suit::Hearts },
            Card { rank: Rank::Two, suit: Suit::Diamonds },
            Card { rank: Rank::Two, suit: Suit::Clubs },
            Card { rank: Rank::Three, suit: Suit::Spades },
            Card { rank: Rank::Four, suit: Suit::Hearts },
            Card { rank: Rank::King, suit: Suit::Diamonds },
            Card { rank: Rank::Five, suit: Suit::Clubs },
        ];
        let state = GameState::init();
        // state.has_three_of_a_kind(cards)
        assert!(state.has_three_of_a_kind(cards));
    }

    #[test]
    fn test_has_straight() {
        let cards = vec![
            Card { rank: Rank::Queen, suit: Suit::Hearts },
            Card { rank: Rank::Jack, suit: Suit::Spades },
            Card { rank: Rank::Ace, suit: Suit::Hearts },
            Card { rank: Rank::King, suit: Suit::Hearts },
            Card { rank: Rank::Nine, suit: Suit::Spades },
            Card { rank: Rank::Ten, suit: Suit::Hearts },
            Card { rank: Rank::Jack, suit: Suit::Hearts },
        ];
        let state = GameState::init();

        assert!(state.has_straight(cards));
    }


}
