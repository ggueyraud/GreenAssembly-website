use sqlx::{Error, FromRow, PgPool};

#[derive(FromRow, Debug)]
pub struct Post {
    pub name: String,
    pub cover: String,
    pub description: Option<String>,
    pub content: String,
    pub date: chrono::DateTime<chrono::Utc>,
    pub author: Author,
}

#[derive(FromRow, Debug)]
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
    pub cover: String
}

pub async fn get_latest(pool: &PgPool, category_id: Option<i16>) -> Result<Vec<LatestArticle>, Error> {
    let mut query = r#"SELECT
        bp.name, bp.uri, bp.date, f.path AS "cover"
    FROM blog_posts bp
    JOIN files f ON bp.cover_id = f.id
    WHERE is_published = TRUE"#.to_string();

    if category_id.is_some() {
        query += " AND bp.category_id = $1";
    }

    query += r#" ORDER BY "date" LIMIT 10"#;

    let mut query = sqlx::query_as::<_, LatestArticle>(&query);
    if let Some(category_id) = category_id {
        query = query.bind(category_id);
    }

    let rows = query
        .fetch_all(pool)
        .await?;

    // let query = 

    // let rows = sqlx::query_as!(
    //     LatestArticle,
    //     r#"SELECT
    //         bp.name,
    //         bp.uri,
    //         bp.date,
    //         f.path AS "cover"
    //     FROM blog_posts bp
    //     JOIN files f ON bp.cover_id = f.id
    //     WHERE is_published = TRUE
    //     ORDER BY "date"
    //     LIMIT 10"#
    // )
    //     .fetch_all(pool)
    //     .await?;

    Ok(rows)
}

pub async fn get_all_published<
    T: std::marker::Unpin + std::marker::Send + for<'c> sqlx::FromRow<'c, sqlx::postgres::PgRow>,
>(
    pool: &PgPool,
    fields: &str,
) -> Result<Vec<T>, Error> {
    let categories = sqlx::query_as::<_, T>(&format!(
        r#"SELECT
            {}
        FROM blog_posts bp
        JOIN FILES f ON bp.cover_id = f.id
        JOIN users u ON bp.user_id = u.id
        WHERE is_published = TRUE
        ORDER BY "order""#,
        fields
    ))
    .fetch_all(pool)
    .await?;

    Ok(categories)
}

pub async fn exists(pool: &PgPool, id: i16) -> bool {
    sqlx::query!("SELECT 1 AS one FROM blog_posts WHERE id = $1", id)
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
