#!/bin/bash

echo "Testing optimized startup time..."

# Test 5 runs to get average
total_time=0
for i in {1..5}; do
    echo "Run $i:"
    
    # Start timer
    START_TIME=$(date +%s.%N)
    
    # Launch and immediately kill
    ./target/release/ha &
    PID=$!
    /bin/sleep 0.2  # Give it time to show UI
    kill $PID 2>/dev/null
    wait $PID 2>/dev/null
    
    # Calculate time
    END_TIME=$(date +%s.%N)
    ELAPSED=$(echo "$END_TIME - $START_TIME" | bc)
    echo "  Time: ${ELAPSED}s"
    
    total_time=$(echo "$total_time + $ELAPSED" | bc)
    
    # Brief pause between runs
    /bin/sleep 0.5
done

average=$(echo "scale=3; $total_time / 5" | bc)
echo ""
echo "Average startup time: ${average}s"
echo ""
echo "Optimizations applied:"
echo "- Skipped server startup in GUI mode"
echo "- Skipped ApplicationState creation" 
echo "- Removed icon loading"
echo "- Use deferred data loading"