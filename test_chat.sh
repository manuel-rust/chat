#!/bin/bash
# Test script para Chat Hexagonal

echo "=== Build ===" 
cargo build --release

echo ""
echo "=== Starting Chat Server in background ===" 
cargo run --release -- server &
SERVER_PID=$!

sleep 2

echo ""
echo "=== Running Chat Client ===" 
cargo run --release -- client << EOF
/send alice "Hello from Alice"
/send bob "Hi Alice, this is Bob"
/send alice "Nice to meet you"
/fetch 10
/exit
EOF

echo ""
echo "=== Checking stored messages ===" 
if [ -f "data/chat_messages.json" ]; then
    echo "Messages stored in data/chat_messages.json:"
    cat data/chat_messages.json | jq .
else
    echo "No messages file found"
fi

# Kill server
kill $SERVER_PID 2>/dev/null

echo ""
echo "=== Test Complete ===" 
