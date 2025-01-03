// #[starknet::contract]
// mod PokerGamePool {
//     use starknet::{
//         ContractAddress, get_caller_address, syscalls, SyscallResultTrait, get_contract_address,
//     };
//     use core::array::{ArrayTrait, Array, ArrayImpl};
//     // use core::option::OptionTrait;
//     use core::starknet::storage::{
//         Map, StorageMapReadAccess, StorageMapWriteAccess, StoragePointerReadAccess,
//         StoragePointerWriteAccess, StorableStoragePointerReadAccess,
//     };

//     #[derive(Drop, starknet::Store)]
//     struct Game {
//         pot_size: u256,
//         is_active: bool,
//     }

//     #[storage]
//     struct Storage {
//         game_token: ContractAddress,
//         current_game_id: u256,
//         games: Map<u256, Game>,
//         owner: ContractAddress,
//     }


//     #[event]
//     #[derive(Drop, starknet::Event)]
//     enum Event {
//         GameCreated: GameCreated,
//         PlayerJoined: PlayerJoined,
//         WinnerPaid: WinnerPaid,
//     }

//     #[derive(Drop, starknet::Event)]
//     struct GameCreated {
//         game_id: u256,
//     }

//     #[derive(Drop, starknet::Event)]
//     struct PlayerJoined {
//         game_id: u256,
//         player: ContractAddress,
//         amount: u256,
//     }

//     #[derive(Drop, starknet::Event)]
//     struct WinnerPaid {
//         game_id: u256,
//         winner: ContractAddress,
//         amount: u256,
//     }

//     #[constructor]
//     fn constructor(ref self: ContractState, token_address: ContractAddress) {
//         self.game_token.write(token_address);
//         self.owner.write(get_caller_address());
//         self.current_game_id.write(0);
//     }

//     #[external(v0)]
//     fn create_game(ref self: ContractState) -> u256 {
//         let game_id = self.current_game_id.read();
//         self.current_game_id.write(game_id + 1);

//         let game = Game { pot_size: 0, is_active: true };
//         self.games.write(game_id, game);

//         self.emit(GameCreated { game_id });

//         game_id
//     }

//     fn get_current_game_id(self: @ContractState) -> u256 {
//         self.current_game_id.read()
//     }

//     fn get_game(self: @ContractState, game_id: u256) -> Game {
//         self.games.read(game_id)
//     }

//     #[external(v0)]
//     fn add_to_pot(ref self: ContractState, game_id: u256, amount: u256) {
//         assert(amount > 0, 'Amount must be positive');

//         let game = self.games.read(game_id);
//         assert(game.is_active, 'Game not active');

//         self._transfer_from(get_caller_address(), amount);

//         let mut game = self.games.read(game_id);
//         game.pot_size += amount;
//         self.games.write(game_id, game);

//         self.emit(PlayerJoined { game_id, player: get_caller_address(), amount });
//     }

//     #[external(v0)]
//     fn declare_winner(ref self: ContractState, game_id: u256, winner: ContractAddress) {
//         assert(get_caller_address() == self.owner.read(), 'Not owner');

//         let mut game = self.games.read(game_id);
//         assert(game.is_active, 'Game not active');
//         assert(game.pot_size > 0, 'No pot to distribute');

//         let pot_amount = game.pot_size;
//         game.pot_size = 0;
//         game.is_active = false;

//         self.games.write(game_id, game);

//         self._transfer(winner, pot_amount);

//         self.emit(WinnerPaid { game_id, winner, amount: pot_amount });
//     }

//     fn get_game_pot(self: @ContractState, game_id: u256) -> u256 {
//         let game = self.games.read(game_id);
//         game.pot_size
//     }

//     #[external(v0)]
//     fn emergency_token_recovery(ref self: ContractState) {
//         assert(get_caller_address() == self.owner.read(), 'Not owner');
//         let balance = self._balance_of(get_contract_address());
//         assert(self._transfer(self.owner.read(), balance), 'Transfer failed');
//     }


//     #[generate_trait]
//     impl InternalFunctions of InternalFunctionsTrait {
//         fn _transfer(ref self: ContractState, to: ContractAddress, amount: u256) -> bool {
//             let mut call_data: Array<felt252> = array![];
//             Serde::serialize(@to, ref call_data);
//             Serde::serialize(@amount, ref call_data);

//             let mut res = syscalls::call_contract_syscall(
//                 self.game_token.read(), selector!("transfer"), call_data.span(),
//             )
//                 .unwrap_syscall();

//             let success = Serde::<bool>::deserialize(ref res).unwrap();
//             assert(success, 'Transfer failed');
//             success
//         }

//         fn _transfer_from(ref self: ContractState, from: ContractAddress, amount: u256) -> bool {
//             let mut call_data: Array<felt252> = array![];
//             Serde::serialize(@from, ref call_data);
//             Serde::serialize(@get_contract_address(), ref call_data);
//             Serde::serialize(@amount, ref call_data);

