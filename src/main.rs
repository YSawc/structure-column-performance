use axum::{
    extract::{Path, Query, State},
    http::StatusCode,
    response::Json,
    routing::{get, post},
    Router,
};
use serde::{Deserialize, Serialize};
use sqlx::{MySqlPool, Row};
use std::collections::HashMap;
use time::OffsetDateTime;
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize)]
struct User {
    id: Uuid,
    name: String,
    email: String,
    age: i32,
    profile: UserProfile,
    created_at: OffsetDateTime,
}

#[derive(Debug, Serialize, Deserialize)]
struct UserProfile {
    bio: String,
    avatar_url: Option<String>,
    preferences: HashMap<String, String>,
    social_links: Vec<String>,
}

#[derive(Debug, Deserialize)]
struct CreateUserRequest {
    name: String,
    email: String,
    age: i32,
    profile: UserProfile,
}

#[derive(Debug, Deserialize)]
struct QueryParams {
    limit: Option<i32>,
}

type AppState = MySqlPool;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let database_url = "mysql://root@localhost:3306/structure_comparison";
    let pool = MySqlPool::connect(database_url).await?;

    // Skip migration (tables already exist)
    // sqlx::migrate!("./migrations").run(&pool).await?;

    let pool_for_app = pool.clone();
    let app = Router::new()
        .route("/users/column", post(create_user_column))
        .route("/users/column", get(get_users_column))
        .route("/users/json", post(create_user_json))
        .route("/users/json", get(get_users_json))
        .route("/benchmark/column/:count", get(benchmark_column))
        .route("/benchmark/json/:count", get(benchmark_json))
        .route("/benchmark/complex/:count", get(benchmark_complex_processing))
        .route("/generate/column/:count", post(generate_column_data))
        .route("/generate/json/:count", post(generate_json_data))
        .route("/generate/complex/:count", post(generate_complex_data))
        .with_state(pool_for_app);

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await?;
    println!("🚀 Server running on http://localhost:3000");
    
    // Automatically generate test data and run benchmarks after server startup
    let pool_for_test = pool.clone();
    tokio::spawn(async move {
        // Wait a bit for server to start
        tokio::time::sleep(tokio::time::Duration::from_secs(2)).await;
        
        println!("📊 Starting automatic test data generation and benchmark execution...");
        
        // Generate test data
        generate_test_data(&pool_for_test).await;
        
        // Run benchmarks
        run_benchmarks(&pool_for_test).await;
        
        println!("✅ Automatic test completed!");
    });
    
    axum::serve(listener, app).await?;

    Ok(())
}

async fn create_user_column(
    State(pool): State<AppState>,
    Json(payload): Json<CreateUserRequest>,
) -> Result<Json<User>, StatusCode> {
    let user_id = Uuid::new_v4();
    let now = OffsetDateTime::now_utc();
    
    // Remove unused variable

    sqlx::query(
        r#"
        INSERT INTO users_column (id, name, email, age, bio, avatar_url, preferences, social_links, created_at)
        VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?)
        "#
    )
    .bind(user_id.to_string())
    .bind(&payload.name)
    .bind(&payload.email)
    .bind(payload.age)
    .bind(&payload.profile.bio)
    .bind(&payload.profile.avatar_url)
    .bind(serde_json::to_string(&payload.profile.preferences).unwrap())
    .bind(serde_json::to_string(&payload.profile.social_links).unwrap())
    .bind(now)
    .execute(&pool)
    .await
    .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    let user = User {
        id: user_id,
        name: payload.name,
        email: payload.email,
        age: payload.age,
        profile: payload.profile,
        created_at: now,
    };

    Ok(Json(user))
}

