#!/bin/bash

echo "=== Testing action_grab function ==="

# Create a temporary action with grab type and G flag
echo "Testing grab action via command server..."

# Use the execute_on_server_with_parameters or create action manually
# Since we have the action_grab function in JavaScript, let's test it by calling it as an action
~/ob/proj/HookAnchor/target/release/ha -a grab

echo "=== Test completed ==="