//             let mut res = syscalls::call_contract_syscall(
//                 self.game_token.read(), selector!("transfer_from"), call_data.span(),
//             )
//                 .unwrap_syscall();

//             let success = Serde::<bool>::deserialize(ref res).unwrap();
//             assert(success, 'TransferFrom failed');
//             success
//         }

//         fn _balance_of(self: @ContractState, account: ContractAddress) -> u256 {
//             let mut call_data: Array<felt252> = array![];
//             Serde::serialize(@account, ref call_data);

//             let mut res = syscalls::call_contract_syscall(
//                 self.game_token.read(), selector!("balance_of"), call_data.span(),
//             )
//                 .unwrap_syscall();

//             Serde::<u256>::deserialize(ref res).unwrap()
//         }
//     }
// }

// // #[cfg(test)]
// // mod tests {
// //     use starknet::ContractAddress;
// //     use core::array::ArrayTrait;
// //     use core::result::ResultTrait;
// //     use core::option::OptionTrait;
// //     use core::traits::TryInto;
// //     use starknet::syscalls::deploy_syscall;
// //     // use core::debug::PrintTrait;
// //     // use core::starknet::testing::{};

// //     // Import the interfaces
// //     #[starknet::interface]
// //     trait IERC20<TContractState> {
// //         fn transfer(ref self: TContractState, recipient: ContractAddress, amount: u256) -> bool;
// //         fn transfer_from(
// //             ref self: TContractState,
// //             sender: ContractAddress,
// //             recipient: ContractAddress,
// //             amount: u256,
// //         ) -> bool;
// //         fn approve(ref self: TContractState, spender: ContractAddress, amount: u256) -> bool;
// //         fn balance_of(self: @TContractState, account: ContractAddress) -> u256;
// //     }

// //     // Mock ERC20 contract for testing
// //     #[starknet::contract]
// //     mod MockERC20 {
// //         use super::IERC20;
// //         use starknet::{ContractAddress, get_caller_address, contract_address_const};
// //         use core::starknet::storage::{
// //             Map, StorageMapReadAccess, StorageMapWriteAccess, StoragePointerReadAccess,
// //             StoragePointerWriteAccess, StorableStoragePointerReadAccess,
// //         };
// //         // use core::testing::test_address;

// //         // Test Players
// //         fn setup_players() -> (
// //             ContractAddress,
// //             ContractAddress,
// //             ContractAddress,
// //             ContractAddress,
// //             ContractAddress,
// //             ContractAddress,
// //             ContractAddress,
// //             ContractAddress,
// //         ) {
// //             let player1 = contract_address_const::<1>();
// //             let player2 = contract_address_const::<2>();
// //             let player3 = contract_address_const::<3>();
// //             let player4 = contract_address_const::<4>();
// //             let player5 = contract_address_const::<5>();
// //             let player6 = contract_address_const::<6>();
// //             let player7 = contract_address_const::<7>();
// //             let player8 = contract_address_const::<8>();
// //             (player1, player2, player3, player4, player5, player6, player7, player8)
// //         }

// //         #[storage]
// //         struct Storage {
// //             balances: Map<ContractAddress, u256>,
// //             allowances: Map<(ContractAddress, ContractAddress), u256>,
// //         }

// //         #[constructor]
// //         fn constructor(ref self: ContractState) {
// //             // Initialize test balances
// //             self.balances.write(contract_address_const::<1>(), 1000000);
// //         }

// //         impl MockERC20 of super::IERC20<ContractState> {
// //             fn transfer(ref self: ContractState, recipient: ContractAddress, amount: u256) -> bool {
// //                 let sender = get_caller_address();
// //                 self.balances.write(sender, self.balances.read(sender) - amount);
// //                 self.balances.write(recipient, self.balances.read(recipient) + amount);
// //                 true
// //             }

// //             fn transfer_from(
// //                 ref self: ContractState,
// //                 sender: ContractAddress,
// //                 recipient: ContractAddress,
// //                 amount: u256,
// //             ) -> bool {
// //                 let spender = get_caller_address();
// //                 self
// //                     .allowances
// //                     .write((sender, spender), self.allowances.read((sender, spender)) - amount);
// //                 self.balances.write(sender, self.balances.read(sender) - amount);
// //                 self.balances.write(recipient, self.balances.read(recipient) + amount);
// //                 true
// //             }

// //             fn approve(ref self: ContractState, spender: ContractAddress, amount: u256) -> bool {
// //                 self.allowances.write((get_caller_address(), spender), amount);
// //                 true
// //             }

// //             fn balance_of(self: @ContractState, account: ContractAddress) -> u256 {
// //                 self.balances.read(account)
// //             }
// //         }
// //     }

// //     #[test]
// //     fn test_create_game() {
// //         // Deploy mock token
// //         let (token_address, _) = deploy_syscall(
// //             MockERC20::TEST_CLASS_HASH.try_into().unwrap(), 0, array![].span(), false,
// //         )
// //             .unwrap();

