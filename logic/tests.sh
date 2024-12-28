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

# Add more test functions as needed...

# Main test runner
main() {
    echo "Starting tests..."

    # Add your test functions here
    run_test test_get_active_player
    run_test test_set_active_player

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