async fn get_users_column(
    State(pool): State<AppState>,
    Query(params): Query<QueryParams>,
) -> Result<Json<Vec<User>>, StatusCode> {
    let limit = params.limit.unwrap_or(100);

    let rows = sqlx::query(
        r#"
        SELECT id, name, email, age, bio, avatar_url, preferences, social_links, created_at
        FROM users_column
        ORDER BY created_at DESC
        LIMIT ?
        "#
    )
    .bind(limit)
    .fetch_all(&pool)
    .await
    .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    let users: Vec<User> = rows
        .iter()
        .map(|row| {
            let id_str: String = row.get("id");
            let preferences_str: String = row.get("preferences");
            let social_links_str: String = row.get("social_links");
            
            let preferences: HashMap<String, String> = 
                serde_json::from_str(&preferences_str).unwrap_or_default();
            let social_links: Vec<String> = 
                serde_json::from_str(&social_links_str).unwrap_or_default();

            User {
                id: Uuid::parse_str(&id_str).unwrap(),
                name: row.get("name"),
                email: row.get("email"),
                age: row.get("age"),
                profile: UserProfile {
                    bio: row.get("bio"),
                    avatar_url: row.get("avatar_url"),
                    preferences,
                    social_links,
                },
                created_at: row.get("created_at"),
            }
        })
        .collect();

    Ok(Json(users))
}

async fn create_user_json(
    State(pool): State<AppState>,
    Json(payload): Json<CreateUserRequest>,
) -> Result<Json<User>, StatusCode> {
    let user_id = Uuid::new_v4();
    let now = OffsetDateTime::now_utc();
    
    let user = User {
        id: user_id,
        name: payload.name,
        email: payload.email,
        age: payload.age,
        profile: payload.profile,
        created_at: now,
    };

    let user_json = serde_json::to_value(&user)
        .map_err(|_| StatusCode::BAD_REQUEST)?;

    sqlx::query(
        r#"
        INSERT INTO users_json (id, data, created_at)
        VALUES (?, ?, ?)
        "#
    )
    .bind(user_id.to_string())
    .bind(user_json.to_string())
    .bind(now)
    .execute(&pool)
    .await
    .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok(Json(user))
}

async fn get_users_json(
    State(pool): State<AppState>,
    Query(params): Query<QueryParams>,
) -> Result<Json<Vec<User>>, StatusCode> {
    let limit = params.limit.unwrap_or(100);

    let rows = sqlx::query(
        r#"
        SELECT data
        FROM users_json
        ORDER BY created_at DESC
        LIMIT ?
        "#
    )
    .bind(limit)
    .fetch_all(&pool)
    .await
    .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    let users: Vec<User> = rows
        .iter()
        .filter_map(|row| {
            let data_str: String = row.get("data");
            serde_json::from_str(&data_str).ok()
        })
        .collect();

    Ok(Json(users))
}

async fn benchmark_column(
    State(pool): State<AppState>,
    Path(count): Path<i32>,
) -> Result<Json<serde_json::Value>, StatusCode> {
    let start = std::time::Instant::now();
    
    let rows = sqlx::query(
        r#"
        SELECT id, name, email, age, bio, avatar_url, preferences, social_links, created_at
        FROM users_column
        ORDER BY created_at DESC
        LIMIT ?
        "#
    )
    .bind(count)
    .fetch_all(&pool)
    .await
    .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    let _users: Vec<User> = rows
        .iter()
        .map(|row| {
            let id_str: String = row.get("id");
            let preferences_str: String = row.get("preferences");
            let social_links_str: String = row.get("social_links");
            
            let preferences: HashMap<String, String> = 
                serde_json::from_str(&preferences_str).unwrap_or_default();
            let social_links: Vec<String> = 
                serde_json::from_str(&social_links_str).unwrap_or_default();

            User {
                id: Uuid::parse_str(&id_str).unwrap(),
                name: row.get("name"),
                email: row.get("email"),
                age: row.get("age"),
                profile: UserProfile {
                    bio: row.get("bio"),
                    avatar_url: row.get("avatar_url"),
                    preferences,
                    social_links,
                },
                created_at: row.get("created_at"),
            }
        })
        .collect();

    let duration = start.elapsed();

    Ok(Json(serde_json::json!({
        "storage_type": "column",
        "count": count,
        "duration_ms": duration.as_millis(),
        "records_processed": rows.len()
    })))
}

