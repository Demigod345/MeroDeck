#!/bin/bash
set -e

# Load environment variables from the file
source node_vars.env

# Test results tracking
declare -a TEST_RESULTS

# Function to run a test and capture the result
run_test() {
    local test_name=$1
    echo "Running test: $test_name"
    if $test_name; then
        echo "✅ Test Passed: $test_name"
        TEST_RESULTS+=("✅ $test_name")
    else
        echo "❌ Test Failed: $test_name"
        TEST_RESULTS+=("❌ $test_name")
    fi
}

# Test to check if get_active_player is working
test_get_active_player() {
    local method_name="get_active_player"
    local expected_output=0

    # Run the command with JSON output
    OUTPUT=$(meroctl --output-format json --node-name "$NODE_NAME" call --as "$MEMBER_PUBLIC_KEY" "$CONTEXT_ID" "$method_name")
    # echo "Raw Output: $OUTPUT"

    # Parse the JSON output and extract the result
    RETURN_VALUE=$(echo "$OUTPUT" | jq -r '.result.output')

    # Validate against the expected value
    if [[ "$RETURN_VALUE" == "$expected_output" ]]; then
        return 0
    else
        echo "Expected $expected_output, but got $RETURN_VALUE"
        return 1
    fi
}

# Test to check if set_active_player is working
test_set_active_player() {
    local set_method="set_active_player"
    local get_method="get_active_player"
    local new_value=2

    # Set the active player
    meroctl --node-name "$NODE_NAME" call --args "{\"request\":{\"new_player\": $new_value}}" --as "$MEMBER_PUBLIC_KEY" "$CONTEXT_ID" "$set_method" 

    # Get the active player
    OUTPUT=$(meroctl --output-format json --node-name "$NODE_NAME" call --as "$MEMBER_PUBLIC_KEY" "$CONTEXT_ID" get_active_player)
    RETURN_VALUE=$(echo "$OUTPUT" | jq -r '.result.output')

    if [[ "$RETURN_VALUE" == "$new_value" ]] ; then
        return 0
    else
        echo "Expected $new_value, but got $RETURN_VALUE"
        return 1
    fi
}

# Test to check if join_game is working
test_join_game_initial() {

    # Joining the first player
    meroctl  --node-name "$NODE_NAME" call --args "{\"request\":{\"public_key\": \"$MEMBER_PUBLIC_KEY\"}}" --as "$MEMBER_PUBLIC_KEY" "$CONTEXT_ID" join_game

    #Get game state
    OUTPUT=$(meroctl --output-format json --node-name "$NODE_NAME" call --as "$MEMBER_PUBLIC_KEY" "$CONTEXT_ID" get_game_state)
    PLAYERS=$(echo "$OUTPUT" | jq -r '.result.output.players')
    if [[ -z "$PLAYERS" ]]; then
        echo "No players found in the game state."
        return 1
    fi
    
    FIRST_PLAYER_PK=$(echo "$OUTPUT" | jq -r '.result.output.players[0].public_key')

    if [[ "$FIRST_PLAYER_PK" == "$MEMBER_PUBLIC_KEY" ]] ; then
        return 0
    else
        echo "Expected $MEMBER_PUBLIC_KEY, but got $FIRST_PLAYER_PK"
        return 1
    fi
    
}

# Join 3 other players here considering there are 3 other 
# See currently you don't even need more than one node to test functionality, you can do that by one node only

