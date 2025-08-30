-- Create database and user
CREATE DATABASE IF NOT EXISTS structure_comparison;
CREATE USER IF NOT EXISTS 'dev'@'localhost' IDENTIFIED BY 'dev';
GRANT ALL PRIVILEGES ON structure_comparison.* TO 'dev'@'localhost';
FLUSH PRIVILEGES;

USE structure_comparison;

-- Table creation
-- Column storage (normalized table structure)
CREATE TABLE users_column (
    id CHAR(36) PRIMARY KEY,
    name VARCHAR(255) NOT NULL,
    email VARCHAR(255) NOT NULL UNIQUE,
    age INTEGER NOT NULL,
    bio TEXT,
    avatar_url VARCHAR(500),
    preferences JSON,
    social_links JSON,
    created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP
);

-- JSON storage (denormalized table structure)
CREATE TABLE users_json (
    id CHAR(36) PRIMARY KEY,
    data JSON NOT NULL,
    created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP
);

-- Index creation
CREATE INDEX idx_users_column_created_at ON users_column(created_at);
CREATE INDEX idx_users_column_email ON users_column(email);
CREATE INDEX idx_users_column_age ON users_column(age);

CREATE INDEX idx_users_json_created_at ON users_json(created_at);
CREATE INDEX idx_users_json_email ON users_json((JSON_UNQUOTE(JSON_EXTRACT(data, '$.email'))));
CREATE INDEX idx_users_json_age ON users_json((CAST(JSON_EXTRACT(data, '$.age') AS UNSIGNED)));

-- Sample data generation procedure for performance testing
DELIMITER //

CREATE PROCEDURE generate_test_data_column(IN count_param INT)
BEGIN
    DECLARE i INT DEFAULT 1;
    DECLARE user_id CHAR(36);
    DECLARE user_email VARCHAR(255);
    DECLARE user_age INT;
    DECLARE user_bio TEXT;
    DECLARE user_avatar VARCHAR(500);
    DECLARE user_prefs JSON;
    DECLARE user_links JSON;
    
    WHILE i <= count_param DO
        SET user_id = UUID();
        SET user_email = CONCAT('user', i, '@example.com');
        SET user_age = 20 + (i % 60);
        SET user_bio = CONCAT('Bio for user ', i);
        SET user_avatar = CASE WHEN i % 3 = 0 THEN CONCAT('https://example.com/avatar', i, '.jpg') ELSE NULL END;
        SET user_prefs = JSON_OBJECT(
            'theme', CASE WHEN i % 2 = 0 THEN 'dark' ELSE 'light' END,
            'language', CASE WHEN i % 3 = 0 THEN 'ja' WHEN i % 3 = 1 THEN 'en' ELSE 'es' END,
            'notifications', CASE WHEN i % 4 = 0 THEN 'true' ELSE 'false' END
        );
        SET user_links = JSON_ARRAY(
            CONCAT('https://twitter.com/user', i),
            CONCAT('https://github.com/user', i)
        );
        
        INSERT INTO users_column (id, name, email, age, bio, avatar_url, preferences, social_links, created_at)
        VALUES (user_id, CONCAT('User ', i), user_email, user_age, user_bio, user_avatar, user_prefs, user_links, 
                DATE_SUB(NOW(), INTERVAL i SECOND));
        
        SET i = i + 1;
    END WHILE;
END //

CREATE PROCEDURE generate_test_data_json(IN count_param INT)
BEGIN
    DECLARE i INT DEFAULT 1;
    DECLARE user_id CHAR(36);
    DECLARE user_data JSON;
    
    WHILE i <= count_param DO
        SET user_id = UUID();
        SET user_data = JSON_OBJECT(
            'id', user_id,
            'name', CONCAT('User ', i),
            'email', CONCAT('user', i, '@example.com'),
            'age', 20 + (i % 60),
            'profile', JSON_OBJECT(
                'bio', CONCAT('Bio for user ', i),
                'avatar_url', CASE WHEN i % 3 = 0 THEN CONCAT('https://example.com/avatar', i, '.jpg') ELSE NULL END,
                'preferences', JSON_OBJECT(
                    'theme', CASE WHEN i % 2 = 0 THEN 'dark' ELSE 'light' END,
                    'language', CASE WHEN i % 3 = 0 THEN 'ja' WHEN i % 3 = 1 THEN 'en' ELSE 'es' END,
                    'notifications', CASE WHEN i % 4 = 0 THEN 'true' ELSE 'false' END
                ),
                'social_links', JSON_ARRAY(
                    CONCAT('https://twitter.com/user', i),
                    CONCAT('https://github.com/user', i)
                )
            ),
            'created_at', DATE_SUB(NOW(), INTERVAL i SECOND)
        );
        
        INSERT INTO users_json (id, data, created_at)
        VALUES (user_id, user_data, DATE_SUB(NOW(), INTERVAL i SECOND));
        
        SET i = i + 1;
    END WHILE;
END //

DELIMITER ;