async fn benchmark_json(
    State(pool): State<AppState>,
    Path(count): Path<i32>,
) -> Result<Json<serde_json::Value>, StatusCode> {
    let start = std::time::Instant::now();
    
    let rows = sqlx::query(
        r#"
        SELECT data
        FROM users_json
        ORDER BY created_at DESC
        LIMIT ?
        "#
    )
    .bind(count)
    .fetch_all(&pool)
    .await
    .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    let _users: Vec<User> = rows
        .iter()
        .filter_map(|row| {
            let data_str: String = row.get("data");
            serde_json::from_str(&data_str).ok()
        })
        .collect();

    let duration = start.elapsed();

    Ok(Json(serde_json::json!({
        "storage_type": "json",
        "count": count,
        "duration_ms": duration.as_millis(),
        "records_processed": rows.len()
    })))
}

async fn generate_column_data(
    State(pool): State<AppState>,
    Path(count): Path<i32>,
) -> Result<Json<serde_json::Value>, StatusCode> {
    for i in 1..=count {
        let user_id = Uuid::new_v4().to_string();
        let preferences = serde_json::json!({
            "theme": if i % 2 == 0 { "dark" } else { "light" },
            "language": match i % 3 { 0 => "ja", 1 => "en", _ => "es" },
            "notifications": if i % 4 == 0 { "true" } else { "false" }
        });
        let social_links = serde_json::json!([
            format!("https://twitter.com/user{}", i),
            format!("https://github.com/user{}", i)
        ]);
        
        sqlx::query(
            r#"
            INSERT INTO users_column (id, name, email, age, bio, avatar_url, preferences, social_links)
            VALUES (?, ?, ?, ?, ?, ?, ?, ?)
            "#
        )
        .bind(user_id)
        .bind(format!("User {}", i))
        .bind(format!("user{}@example.com", i))
        .bind(20 + (i % 60))
        .bind(format!("Bio for user {}", i))
        .bind(if i % 3 == 0 { Some(format!("https://example.com/avatar{}.jpg", i)) } else { None })
        .bind(preferences.to_string())
        .bind(social_links.to_string())
        .execute(&pool)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    }

    Ok(Json(serde_json::json!({
        "message": format!("Generated {} records in users_column", count)
    })))
}

