#[starknet::contract]
mod cairo_token {
    use starknet::ContractAddress;
    use starknet::get_caller_address;

    use core::starknet::storage::{
        StorageMapReadAccess, StorageMapWriteAccess, Map, StoragePointerReadAccess,
        StoragePointerWriteAccess,
    };

    #[derive(Drop, Serde, starknet::Store)]
    struct Game {
        pot_size: u256,
        is_active: bool,
    }

    #[storage]
    struct Storage {
        owner: ContractAddress,
        name: felt252,
        symbol: felt252,
        total_supply: u256,
        decimal: u8,
        balances: Map::<ContractAddress, u256>,
        allowances: Map::<(ContractAddress, ContractAddress), u256>,
        games: Map::<u256, Game>,
        current_game_id: u256,
    }

    #[constructor]
    fn constructor(
        ref self: ContractState,
    ) {
        self.name.write(5576989535782658923);
        self.symbol.write(1296388687);
        self.decimal.write(18);
        self.owner.write(get_caller_address());
    }

    #[external(v0)]
    #[generate_trait]
    impl CairoTokenTraitImpl of CairoTokenTrait {
        fn name(self: @ContractState) -> felt252 {
            self.name.read()
        }

        fn owner(self: @ContractState) -> ContractAddress {
            self.owner.read()
        }

        fn symbol(self: @ContractState) -> felt252 {
            self.symbol.read()
        }

        fn totalSupply(self: @ContractState) -> u256 {
            self.total_supply.read()
        }

        fn mint(ref self: ContractState, to: ContractAddress, amount: u256) {
            // assert(get_caller_address() == self.owner.read(), 'Invalid caller');
            let new_total_supply = self.total_supply.read() + amount;
            self.total_supply.write(new_total_supply);
            let new_balance = self.balances.read(to) + amount;
            self.balances.write(to, new_balance);
        }

        fn transfer(ref self: ContractState, to: ContractAddress, amount: u256) {
            let caller: ContractAddress = get_caller_address();
            self._transfer(caller, to, amount);
        }

        fn transferFrom(
            ref self: ContractState, sender: ContractAddress, to: ContractAddress, amount: u256,
        ) {
            let caller = get_caller_address();
            assert(self.allowances.read((sender, caller)) >= amount, 'No allowance');
            self
                .allowances
                .write((sender, caller), self.allowances.read((sender, caller)) - amount);
            self._transfer(sender, to, amount);
        }

        fn approve(ref self: ContractState, spender: ContractAddress, amount: u256) {
            let caller: ContractAddress = get_caller_address();
            let mut prev_allowance: u256 = self.allowances.read((caller, spender));
            self.allowances.write((caller, spender), prev_allowance + amount);
        }

        fn allowance(
            self: @ContractState, owner: ContractAddress, spender: ContractAddress,
        ) -> u256 {
            self.allowances.read((owner, spender))
        }

        fn balanceOf(self: @ContractState, account: ContractAddress) -> u256 {
            self.balances.read(account)
        }

        fn create_game(ref self: ContractState) {
            let caller: ContractAddress = get_caller_address();
            assert(self.owner.read() == caller, 'Invalid caller');
            let new_game_id = self.current_game_id.read() + 1;
            self.games.write(new_game_id, Game { pot_size: 0, is_active: true });
            self.current_game_id.write(new_game_id);
        }

        fn add_to_pot(ref self: ContractState, gameId: u256, amount: u256) {
            let caller: ContractAddress = get_caller_address();
            assert(self.balances.read(caller) >= amount, 'Insufficient balance');
            let mut game = self.games.read(gameId);
            assert(game.is_active, 'Game is not active');
            self.balances.write(caller, self.balances.read(caller) - amount);
            let new_pot_size = game.pot_size + amount;
            self.games.write(gameId, Game { pot_size: new_pot_size, is_active: true });
        }

        fn declare_winner(ref self: ContractState, game_id: u256, winner: ContractAddress) {
            assert(self.games.read(game_id).is_active, 'Game is not active');
            let pot_size = self.games.read(game_id).pot_size;
            self.balances.write(winner, self.balances.read(winner) + pot_size);
            self.games.write(game_id, Game { pot_size: 0, is_active: false });
        }

        fn get_game(self: @ContractState, game_id: u256) -> Game {
            self.games.read(game_id)
        }

        fn get_current_game_id(self: @ContractState) -> u256 {
            self.current_game_id.read()
        }
    }

    #[generate_trait]
    impl PrivateFunctions of CairoTokenPrivateFunctionsTrait {
        fn _transfer(
            ref self: ContractState,
            sender: ContractAddress,
            recipient: ContractAddress,
            amount: u256,
        ) {
            assert(self.balances.read(sender) >= amount, 'Insufficient bal');
            self.balances.write(recipient, self.balances.read(recipient) + amount);
            self.balances.write(sender, self.balances.read(sender) - amount)
        }
    }
}
