use axum::{
    extract::{Path, Query, State},
    http::StatusCode,
    response::Json,
    routing::{get},
    Router,
};
use serde::{Deserialize, Serialize};
use sqlx::{MySqlPool, Row, types::Json as SqlxJson};
use std::collections::HashMap;
use tower_http::cors::CorsLayer;
use tracing::info;
use uuid::Uuid;
use chrono::{DateTime, Utc};

#[derive(Debug, Serialize, Deserialize)]
struct Post {
    id: String,
    title: String,
    content: String,
    excerpt: String,
    slug: String,
    author: String,
    published: bool,
    featured_image: Option<String>,
    tags: SqlxJson<Vec<String>>,
    created_at: DateTime<Utc>,
    updated_at: DateTime<Utc>,
}

#[derive(Debug, Serialize, Deserialize)]
struct CreatePost {
    title: String,
    content: String,
    excerpt: String,
    slug: String,
    author: String,
    published: bool,
    featured_image: Option<String>,
    tags: Vec<String>,
}

#[derive(Debug, Serialize, Deserialize)]
struct UpdatePost {
    title: Option<String>,
    content: Option<String>,
    excerpt: Option<String>,
    slug: Option<String>,
    author: Option<String>,
    published: Option<bool>,
    featured_image: Option<String>,
    tags: Option<Vec<String>>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
struct SiteConfig {
    site_name: String,
    site_description: String,
    site_url: String,
    site_author: String,
    site_email: String,
    site_keywords: String,
    site_language: String,
    twitter_handle: String,
    github_url: String,
    enable_comments: bool,
    posts_per_page: i32,
    enable_search: bool,
}

#[derive(Clone)]
struct AppState {
    db: MySqlPool,
    config: SiteConfig,
}

#[derive(Debug, Deserialize)]
struct PostQuery {
    page: Option<i32>,
    limit: Option<i32>,
    tag: Option<String>,
    search: Option<String>,
}

async fn get_posts(
    Query(params): Query<PostQuery>,
    State(state): State<AppState>,
) -> Result<Json<HashMap<String, serde_json::Value>>, StatusCode> {
    let page = params.page.unwrap_or(1).max(1);
    let limit = params.limit.unwrap_or(state.config.posts_per_page).min(50);
    let offset = (page - 1) * limit;

    let mut query = "SELECT id, title, content, excerpt, slug, author, published, featured_image, tags, created_at, updated_at FROM posts WHERE published = true".to_string();
    let mut count_query = "SELECT COUNT(*) as total FROM posts WHERE published = true".to_string();
    
    if let Some(tag) = &params.tag {
        query.push_str(&format!(" AND JSON_CONTAINS(tags, '{}')", serde_json::to_string(tag).unwrap()));
        count_query.push_str(&format!(" AND JSON_CONTAINS(tags, '{}')", serde_json::to_string(tag).unwrap()));
    }
    
    if let Some(search) = &params.search {
        let search_term = format!("%{}%", search);
        query.push_str(&format!(" AND (title LIKE '{}' OR content LIKE '{}')", search_term, search_term));
        count_query.push_str(&format!(" AND (title LIKE '{}' OR content LIKE '{}')", search_term, search_term));
    }
    
    query.push_str(" ORDER BY created_at DESC LIMIT ? OFFSET ?");

    let posts_result = sqlx::query(&query)
        .bind(limit)
        .bind(offset)
        .fetch_all(&state.db)
        .await;

    let count_result = sqlx::query(&count_query)
        .fetch_one(&state.db)
        .await;

    match (posts_result, count_result) {
        (Ok(rows), Ok(count_row)) => {
            let posts: Vec<Post> = rows.into_iter().map(|row| {
                Post {
                    id: row.get("id"),
                    title: row.get("title"),
                    content: row.get("content"),
                    excerpt: row.get("excerpt"),
                    slug: row.get("slug"),
                    author: row.get("author"),
                    published: row.get("published"),
                    featured_image: row.get("featured_image"),
                    tags: row.get("tags"),
                    created_at: row.get("created_at"),
                    updated_at: row.get("updated_at"),
                }
            }).collect();

            let total: i64 = count_row.get("total");
            let total_pages = (total as f64 / limit as f64).ceil() as i32;

            let mut response = HashMap::new();
            response.insert("posts".to_string(), serde_json::to_value(posts).unwrap());
            response.insert("pagination".to_string(), serde_json::json!({
                "current_page": page,
                "total_pages": total_pages,
                "total_posts": total,
                "posts_per_page": limit
            }));

            Ok(Json(response))
        }
        _ => Err(StatusCode::INTERNAL_SERVER_ERROR),
    }
}

async fn get_post_by_slug(
    Path(slug): Path<String>,
    State(state): State<AppState>,
) -> Result<Json<Post>, StatusCode> {
    let result = sqlx::query(
        "SELECT id, title, content, excerpt, slug, author, published, featured_image, tags, created_at, updated_at FROM posts WHERE slug = ? AND published = true"
    )
    .bind(&slug)
    .fetch_one(&state.db)
    .await;

    match result {
        Ok(row) => {
            let post = Post {
                id: row.get("id"),
                title: row.get("title"),
                content: row.get("content"),
                excerpt: row.get("excerpt"),
                slug: row.get("slug"),
                author: row.get("author"),
                published: row.get("published"),
                featured_image: row.get("featured_image"),
                tags: row.get("tags"),
                created_at: row.get("created_at"),
                updated_at: row.get("updated_at"),
            };
            Ok(Json(post))
        }
        Err(_) => Err(StatusCode::NOT_FOUND),
    }
}

async fn create_post(
    State(state): State<AppState>,
    Json(payload): Json<CreatePost>,
) -> Result<Json<Post>, StatusCode> {
    let id = Uuid::new_v4().to_string();
    let now = Utc::now();

    let result = sqlx::query(
        "INSERT INTO posts (id, title, content, excerpt, slug, author, published, featured_image, tags, created_at, updated_at) VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)"
    )
    .bind(&id)
    .bind(&payload.title)
    .bind(&payload.content)
    .bind(&payload.excerpt)
    .bind(&payload.slug)
    .bind(&payload.author)
    .bind(payload.published)
    .bind(&payload.featured_image)
    .bind(SqlxJson(&payload.tags))
    .bind(now)
    .bind(now)
    .execute(&state.db)
    .await;

    match result {
        Ok(_) => {
            let post = Post {
                id,
                title: payload.title,
                content: payload.content,
                excerpt: payload.excerpt,
                slug: payload.slug,
                author: payload.author,
                published: payload.published,
                featured_image: payload.featured_image,
                tags: SqlxJson(payload.tags),
                created_at: now,
                updated_at: now,
            };
            Ok(Json(post))
        }
        Err(_) => Err(StatusCode::INTERNAL_SERVER_ERROR),
    }
}

async fn get_admin_posts(
    Query(params): Query<HashMap<String, String>>,
    State(state): State<AppState>,
) -> Result<Json<HashMap<String, serde_json::Value>>, StatusCode> {
    let page: i32 = params.get("page").and_then(|p| p.parse().ok()).unwrap_or(1);
    let limit: i32 = params.get("limit").and_then(|l| l.parse().ok()).unwrap_or(10);
    let offset = (page - 1) * limit;

    let mut query = "SELECT id, title, content, excerpt, slug, author, published, featured_image, tags, created_at, updated_at FROM posts".to_string();
    let mut count_query = "SELECT COUNT(*) as total FROM posts".to_string();
    let mut conditions = Vec::new();

    if let Some(search) = params.get("search") {
        if !search.is_empty() {
            conditions.push(format!("(title LIKE '%{}%' OR content LIKE '%{}%')", search, search));
        }
    }

    if let Some(tag) = params.get("tag") {
        if !tag.is_empty() {
            conditions.push(format!("JSON_CONTAINS(tags, '\"{}\"')", tag));
        }
    }

    if !conditions.is_empty() {
        let where_clause = format!(" WHERE {}", conditions.join(" AND "));
        query.push_str(&where_clause);
        count_query.push_str(&where_clause);
    }

    query.push_str(" ORDER BY created_at DESC LIMIT ? OFFSET ?");

    let posts_result = sqlx::query(&query)
        .bind(limit)
        .bind(offset)
        .fetch_all(&state.db)
        .await;

    let count_result = sqlx::query(&count_query)
        .fetch_one(&state.db)
        .await;

    match (posts_result, count_result) {
        (Ok(rows), Ok(count_row)) => {
            let posts: Vec<Post> = rows.into_iter().map(|row| {
                Post {
                    id: row.get("id"),
                    title: row.get("title"),
                    content: row.get("content"),
                    excerpt: row.get("excerpt"),
                    slug: row.get("slug"),
                    author: row.get("author"),
                    published: row.get("published"),
                    featured_image: row.get("featured_image"),
                    tags: row.get("tags"),
                    created_at: row.get("created_at"),
                    updated_at: row.get("updated_at"),
                }
            }).collect();

            let total: i64 = count_row.get("total");
            let total_pages = (total as f64 / limit as f64).ceil() as i32;

            let mut response = HashMap::new();
            response.insert("posts".to_string(), serde_json::to_value(posts).unwrap());
            response.insert("pagination".to_string(), serde_json::json!({
                "current_page": page,
                "total_pages": total_pages,
                "total_posts": total,
                "posts_per_page": limit
            }));

            Ok(Json(response))
        }
        _ => Err(StatusCode::INTERNAL_SERVER_ERROR),
    }
}

async fn update_post(
    Path(id): Path<String>,
    State(state): State<AppState>,
    Json(payload): Json<UpdatePost>,
) -> Result<Json<Post>, StatusCode> {
    let now = Utc::now();
    
    // First, get the current post
    let current_post_result = sqlx::query(
        "SELECT id, title, content, excerpt, slug, author, published, featured_image, tags, created_at, updated_at FROM posts WHERE id = ?"
    )
    .bind(&id)
    .fetch_one(&state.db)
    .await;
    
    let current_post = match current_post_result {
        Ok(row) => {
            Post {
                id: row.get("id"),
                title: row.get("title"),
                content: row.get("content"),
                excerpt: row.get("excerpt"),
                slug: row.get("slug"),
                author: row.get("author"),
                published: row.get("published"),
                featured_image: row.get("featured_image"),
                tags: row.get("tags"),
                created_at: row.get("created_at"),
                updated_at: row.get("updated_at"),
            }
        }
        Err(_) => return Err(StatusCode::NOT_FOUND),
    };
    
    // Use current values as defaults, update with new values if provided
    let title = payload.title.unwrap_or(current_post.title);
    let content = payload.content.unwrap_or(current_post.content);
    let excerpt = payload.excerpt.unwrap_or(current_post.excerpt);
    let slug = payload.slug.unwrap_or(current_post.slug);
    let author = payload.author.unwrap_or(current_post.author);
    let published = payload.published.unwrap_or(current_post.published);
    let featured_image = payload.featured_image.or(current_post.featured_image);
    let tags = payload.tags.unwrap_or(current_post.tags.0);
    
    let result = sqlx::query(
        "UPDATE posts SET title = ?, content = ?, excerpt = ?, slug = ?, author = ?, published = ?, featured_image = ?, tags = ?, updated_at = ? WHERE id = ?"
    )
    .bind(&title)
    .bind(&content)
    .bind(&excerpt)
    .bind(&slug)
    .bind(&author)
    .bind(published)
    .bind(&featured_image)
    .bind(SqlxJson(&tags))
    .bind(now)
    .bind(&id)
    .execute(&state.db)
    .await;
    
    match result {
        Ok(result) if result.rows_affected() > 0 => {
            let post = Post {
                id: current_post.id,
                title,
                content,
                excerpt,
                slug,
                author,
                published,
                featured_image,
                tags: SqlxJson(tags),
                created_at: current_post.created_at,
                updated_at: now,
            };
            Ok(Json(post))
        }
        _ => Err(StatusCode::NOT_FOUND),
    }
}

async fn delete_post(
    Path(id): Path<String>,
    State(state): State<AppState>,
) -> Result<StatusCode, StatusCode> {
    let result = sqlx::query("DELETE FROM posts WHERE id = ?")
        .bind(&id)
        .execute(&state.db)
        .await;

    match result {
        Ok(result) if result.rows_affected() > 0 => Ok(StatusCode::NO_CONTENT),
        _ => Err(StatusCode::NOT_FOUND),
    }
}

async fn get_post_by_id(
    Path(id): Path<String>,
    State(state): State<AppState>,
) -> Result<Json<Post>, StatusCode> {
    let result = sqlx::query(
        "SELECT id, title, content, excerpt, slug, author, published, featured_image, tags, created_at, updated_at FROM posts WHERE id = ?"
    )
    .bind(&id)
    .fetch_one(&state.db)
    .await;

    match result {
        Ok(row) => {
            let post = Post {
                id: row.get("id"),
                title: row.get("title"),
                content: row.get("content"),
                excerpt: row.get("excerpt"),
                slug: row.get("slug"),
                author: row.get("author"),
                published: row.get("published"),
                featured_image: row.get("featured_image"),
                tags: row.get("tags"),
                created_at: row.get("created_at"),
                updated_at: row.get("updated_at"),
            };
            Ok(Json(post))
        }
        Err(_) => Err(StatusCode::NOT_FOUND),
    }
}

async fn get_site_config(State(state): State<AppState>) -> Json<SiteConfig> {
    Json(state.config.clone())
}

async fn health_check() -> &'static str {
    "OK"
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv::dotenv().ok();
    