test_game_join_4players() {
    # Joining the second player
    meroctl  --node-name "$NODE_NAME" call --args "{\"request\":{\"public_key\": \"2222222\"}}" --as "$MEMBER_PUBLIC_KEY" "$CONTEXT_ID" join_game

    #Get game state
    OUTPUT=$(meroctl --output-format json --node-name "$NODE_NAME" call --as "$MEMBER_PUBLIC_KEY" "$CONTEXT_ID" get_game_state)
    PLAYERS=$(echo "$OUTPUT" | jq -r '.result.output.players')

    PLAYER_COUNT=$(echo "$PLAYERS" | jq length)

    if [[ "$PLAYER_COUNT" -ne 2 ]]; then
        echo "Expected 2 players, but got ${#PLAYERS[@]}"
        echo "Players: $PLAYERS"
        return 1
    fi

    SECOND_PLAYER_PK=$(echo "$OUTPUT" | jq -r '.result.output.players[1].public_key')

    if [[ "$SECOND_PLAYER_PK" == "2222222" ]] ; then
        echo "Second player joined successfully."
    else
        echo "Expected 2222222, but got $SECOND_PLAYER_PK"
        return 1
    fi

    # Joining the third player
    meroctl  --node-name "$NODE_NAME" call --args "{\"request\":{\"public_key\": \"3333333\"}}" --as "$MEMBER_PUBLIC_KEY" "$CONTEXT_ID" join_game

    #Get game state
    OUTPUT=$(meroctl --output-format json --node-name "$NODE_NAME" call --as "$MEMBER_PUBLIC_KEY" "$CONTEXT_ID" get_game_state)
    PLAYERS=$(echo "$OUTPUT" | jq -r '.result.output.players')

    PLAYER_COUNT=$(echo "$PLAYERS" | jq length)

    if [[ "$PLAYER_COUNT" -ne 3 ]]; then
        echo "Expected 3 players, but got $PLAYER_COUNT"
        return 1
    fi

    THIRD_PLAYER_PK=$(echo "$OUTPUT" | jq -r '.result.output.players[2].public_key')

    if [[ "$THIRD_PLAYER_PK" == "3333333" ]] ; then
        echo "Third player joined successfully."
    else
        echo "Expected 3333333, but got $THIRD_PLAYER_PK"
        return 1
    fi

    # Joining the fourth player
    meroctl  --node-name "$NODE_NAME" call --args "{\"request\":{\"public_key\": \"4444444\"}}" --as "$MEMBER_PUBLIC_KEY" "$CONTEXT_ID" join_game

    #Get game state
    OUTPUT=$(meroctl --output-format json --node-name "$NODE_NAME" call --as "$MEMBER_PUBLIC_KEY" "$CONTEXT_ID" get_game_state)
    PLAYERS=$(echo "$OUTPUT" | jq -r '.result.output.players')

    PLAYER_COUNT=$(echo "$PLAYERS" | jq length)

    if [[ "$PLAYER_COUNT" -ne 4 ]]; then
        echo "Expected 4 players, but got $PLAYER_COUNT"
        return 1
    fi

    FOURTH_PLAYER_PK=$(echo "$OUTPUT" | jq -r '.result.output.players[3].public_key')

    if [[ "$FOURTH_PLAYER_PK" == "4444444" ]] ; then
        echo "Fourth player joined successfully."
    else
        echo "Expected 4444444, but got $FOURTH_PLAYER_PK"
        return 1
    fi

    return 0
}

# Now play a game with 4 players

# Starting the game
test_start_game() {
    meroctl  --node-name "$NODE_NAME" call --as "$MEMBER_PUBLIC_KEY" "$CONTEXT_ID" start_game

    #Get game state
    OUTPUT=$(meroctl --output-format json --node-name "$NODE_NAME" call --as "$MEMBER_PUBLIC_KEY" "$CONTEXT_ID" get_game_state)
    GAME_STATE=$(echo "$OUTPUT" | jq -r '.result.output')
    PLAYERS=$(echo "$OUTPUT" | jq -r '.result.output.players')
    COMM_CARDS=$(echo "$OUTPUT" | jq -r '.result.output.community_cards')

    echo "Players are ..."
    echo "$PLAYERS"

    echo "Community cards are ..."
    echo "$COMM_CARDS"

    return 0

    
}