// //         // Deploy poker pool
// //         let (pool_address, _) = deploy_syscall(
// //             super::PokerGamePool::TEST_CLASS_HASH.try_into().unwrap(),
// //             0,
// //             array![token_address.into()].span(),
// //             false,
// //         )
// //             .unwrap();

// //         let mut pool = super::PokerGamePool::contract_state_for_testing();

// //         // Test game creation
// //         let game_id = pool.create_game();
// //         assert(game_id == 0, 'First game should have ID 0');

// //         let pot = pool.get_game_pot(game_id);
// //         assert(pot == 0, 'New game should have 0 pot');
// //     }

// //     #[test]
// //     fn test_add_to_pot() {
// //         // Deploy contracts
// //         let (token_address, _) = deploy_syscall(
// //             MockERC20::TEST_CLASS_HASH.try_into().unwrap(), 0, array![].span(), false,
// //         )
// //             .unwrap();

// //         let (pool_address, _) = deploy_syscall(
// //             PokerGamePool::TEST_CLASS_HASH.try_into().unwrap(),
// //             0,
// //             array![token_address.into()].span(),
// //             false,
// //         )
// //             .unwrap();

// //         let pool = IPokerGamePool::contract(pool_address);
// //         let token = IERC20::contract(token_address);

// //         // Create game
// //         let game_id = pool.create_game();

// //         // Approve tokens
// //         token.approve(pool_address, 1000);

// //         // Add to pot
// //         let amount: u256 = 100;
// //         pool.add_to_pot(game_id, amount);

// //         // Verify pot size
// //         let pot = pool.get_game_pot(game_id);
// //         assert(pot == amount, 'Pot size should match deposit');
// //     }

// //     #[test]
// //     fn test_declare_winner() {
// //         // Deploy contracts
// //         let (token_address, _) = deploy_syscall(
// //             MockERC20::TEST_CLASS_HASH.try_into().unwrap(), 0, array![].span(), false,
// //         )
// //             .unwrap();

// //         let (pool_address, _) = deploy_syscall(
// //             PokerGamePool::TEST_CLASS_HASH.try_into().unwrap(),
// //             0,
// //             array![token_address.into()].span(),
// //             false,
// //         )
// //             .unwrap();

// //         let pool = IPokerGamePool::contract(pool_address);
// //         let token = IERC20::contract(token_address);

// //         // Create game
// //         let game_id = pool.create_game();

// //         // Approve and add tokens to pot
// //         token.approve(pool_address, 1000);
// //         pool.add_to_pot(game_id, 100);

// //         // Declare winner
// //         let winner = test_address();
// //         pool.declare_winner(game_id, winner);

// //         // Verify winner received tokens
// //         let winner_balance = token.balance_of(winner);
// //         assert(winner_balance == 100, 'Winner should receive pot');

// //         // Verify pot is empty
// //         let pot = pool.get_game_pot(game_id);
// //         assert(pot == 0, 'Pot should be empty after win');
// //     }

// //     #[test]
// //     #[should_panic(expected: ('Game not active'))]
// //     fn test_add_to_inactive_game() {
// //         // Deploy contracts
// //         let (token_address, _) = deploy_syscall(
// //             MockERC20::TEST_CLASS_HASH.try_into().unwrap(), 0, array![].span(), false,
// //         )
// //             .unwrap();

// //         let (pool_address, _) = deploy_syscall(
// //             PokerGamePool::TEST_CLASS_HASH.try_into().unwrap(),
// //             0,
// //             array![token_address.into()].span(),
// //             false,
// //         )
// //             .unwrap();

// //         let pool = IPokerGamePool::contract(pool_address);
// //         let token = IERC20::contract(token_address);

// //         // Create and finish game
// //         let game_id = pool.create_game();
// //         token.approve(pool_address, 1000);
// //         pool.add_to_pot(game_id, 100);
// //         pool.declare_winner(game_id, test_address());

// //         // Try to add to finished game
// //         pool.add_to_pot(game_id, 100); // Should panic
// //     }

// //     #[test]
// //     #[should_panic(expected: ('Not owner'))]
// //     fn test_unauthorized_winner_declaration() {
// //         // Setup similar to above tests
// //         let (token_address, _) = deploy_syscall(
// //             MockERC20::TEST_CLASS_HASH.try_into().unwrap(), 0, array![].span(), false,
// //         )
// //             .unwrap();

// //         let (pool_address, _) = deploy_syscall(
// //             PokerGamePool::TEST_CLASS_HASH.try_into().unwrap(),
// //             0,
// //             array![token_address.into()].span(),
// //             false,
// //         )
// //             .unwrap();

// //         let pool = IPokerGamePool::contract(pool_address);

// //         // Create game and try to declare winner from non-owner address
// //         let game_id = pool.create_game();
// //         // Call from different address
// //         testing::set_caller_address(test_address());
// //         pool.declare_winner(game_id, test_address()); // Should panic
// //     }
// // }

mod erc20;