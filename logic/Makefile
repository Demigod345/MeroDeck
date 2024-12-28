# Variables
NODE_SCRIPT := script.sh
TEST_SCRIPT := tests.sh

# Default target
.PHONY: all
all: init-node run-tests

# Target to initialize the node
.PHONY: init-node
init-node:
	@echo "Initializing the node..."
	@bash $(NODE_SCRIPT)

# Target to run the tests
.PHONY: run-tests
run-tests:
	@echo "Running tests..."
	@bash $(TEST_SCRIPT)

# Target to execute both scripts
.PHONY: run-all
run-all: init-node run-tests
	@echo "Node initialization and tests completed successfully."

# Clean target (optional, e.g., for cleaning generated files or logs)
.PHONY: clean
clean:
	@echo "Cleaning up..."
	@rm -f node_vars.env
	@rm -rf res/