set_testing_cards() {
    
    meroctl --node-name "$NODE_NAME" call --args "{\"request\":{\"player_index\":0,\"cards\":[{\"suit\":\"Spades\",\"rank\":\"Ace\"},{\"suit\":\"Hearts\",\"rank\":\"Seven\"}]}}" --as "$MEMBER_PUBLIC_KEY" "$CONTEXT_ID" set_player_cards
    meroctl --node-name "$NODE_NAME" call --args "{\"request\":{\"player_index\":1,\"cards\":[{\"suit\":\"Diamonds\",\"rank\":\"King\"},{\"suit\":\"Clubs\",\"rank\":\"Ten\"}]}}" --as "$MEMBER_PUBLIC_KEY" "$CONTEXT_ID" set_player_cards
    meroctl --node-name "$NODE_NAME" call --args "{\"request\":{\"player_index\":2,\"cards\":[{\"suit\":\"Hearts\",\"rank\":\"Queen\"},{\"suit\":\"Spades\",\"rank\":\"Jack\"}]}}" --as "$MEMBER_PUBLIC_KEY" "$CONTEXT_ID" set_player_cards
    meroctl --node-name "$NODE_NAME" call --args "{\"request\":{\"player_index\":3,\"cards\":[{\"suit\":\"Diamonds\",\"rank\":\"Nine\"},{\"suit\":\"Hearts\",\"rank\":\"Eight\"}]}}" --as "$MEMBER_PUBLIC_KEY" "$CONTEXT_ID" set_player_cards

    # Set the community cards
    meroctl --node-name "$NODE_NAME" call --args "{\"cards\":[{\"suit\":\"Hearts\",\"rank\":\"Ace\"},{\"suit\":\"Hearts\",\"rank\":\"King\"},{\"suit\":\"Spades\",\"rank\":\"Nine\"},{\"suit\":\"Hearts\",\"rank\":\"Ten\"},{\"suit\":\"Hearts\",\"rank\":\"Jack\"}]}" --as "$MEMBER_PUBLIC_KEY" "$CONTEXT_ID" set_community_cards

    # Get the game state
    OUTPUT=$(meroctl --output-format json --node-name "$NODE_NAME" call --as "$MEMBER_PUBLIC_KEY" "$CONTEXT_ID" get_game_state)
    COMMUNITY_CARDS=$(echo "$OUTPUT" | jq -r '.result.output.community_cards')

    # Expected community cards
    EXPECTED_CARDS='[{"rank":"Ace","suit":"Hearts"},{"rank":"King","suit":"Hearts"},{"rank":"Nine","suit":"Spades"},{"rank":"Ten","suit":"Hearts"},{"rank":"Jack","suit":"Hearts"}]'

    # Parse both expected and actual community cards to JSON and compare
    EXPECTED_CARDS_JSON=$(echo "$EXPECTED_CARDS" | jq -c .)
    COMMUNITY_CARDS_JSON=$(echo "$COMMUNITY_CARDS" | jq -c .)

    if [[ "$COMMUNITY_CARDS_JSON" == "$EXPECTED_CARDS_JSON" ]]; then
        echo "Community cards are set correctly." # This is happening now
    else
        echo "Community cards do not match the expected values."
        echo "Expected: $EXPECTED_CARDS_JSON"
        echo "Got: $COMMUNITY_CARDS_JSON"
        return 1
    fi

    # Get the player cards
    for i in {0..3}; do
        #Get game state
        OUTPUT=$(meroctl --output-format json --node-name "$NODE_NAME" call --as "$MEMBER_PUBLIC_KEY" "$CONTEXT_ID" get_game_state)
        PLAYER_CARDS=$(echo "$OUTPUT" | jq -r '.result.output.players['"$i"'].cards')

        case $i in
            0)
                EXPECTED_CARDS='[{"rank":"Ace","suit":"Spades"},{"rank":"Seven","suit":"Hearts"}]'
                ;;
            1)
                EXPECTED_CARDS='[{"rank":"King","suit":"Diamonds"},{"rank":"Ten","suit":"Clubs"}]'
                ;;
            2)
                EXPECTED_CARDS='[{"rank":"Queen","suit":"Hearts"},{"rank":"Jack","suit":"Spades"}]'
                ;;
            3)
                EXPECTED_CARDS='[{"rank":"Nine","suit":"Diamonds"},{"rank":"Eight","suit":"Hearts"}]'
                ;;
        esac

        EXPECTED_CARDS_JSON=$(echo "$EXPECTED_CARDS" | jq -c .)
        PLAYER_CARDS_JSON=$(echo "$PLAYER_CARDS" | jq -c .)

        if [[ "$PLAYER_CARDS_JSON" == "$EXPECTED_CARDS_JSON" ]]; then
            echo "Player $i cards are set correctly."
        else
            echo "Player $i cards do not match the expected values."
            echo "Expected: $EXPECTED_CARDS_JSON"
            echo "Got: $PLAYER_CARDS_JSON"
            return 1
        fi
    done

    return 0
}

# Now statrt playing turns

test_first_action() {
    # First player action
    meroctl --node-name "$NODE_NAME" call --args "{\"request\":{\"player_index\":0,\"action\":{\"Bet\":2}}}" --as "$MEMBER_PUBLIC_KEY" "$CONTEXT_ID" process_action

    # Get the game state
    OUTPUT=$(meroctl --output-format json --node-name "$NODE_NAME" call --as "$MEMBER_PUBLIC_KEY" "$CONTEXT_ID" get_game_state)
    GAME_STATE=$(echo "$OUTPUT" | jq -r '.result.output')
    ROUND_BETS=$(echo "$OUTPUT" | jq -r '.result.output.round_bets')
    CHECKED_PLAYERS=$(echo "$OUTPUT" | jq -r '.result.output.checked_players')
    ACTIVE_PLAYER=$(echo "$OUTPUT" | jq -r '.result.output.action_position')
    POT_SIZE=$(echo "$OUTPUT" | jq -r '.result.output.pot')

    echo "Betting stage is ..."
    echo "$ROUND_BETS"

    echo "Checked players are ..."
    echo "$CHECKED_PLAYERS"

    echo "Current active position is $ACTIVE_PLAYER and pot size is $POT_SIZE"


    return 0
}