    tracing_subscriber::fmt::init();

    let database_url = std::env::var("DATABASE_URL")
        .expect("DATABASE_URL must be set");
    
    let pool = MySqlPool::connect(&database_url).await?;
    
    sqlx::migrate!("./migrations")
        .run(&pool)
        .await?;

    let config = SiteConfig {
        site_name: std::env::var("SITE_NAME").unwrap_or_else(|_| "My Blog".to_string()),
        site_description: std::env::var("SITE_DESCRIPTION").unwrap_or_else(|_| "A beautiful blog".to_string()),
        site_url: std::env::var("SITE_URL").unwrap_or_else(|_| "http://localhost:4321".to_string()),
        site_author: std::env::var("SITE_AUTHOR").unwrap_or_else(|_| "Blog Author".to_string()),
        site_email: std::env::var("SITE_EMAIL").unwrap_or_else(|_| "author@example.com".to_string()),
        site_keywords: std::env::var("SITE_KEYWORDS").unwrap_or_else(|_| "blog".to_string()),
        site_language: std::env::var("SITE_LANGUAGE").unwrap_or_else(|_| "en".to_string()),
        twitter_handle: std::env::var("TWITTER_HANDLE").unwrap_or_else(|_| "@blog".to_string()),
        github_url: std::env::var("GITHUB_URL").unwrap_or_else(|_| "https://github.com".to_string()),
        enable_comments: std::env::var("ENABLE_COMMENTS").unwrap_or_else(|_| "true".to_string()).parse().unwrap_or(true),
        posts_per_page: std::env::var("POSTS_PER_PAGE").unwrap_or_else(|_| "10".to_string()).parse().unwrap_or(10),
        enable_search: std::env::var("ENABLE_SEARCH").unwrap_or_else(|_| "true".to_string()).parse().unwrap_or(true),
    };

    let state = AppState { db: pool, config };

    let app = Router::new()
        .route("/api/posts", get(get_posts).post(create_post))
        .route("/api/posts/:slug", get(get_post_by_slug))
        .route("/api/admin/posts", get(get_admin_posts))
        .route("/api/admin/posts/:id", get(get_post_by_id).put(update_post).delete(delete_post))
        .route("/api/config", get(get_site_config))
        .route("/health", get(health_check))
        .layer(CorsLayer::permissive())
        .with_state(state);

    let host = std::env::var("SERVER_HOST").unwrap_or_else(|_| "127.0.0.1".to_string());
    let port = std::env::var("SERVER_PORT").unwrap_or_else(|_| "3000".to_string());
    let addr = format!("{}:{}", host, port);
    
    info!("Server starting on {}", addr);
    
    let listener = tokio::net::TcpListener::bind(&addr).await?;
    axum::serve(listener, app).await?;

    Ok(())
}
