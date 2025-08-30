#!/bin/bash

echo "🚀 Starting MySQL benchmark comparison..."

# Start MySQL in devenv environment
echo "📦 Starting devenv environment..."
devenv up &
DEVENV_PID=$!

# Wait for MySQL to start
echo "⏳ Waiting for MySQL to start..."
sleep 15

# Database setup
echo "🔧 Setting up database..."
devenv shell -c "mysql -u root -e 'CREATE DATABASE IF NOT EXISTS structure_comparison; CREATE USER IF NOT EXISTS \"dev\"@\"localhost\" IDENTIFIED BY \"dev\"; GRANT ALL PRIVILEGES ON structure_comparison.* TO \"dev\"@\"localhost\"; FLUSH PRIVILEGES;'"

# Application startup
echo "🖥️  Starting application..."
devenv shell -c "cargo run" &
APP_PID=$!

# Wait for app to start
sleep 10

echo "📊 Running benchmarks..."

# Test data generation
echo "🔧 Generating test data..."
curl -X POST http://localhost:3000/generate/column/10000
curl -X POST http://localhost:3000/generate/json/10000

echo "⚡ Running performance tests..."

# Column storage benchmark
echo "📈 Column storage benchmark..."
for i in {1..5}; do
    curl -s http://localhost:3000/benchmark/column/1000 | jq '.duration_ms'
done

# JSON storage benchmark  
echo "📈 JSON storage benchmark..."
for i in {1..5}; do
    curl -s http://localhost:3000/benchmark/json/1000 | jq '.duration_ms'
done

# Cleanup
kill $APP_PID $DEVENV_PID

echo "✅ Benchmark complete!"