get_game_details() {
    # Get the game state
    OUTPUT=$(meroctl --output-format json --node-name "$NODE_NAME" call --as "$MEMBER_PUBLIC_KEY" "$CONTEXT_ID" get_game_state)
    GAME_STATE=$(echo "$OUTPUT" | jq -r '.result.output')
    ROUND_BETS=$(echo "$OUTPUT" | jq -r '.result.output.round_bets')
    CHECKED_PLAYERS=$(echo "$OUTPUT" | jq -r '.result.output.checked_players')
    ACTIVE_PLAYER=$(echo "$OUTPUT" | jq -r '.result.output.action_position')
    POT_SIZE=$(echo "$OUTPUT" | jq -r '.result.output.pot')
    GAMEPHASE=$(echo "$OUTPUT" | jq -r '.result.output.phase')
    PLAYERS_MOVE=$(echo "$OUTPUT" | jq -r '.result.output.players')

    echo "Betting stage is ..."
    echo "$ROUND_BETS"

    echo "Checked players are ..."
    echo "$CHECKED_PLAYERS"

    echo "Moves are ..."
    echo "$PLAYERS_MOVE"

    echo "Current active position is $ACTIVE_PLAYER and pot size is $POT_SIZE"
    echo "Game phase is $GAMEPHASE"
}


test_first_betting_round() {
    #Second player action

    # Capture the response
    BET_RESPONSE=$(meroctl --node-name "$NODE_NAME" --output-format json call --args "{\"request\":{\"player_index\":1,\"action\":{\"Bet\":2}}}" --as "$MEMBER_PUBLIC_KEY" "$CONTEXT_ID" process_action)

    # Check if there is an error in the response
    ERROR_DATA=$(echo "$BET_RESPONSE" | jq -r '.error.data // empty')
    if [[ -n "$ERROR_DATA" ]]; then
        # Extract the array of ASCII values and convert to plain text
        ASCII_ARRAY=$(echo "$ERROR_DATA" | grep -o '\[.*\]')
        PLAIN_TEXT=$(echo "$ASCII_ARRAY" | jq -r '.[]' | awk '{printf "%c", $1}' | xargs)
        if [[ "$PLAIN_TEXT" == "Cannot bet when there's a bet" ]]; then
            echo "Cannot bet after betting is working"
        else
            echo "Expected error: \"Cannot bet when there's a bet\", got error: $PLAIN_TEXT"
            return 1
        fi
    fi

    # Sending correct player action
    echo "Player 2 raises to 4"
    meroctl --node-name "$NODE_NAME" call --args "{\"request\":{\"player_index\":1,\"action\":{\"Raise\":4}}}" --as "$MEMBER_PUBLIC_KEY" "$CONTEXT_ID" process_action
    # Get the game state
    get_game_details

    # Doing the action of third player
    echo "Player 3 raises to 8"
    meroctl --node-name "$NODE_NAME" call --args "{\"request\":{\"player_index\":2,\"action\":{\"Raise\":8}}}" --as "$MEMBER_PUBLIC_KEY" "$CONTEXT_ID" process_action
    # Get the game state
    get_game_details

    # Doing the action of fourth player
    echo "Player 4 calls 8"
    meroctl --node-name "$NODE_NAME" call --args "{\"request\":{\"player_index\":3,\"action\":\"Call\"}}" --as "$MEMBER_PUBLIC_KEY" "$CONTEXT_ID" process_action

    get_game_details

    # Player 1 folds
    echo "Player 1 folds"
    meroctl --node-name "$NODE_NAME" call --args "{\"request\":{\"player_index\":0,\"action\":\"Fold\"}}" --as "$MEMBER_PUBLIC_KEY" "$CONTEXT_ID" process_action

    get_game_details

    # Player 2 calls
    echo "Player 2 calls 8"
    meroctl --node-name "$NODE_NAME" call --args "{\"request\":{\"player_index\":1,\"action\":\"Call\"}}" --as "$MEMBER_PUBLIC_KEY" "$CONTEXT_ID" process_action

    get_game_details
    return 0

}


