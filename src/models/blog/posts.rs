use chrono::{DateTime, Utc};
use serde::Serialize;
use sqlx::{Error, FromRow, PgPool};

#[derive(FromRow, Debug)]
pub struct Post {
    pub name: String,
    pub cover: String,
    pub description: Option<String>,
    pub content: String,
    pub date: DateTime<Utc>,
    pub author: Author,
}

#[derive(FromRow, Debug, Serialize)]
pub struct Author {
    pub fullname: String,
    pub picture: Option<String>,
}

pub async fn get(pool: &PgPool, id: i16) -> Result<Post, Error> {
    let row = sqlx::query!(
        r#"SELECT
            bp.name,
            bp.description,
            content,
            date,
            CONCAT(u.firstname, ' ', u.lastname) AS "fullname!",
            f.path AS "path?",
            c.path AS "cover"
        FROM blog_posts bp
        JOIN users u ON user_id = u.id
        JOIN files c ON bp.cover_id = c.id
        LEFT JOIN files f ON u.picture_id = f.id
        WHERE bp.id = $1"#,
        id
    )
    .fetch_one(pool)
    .await?;

    Ok(Post {
        name: row.name,
        cover: row.cover,
        description: row.description,
        content: row.content,
        date: row.date,
        author: Author {
            fullname: row.fullname,
            picture: row.path,
        },
    })
}

#[derive(FromRow, Debug)]
pub struct LatestArticle {
    pub name: String,
    pub uri: String,
    pub date: chrono::DateTime<chrono::Utc>,
    pub cover: String,
}

pub async fn get_latest(
    pool: &PgPool,
    category_id: Option<i16>,
) -> Result<Vec<LatestArticle>, Error> {
    let mut query = r#"SELECT
        bp.name, bp.uri, bp.date, f.path AS "cover"
    FROM blog_posts bp
    JOIN files f ON bp.cover_id = f.id
    WHERE is_published = TRUE"#
        .to_string();

    if category_id.is_some() {
        query += " AND bp.category_id = $1";
    }

    query += r#" ORDER BY "date" LIMIT 10"#;

    let mut query = sqlx::query_as::<_, LatestArticle>(&query);
    if let Some(category_id) = category_id {
        query = query.bind(category_id);
    }

    query.fetch_all(pool).await
}

#[derive(FromRow, Serialize, Debug)]
pub struct PostAdminInformations {
    id: i16,
    name: String,
    cover: String,
    date: DateTime<Utc>,
    is_published: Option<bool>,
    is_seo: Option<bool>,
    author: Author,
}

pub async fn _get_all(
    pool: &PgPool,
    page: Option<i64>,
) -> Result<Vec<PostAdminInformations>, Error> {
    let page = page.unwrap_or(0);

    let rows = sqlx::query!(
        r#"SELECT
            bp.id AS "id!",
            bp.name AS "name!",
            bp.date AS "date!",
            bp.is_published,
            bp.is_seo,
            CONCAT(u.firstname, ' ', u.lastname) AS "fullname!",
            f.path AS "path?",
            c.path AS "cover!"
        FROM blog_posts bp
        JOIN users u ON user_id = u.id
        JOIN files c ON bp.cover_id = c.id
        LEFT JOIN files f ON u.picture_id = f.id
        OFFSET $1 LIMIT 15"#,
        page * 15
    )
    .fetch_all(pool)
    .await?;

    Ok(rows
        .iter()
        .map(|row| PostAdminInformations {
            id: row.id,
            name: row.name.clone(),
            cover: row.cover.clone(),
            date: row.date,
            is_published: row.is_published,
            is_seo: row.is_published,
            author: Author {
                fullname: row.fullname.clone(),
                picture: row.path.clone(),
            },
        })
        .collect::<Vec<_>>())
}

pub async fn exists(pool: &PgPool, id: i16, name: &str) -> bool {
    sqlx::query!(
        "SELECT 1 AS one FROM blog_posts WHERE id = $1 AND name = $2",
        id,
        name
    )
    .fetch_one(pool)
    .await
    .is_ok()
}

pub async fn is_published(pool: &PgPool, id: i16) -> bool {
    sqlx::query!(
        "SELECT 1 AS one FROM blog_posts WHERE id = $1 AND is_published = TRUE",
        id
    )
    .fetch_one(pool)
    .await
    .is_ok()
}

pub async fn get_uri(pool: &PgPool, id: i16) -> Result<String, Error> {
    sqlx::query!("SELECT uri FROM blog_posts WHERE id = $1", id)
        .fetch_one(pool)
        .await
        .map(|record| record.uri)
}

pub async fn is_redirected(pool: &PgPool, id: i16, uri: &str) -> bool {
    sqlx::query!(
        "SELECT 1 AS one FROM blog_post_redirections WHERE post_id = $1 AND uri = $2",
        id,
        uri
    )
    .fetch_one(pool)
    .await
    .is_ok()
}
