-- Generate test data
SELECT generate_test_data_column(10000);
SELECT generate_test_data_json(10000);

-- Data verification
SELECT 'Column storage:' as info, COUNT(*) as count FROM users_column;
SELECT 'JSON storage:' as info, COUNT(*) as count FROM users_json;

-- Performance verification of sample queries
EXPLAIN ANALYZE 
SELECT id, name, email, age, bio, avatar_url, preferences, social_links, created_at
FROM users_column
ORDER BY created_at DESC
LIMIT 100;

EXPLAIN ANALYZE 
SELECT data
FROM users_json
ORDER BY created_at DESC
LIMIT 100;