test_flop_round() {
    echo "=======================Player 2 Bets 10 chips============================="
    meroctl --node-name "$NODE_NAME" call --args "{\"request\":{\"player_index\":1,\"action\":{\"Bet\":10}}}" --as "$MEMBER_PUBLIC_KEY" "$CONTEXT_ID" process_action

    get_game_details

    echo "========================Player 3 raises to 20 chips========================"
    meroctl --node-name "$NODE_NAME" call --args "{\"request\":{\"player_index\":2,\"action\":{\"Raise\":20}}}" --as "$MEMBER_PUBLIC_KEY" "$CONTEXT_ID" process_action

    get_game_details

    echo "========================Player 4 calls 20 chips==========================="
    meroctl --node-name "$NODE_NAME" call --args "{\"request\":{\"player_index\":3,\"action\":\"Call\"}}" --as "$MEMBER_PUBLIC_KEY" "$CONTEXT_ID" process_action

    get_game_details

    echo "========================Player 2 calls 20 chips (10 more chips)==========================="
    meroctl --node-name "$NODE_NAME" call --args "{\"request\":{\"player_index\":1,\"action\":\"Call\"}}" --as "$MEMBER_PUBLIC_KEY" "$CONTEXT_ID" process_action

    get_game_details # The game phase should be turn now
}

test_turn_round() {
    echo "=======================Player 2 Bets 20 chips============================="
    meroctl --node-name "$NODE_NAME" call --args "{\"request\":{\"player_index\":1,\"action\":{\"Bet\":20}}}" --as "$MEMBER_PUBLIC_KEY" "$CONTEXT_ID" process_action

    get_game_details

    echo "========================Player 3 raises to 40 chips========================"
    meroctl --node-name "$NODE_NAME" call --args "{\"request\":{\"player_index\":2,\"action\":{\"Raise\":40}}}" --as "$MEMBER_PUBLIC_KEY" "$CONTEXT_ID" process_action

    get_game_details

    echo "========================Player 4 folds==========================="
    meroctl --node-name "$NODE_NAME" call --args "{\"request\":{\"player_index\":3,\"action\":\"Fold\"}}" --as "$MEMBER_PUBLIC_KEY" "$CONTEXT_ID" process_action

    get_game_details

    echo "========================Player 2 calls 40 chips (adding 20 more chips)==========================="
    meroctl --node-name "$NODE_NAME" call --args "{\"request\":{\"player_index\":1,\"action\":\"Call\"}}" --as "$MEMBER_PUBLIC_KEY" "$CONTEXT_ID" process_action

    get_game_details # The game phase should be river now
}

test_river_round() {
    echo "=======================Plyer 2 checks============================="
    meroctl --node-name "$NODE_NAME" call --args "{\"request\":{\"player_index\":1,\"action\":\"Check\"}}" --as "$MEMBER_PUBLIC_KEY" "$CONTEXT_ID" process_action

    get_game_details

    echo "========================Player 3 bets 50 chips========================"
    meroctl --node-name "$NODE_NAME" call --args "{\"request\":{\"player_index\":2,\"action\":{\"Bet\":50}}}" --as "$MEMBER_PUBLIC_KEY" "$CONTEXT_ID" process_action

    get_game_details

    echo "========================Player 2 calls 50 chips (adding 50 chips)==========================="
    meroctl --node-name "$NODE_NAME" call --args "{\"request\":{\"player_index\":1,\"action\":\"Call\"}}" --as "$MEMBER_PUBLIC_KEY" "$CONTEXT_ID" process_action

    get_game_details # The game phase should be showdown now
    OUTPUT=$(meroctl --output-format json --node-name "$NODE_NAME" call --as "$MEMBER_PUBLIC_KEY" "$CONTEXT_ID" get_game_state)
    WINNER=$(echo "$OUTPUT" | jq -r '.result.output.winner')
    echo "Winner is $WINNER"
}



# Add more test functions as needed...

# Main test runner
main() {
    echo "Starting tests..."

    # Add your test functions here
    run_test test_get_active_player
    run_test test_set_active_player
    run_test test_join_game_initial
    run_test test_game_join_4players
    run_test test_start_game
    run_test set_testing_cards
    run_test test_first_action
    run_test test_first_betting_round
    run_test test_flop_round
    run_test test_turn_round
    run_test test_river_round

    echo ""
    echo "Test Summary:"
    for result in "${TEST_RESULTS[@]}"; do
        echo "$result"
    done

    # Fail the script if any test failed
    if [[ "${TEST_RESULTS[*]}" == *"❌"* ]]; then
        echo "Some tests failed."
        exit 1
    fi

    echo "All tests passed successfully!"
}

# Run the main function
main