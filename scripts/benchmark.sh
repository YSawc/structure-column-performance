#!/bin/bash

echo "🚀 Starting performance comparison..."

# Server startup verification
echo "Checking if server is running..."
if ! curl -s http://localhost:3000/users/column > /dev/null; then
    echo "❌ Server is not running. Please start with: cargo run"
    exit 1
fi

echo "✅ Server is running"

echo ""
echo "📊 Running performance tests..."

# Column storage benchmark
echo "Testing column storage (100 records)..."
curl -s "http://localhost:3000/benchmark/column/100" | jq

echo ""
echo "Testing column storage (1000 records)..."
curl -s "http://localhost:3000/benchmark/column/1000" | jq

echo ""
echo "Testing column storage (5000 records)..."
curl -s "http://localhost:3000/benchmark/column/5000" | jq

echo ""
echo "📄 Testing JSON storage (100 records)..."
curl -s "http://localhost:3000/benchmark/json/100" | jq

echo ""
echo "Testing JSON storage (1000 records)..."
curl -s "http://localhost:3000/benchmark/json/1000" | jq

echo ""
echo "Testing JSON storage (5000 records)..."
curl -s "http://localhost:3000/benchmark/json/5000" | jq

echo ""
echo "🏁 Performance comparison complete!"
echo ""
echo "💡 To run detailed benchmarks with criterion:"
echo "   cargo bench"