async fn generate_json_data(
    State(pool): State<AppState>,
    Path(count): Path<i32>,
) -> Result<Json<serde_json::Value>, StatusCode> {
    for i in 1..=count {
        let user_id = Uuid::new_v4().to_string();
        let user_data = serde_json::json!({
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
        
        sqlx::query(
            r#"
            INSERT INTO users_json (id, data)
            VALUES (?, ?)
            "#
        )
        .bind(user_id)
        .bind(user_data.to_string())
        .execute(&pool)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    }

    Ok(Json(serde_json::json!({
        "message": format!("Generated {} records in users_json", count)
    })))
}

// Function for automatic testing
async fn generate_test_data(pool: &MySqlPool) {
    println!("📝 Generating test data...");
    
    // Clear existing data
    if let Err(e) = sqlx::query("DELETE FROM users_column").execute(pool).await {
        eprintln!("Column table clear error: {}", e);
    }
    if let Err(e) = sqlx::query("DELETE FROM users_json").execute(pool).await {
        eprintln!("JSON table clear error: {}", e);
    }
    println!("🗑️ Cleared existing data");
    
    // Generate data for column table
    for i in 1..=100000 {
        let user_id = Uuid::new_v4().to_string();
        let preferences = serde_json::json!({
            "theme": if i % 2 == 0 { "dark" } else { "light" },
            "language": match i % 3 { 0 => "ja", 1 => "en", _ => "es" },
            "notifications": if i % 4 == 0 { "true" } else { "false" }
        });
        let social_links = serde_json::json!([
            format!("https://twitter.com/user{}", i),
            format!("https://github.com/user{}", i)
        ]);
        
        if let Err(e) = sqlx::query(
            r#"
            INSERT INTO users_column (id, name, email, age, bio, avatar_url, preferences, social_links)
            VALUES (?, ?, ?, ?, ?, ?, ?, ?)
            "#
        )
        .bind(user_id)
        .bind(format!("User {}", i))
        .bind(format!("user{}@example.com", i))
        .bind(20 + (i % 60))
        .bind(format!("Bio for user {}", i))
        .bind(if i % 3 == 0 { Some(format!("https://example.com/avatar{}.jpg", i)) } else { None })
        .bind(preferences.to_string())
        .bind(social_links.to_string())
        .execute(pool)
        .await {
            eprintln!("Column data generation error: {}", e);
        }
    }
    
    // Generate data for complex JSON table
    for i in 1..=100000 {
        let user_id = Uuid::new_v4().to_string();
        let complex_data = serde_json::json!({
            "id": user_id,
            "name": format!("Complex User {}", i),
            "email": format!("complex.user{}@example.com", i),
            "age": 20 + (i % 60),
            "profile": {
                "bio": format!("Complex bio for user {} with very long description that includes multiple sentences and various details about their background, interests, and activities.", i),
                "avatar_url": if i % 3 == 0 { Some(format!("https://example.com/avatar{}.jpg", i)) } else { None },
                "preferences": {
                    "theme": if i % 2 == 0 { "dark" } else { "light" },
                    "language": match i % 3 { 0 => "ja", 1 => "en", _ => "es" },
                    "notifications": if i % 4 == 0 { "true" } else { "false" },
                    "timezone": "Asia/Tokyo",
                    "currency": "JPY",
                    "date_format": "YYYY-MM-DD",
                    "time_format": "24h",
                    "accessibility": {
                        "high_contrast": i % 2 == 0,
                        "screen_reader": i % 3 == 0,
                        "font_size": "medium"
                    }
                },
                "social_links": [
                    format!("https://twitter.com/complex_user{}", i),
                    format!("https://github.com/complex_user{}", i),
                    format!("https://linkedin.com/in/complex_user{}", i),
                    format!("https://facebook.com/complex_user{}", i)
                ],
                "achievements": [
                    {
                        "id": format!("achievement_{}", i),
                        "name": format!("Achievement {}", i),
                        "description": format!("Description for achievement {}", i),
                        "earned_at": "2024-08-30T08:00:00Z",
                        "points": 100 + (i * 10)
                    },
                    {
                        "id": format!("achievement_{}_2", i),
                        "name": format!("Special Achievement {}", i),
                        "description": format!("Special description for achievement {}", i),
                        "earned_at": "2024-08-30T09:00:00Z",
                        "points": 200 + (i * 15)
                    }
                ],
                "statistics": {
                    "posts_count": 100 + (i * 5),
                    "followers_count": 500 + (i * 20),
                    "following_count": 200 + (i * 10),
                    "likes_received": 1000 + (i * 50),
                    "comments_made": 50 + (i * 3)
                }
            },
            "metadata": {
                "created_at": "2024-08-30T08:00:00Z",
                "last_login": "2024-08-30T21:00:00Z",
                "login_count": 100 + i,
                "is_verified": i % 5 == 0,
                "is_premium": i % 7 == 0,
                "tags": [
                    format!("tag_{}", i),
                    format!("category_{}", i % 10),
                    if i % 2 == 0 { "active" } else { "inactive" },
                    if i % 3 == 0 { "verified" } else { "unverified" }
                ]
            }
        });
        
        if let Err(e) = sqlx::query(
            r#"
            INSERT INTO users_json (id, data)
            VALUES (?, ?)
            "#
        )
        .bind(user_id)
        .bind(complex_data.to_string())
        .execute(pool)
        .await {
            eprintln!("Complex JSON data generation error: {}", e);
        }
    }
    
    println!("✅ Test data generation completed (100,000 records per table)");
}

async fn run_benchmarks(pool: &MySqlPool) {
    println!("🏃 Running benchmarks...");
    
    let test_counts = vec![1000, 10000, 50000, 100000];
    
    for count in test_counts {
        println!("\n📊 Column storage test ({}) records...", count);
        let start = std::time::Instant::now();
        
        let rows = match sqlx::query(
            r#"
            SELECT id, name, email, age, bio, avatar_url, preferences, social_links, created_at
            FROM users_column
            ORDER BY created_at DESC
            LIMIT ?
            "#
        )
        .bind(count)
        .fetch_all(pool)
        .await {
            Ok(rows) => rows,
            Err(e) => {
                eprintln!("Column benchmark error: {}", e);
                continue;
            }
        };

        let duration = start.elapsed();
        println!("  Result: {}ms, {} records processed", duration.as_millis(), rows.len());
        
        println!("📄 JSON storage test ({}) records...", count);
        let start = std::time::Instant::now();
        
        let rows = match sqlx::query(
            r#"
            SELECT data
            FROM users_json
            ORDER BY created_at DESC
            LIMIT ?
            "#
        )
        .bind(count)
        .fetch_all(pool)
        .await {
            Ok(rows) => rows,
            Err(e) => {
                eprintln!("JSON benchmark error: {}", e);
                continue;
            }
        };

        let duration = start.elapsed();
        println!("  Result: {}ms, {} records processed", duration.as_millis(), rows.len());
        
        // Complex processing benchmark
        println!("🔧 Complex JSON processing test ({}) records...", count);
        let start = std::time::Instant::now();
        
        let mut processed_users = Vec::new();
        for row in rows {
            let data_str: String = row.get("data");
            if let Ok(user_data) = serde_json::from_str::<serde_json::Value>(&data_str) {
                // Simulate complex processing
                let mut processed_user = user_data.clone();
                
                // Calculate statistics
                if let Some(profile) = processed_user.get_mut("profile") {
                    if let Some(stats) = profile.get_mut("statistics") {
                        if let Some(posts) = stats.get("posts_count").and_then(|v| v.as_u64()) {
                            if let Some(followers) = stats.get("followers_count").and_then(|v| v.as_u64()) {
                                let engagement_rate = if followers > 0 {
                                    (posts as f64 / followers as f64) * 100.0
                                } else {
                                    0.0
                                };
                                stats["engagement_rate"] = serde_json::json!(engagement_rate);
                            }
                        }
                    }
                }
                
                // Analyze tags
                if let Some(metadata) = processed_user.get_mut("metadata") {
                    if let Some(tags) = metadata.get("tags").and_then(|v| v.as_array()) {
                        let tag_count = tags.len();
                        let verified_tags = tags.iter().filter(|tag| tag.as_str() == Some("verified")).count();
                        metadata["tag_analysis"] = serde_json::json!({
                            "total_tags": tag_count,
                            "verified_tags": verified_tags,
                            "verification_rate": if tag_count > 0 { (verified_tags as f64 / tag_count as f64) * 100.0 } else { 0.0 }
                        });
                    }
                }
                
                // Aggregate achievements
                if let Some(profile) = processed_user.get("profile") {
                    if let Some(achievements) = profile.get("achievements").and_then(|v| v.as_array()) {
                        let total_points: u64 = achievements.iter()
                            .filter_map(|achievement| achievement.get("points").and_then(|v| v.as_u64()))
                            .sum();
                        
                        if let Some(profile_mut) = processed_user.get_mut("profile") {
                            profile_mut["total_achievement_points"] = serde_json::json!(total_points);
                        }
                    }
                }
                
                // String processing
                if let Some(profile) = processed_user.get("profile") {
                    if let Some(bio) = profile.get("bio").and_then(|v| v.as_str()) {
                        let word_count = bio.split_whitespace().count();
                        let char_count = bio.chars().count();
                        let sentence_count = bio.split('.').count() - 1;
                        
                        if let Some(profile_mut) = processed_user.get_mut("profile") {
                            profile_mut["bio_analysis"] = serde_json::json!({
                                "word_count": word_count,
                                "char_count": char_count,
                                "sentence_count": sentence_count,
                                "avg_words_per_sentence": if sentence_count > 0 { word_count as f64 / sentence_count as f64 } else { 0.0 }
                            });
                        }
                    }
                }
                
                processed_users.push(processed_user);
            }
        }

        let duration = start.elapsed();
        println!("  Result: {}ms, {} records processed (including complex processing)", duration.as_millis(), processed_users.len());
    }
    
    println!("\n🏁 Benchmark completed!");
}

// Function to generate complex JSON data
async fn generate_complex_data(
    State(pool): State<AppState>,
    Path(count): Path<i32>,
) -> Result<Json<serde_json::Value>, StatusCode> {
    for i in 1..=count {
        let user_id = Uuid::new_v4().to_string();
        
        // Generate more complex JSON data
        let complex_data = serde_json::json!({
            "id": user_id,
            "name": format!("Complex User {}", i),
            "email": format!("complex.user{}@example.com", i),
            "age": 20 + (i % 60),
            "profile": {
                "bio": format!("Complex bio for user {} with very long description that includes multiple sentences and various details about their background, interests, and activities.", i),
                "avatar_url": if i % 3 == 0 { Some(format!("https://example.com/avatar{}.jpg", i)) } else { None },
                "preferences": {
                    "theme": if i % 2 == 0 { "dark" } else { "light" },
                    "language": match i % 3 { 0 => "ja", 1 => "en", _ => "es" },
                    "notifications": if i % 4 == 0 { "true" } else { "false" },
                    "timezone": "Asia/Tokyo",
                    "currency": "JPY",
                    "date_format": "YYYY-MM-DD",
                    "time_format": "24h",
                    "accessibility": {
                        "high_contrast": i % 2 == 0,
                        "screen_reader": i % 3 == 0,
                        "font_size": "medium"
                    }
                },
                "social_links": [
                    format!("https://twitter.com/complex_user{}", i),
                    format!("https://github.com/complex_user{}", i),
                    format!("https://linkedin.com/in/complex_user{}", i),
                    format!("https://facebook.com/complex_user{}", i)
                ],
                "achievements": [
                    {
                        "id": format!("achievement_{}", i),
                        "name": format!("Achievement {}", i),
                        "description": format!("Description for achievement {}", i),
                        "earned_at": "2024-08-30T08:00:00Z",
                        "points": 100 + (i * 10)
                    },
                    {
                        "id": format!("achievement_{}_2", i),
                        "name": format!("Special Achievement {}", i),
                        "description": format!("Special description for achievement {}", i),
                        "earned_at": "2024-08-30T09:00:00Z",
                        "points": 200 + (i * 15)
                    }
                ],
                "statistics": {
                    "posts_count": 100 + (i * 5),
                    "followers_count": 500 + (i * 20),
                    "following_count": 200 + (i * 10),
                    "likes_received": 1000 + (i * 50),
                    "comments_made": 50 + (i * 3)
                }
            },
            "metadata": {
                "created_at": "2024-08-30T08:00:00Z",
                "last_login": "2024-08-30T21:00:00Z",
                "login_count": 100 + i,
                "is_verified": i % 5 == 0,
                "is_premium": i % 7 == 0,
                "tags": [
                    format!("tag_{}", i),
                    format!("category_{}", i % 10),
                    if i % 2 == 0 { "active" } else { "inactive" },
                    if i % 3 == 0 { "verified" } else { "unverified" }
                ]
            }
        });
        
        sqlx::query(
            r#"
            INSERT INTO users_json (id, data)
            VALUES (?, ?)
            "#
        )
        .bind(user_id)
        .bind(complex_data.to_string())
        .execute(&pool)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    }

    Ok(Json(serde_json::json!({
        "message": format!("Generated {} complex records in users_json", count)
    })))
}

// Benchmark function including complex processing
async fn benchmark_complex_processing(
    State(pool): State<AppState>,
    Path(count): Path<i32>,
) -> Result<Json<serde_json::Value>, StatusCode> {
    let start = std::time::Instant::now();
    
    // Get JSON data
    let rows = sqlx::query(
        r#"
        SELECT data
        FROM users_json
        ORDER BY created_at DESC
        LIMIT ?
        "#
    )
    .bind(count)
    .fetch_all(&pool)
    .await
    .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    // Execute complex processing
    let mut processed_users = Vec::new();
    
    for row in rows {
        let data_str: String = row.get("data");
        if let Ok(user_data) = serde_json::from_str::<serde_json::Value>(&data_str) {
            // Simulate complex processing
            let mut processed_user = user_data.clone();
            
            // 1. Calculate statistics
            if let Some(profile) = processed_user.get_mut("profile") {
                if let Some(stats) = profile.get_mut("statistics") {
                    if let Some(posts) = stats.get("posts_count").and_then(|v| v.as_u64()) {
                        if let Some(followers) = stats.get("followers_count").and_then(|v| v.as_u64()) {
                            let engagement_rate = if followers > 0 {
                                (posts as f64 / followers as f64) * 100.0
                            } else {
                                0.0
                            };
                            stats["engagement_rate"] = serde_json::json!(engagement_rate);
                        }
                    }
                }
            }
            
            // 2. Tag analysis
            if let Some(metadata) = processed_user.get_mut("metadata") {
                if let Some(tags) = metadata.get("tags").and_then(|v| v.as_array()) {
                    let tag_count = tags.len();
                    let verified_tags = tags.iter().filter(|tag| tag.as_str() == Some("verified")).count();
                    metadata["tag_analysis"] = serde_json::json!({
                        "total_tags": tag_count,
                        "verified_tags": verified_tags,
                        "verification_rate": if tag_count > 0 { (verified_tags as f64 / tag_count as f64) * 100.0 } else { 0.0 }
                    });
                }
            }
            
            // 3. Achievement aggregation
            if let Some(profile) = processed_user.get("profile") {
                if let Some(achievements) = profile.get("achievements").and_then(|v| v.as_array()) {
                    let total_points: u64 = achievements.iter()
                        .filter_map(|achievement| achievement.get("points").and_then(|v| v.as_u64()))
                        .sum();
                    
                    if let Some(profile_mut) = processed_user.get_mut("profile") {
                        profile_mut["total_achievement_points"] = serde_json::json!(total_points);
                    }
                }
            }
            
            // 4. Complex string processing
            if let Some(profile) = processed_user.get("profile") {
                if let Some(bio) = profile.get("bio").and_then(|v| v.as_str()) {
                    let word_count = bio.split_whitespace().count();
                    let char_count = bio.chars().count();
                    let sentence_count = bio.split('.').count() - 1;
                    
                    if let Some(profile_mut) = processed_user.get_mut("profile") {
                        profile_mut["bio_analysis"] = serde_json::json!({
                            "word_count": word_count,
                            "char_count": char_count,
                            "sentence_count": sentence_count,
                            "avg_words_per_sentence": if sentence_count > 0 { word_count as f64 / sentence_count as f64 } else { 0.0 }
                        });
                    }
                }
            }
            
            processed_users.push(processed_user);
        }
    }

    let duration = start.elapsed();

    Ok(Json(serde_json::json!({
        "storage_type": "complex_json_processing",
        "count": count,
        "duration_ms": duration.as_millis(),
        "records_processed": processed_users.len(),
        "processing_details": {
            "engagement_calculation": "completed",
            "tag_analysis": "completed", 
            "achievement_aggregation": "completed",
            "text_analysis": "completed"
        }
    })))
}