use sqlx::SqlitePool;
use uuid::Uuid;
use serde_json::json;
use std::collections::HashMap;

pub async fn generate_test_data_column(pool: &SqlitePool, count: i32) -> anyhow::Result<()> {
    for i in 1..=count {
        let user_id = Uuid::new_v4().to_string();
        let preferences = json!({
            "theme": if i % 2 == 0 { "dark" } else { "light" },
            "language": match i % 3 { 0 => "ja", 1 => "en", _ => "es" },
            "notifications": if i % 4 == 0 { "true" } else { "false" }
        });
        let social_links = json!([
            format!("https://twitter.com/user{}", i),
            format!("https://github.com/user{}", i)
        ]);
        
        sqlx::query!(
            r#"
            INSERT INTO users_column (id, name, email, age, bio, avatar_url, preferences, social_links)
            VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8)
            "#,
            user_id,
            format!("User {}", i),
            format!("user{}@example.com", i),
            20 + (i % 60),
            format!("Bio for user {}", i),
            if i % 3 == 0 { Some(format!("https://example.com/avatar{}.jpg", i)) } else { None },
            preferences.to_string(),
            social_links.to_string()
        )
        .execute(pool)
        .await?;
    }
    Ok(())
}

pub async fn generate_test_data_json(pool: &SqlitePool, count: i32) -> anyhow::Result<()> {
    for i in 1..=count {
        let user_id = Uuid::new_v4().to_string();
        let user_data = json!({
            "id": user_id,
            "name": format!("User {}", i),
            "email": format!("user{}@example.com", i),
            "age": 20 + (i % 60),
            "profile": {
                "bio": format!("Bio for user {}", i),
                "avatar_url": if i % 3 == 0 { Some(format!("https://example.com/avatar{}.jpg", i)) } else { None },
                "preferences": {
                    "theme": if i % 2 == 0 { "dark" } else { "light" },
                    "language": match i % 3 { 0 => "ja", 1 => "en", _ => "es" },
                    "notifications": if i % 4 == 0 { "true" } else { "false" }
                },
                "social_links": [
                    format!("https://twitter.com/user{}", i),
                    format!("https://github.com/user{}", i)
                ]
            },
            "created_at": "2024-08-30T08:00:00Z"
        });
        
        sqlx::query!(
            r#"
            INSERT INTO users_json (id, data)
            VALUES (?1, ?2)
            "#,
            user_id,
            user_data.to_string()
        )
        .execute(pool)
        .await?;
    }
    Ok(())
}