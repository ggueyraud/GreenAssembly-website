use super::{Project, ProjectTile};
use sqlx::{Error, PgPool};

pub async fn exists(pool: &PgPool, id: i16) -> bool {
    sqlx::query!("SELECT 1 AS one FROM portfolio_projects WHERE id = $1", id)
        .fetch_one(pool)
        .await
        .is_ok()
}

pub async fn is_published(pool: &PgPool, id: i16) -> bool {
    sqlx::query!(
        "SELECT 1 AS one FROM portfolio_projects WHERE id = $1 AND is_published = TRUE",
        id
    )
    .fetch_one(pool)
    .await
    .is_ok()
}

pub async fn get_all(pool: &PgPool) -> Result<Vec<ProjectTile>, Error> {
    sqlx::query_as!(
        ProjectTile,
        "SELECT id, name, category_id FROM portfolio_projects ORDER BY id DESC"
    )
    .fetch_all(pool)
    .await
}

pub async fn get(pool: &PgPool, id: i16) -> Result<Project, Error> {
    sqlx::query_as!(
        Project,
        "SELECT name, description, content, is_seo FROM portfolio_projects WHERE id = $1 LIMIT 1",
        id
    )
    .fetch_one(pool)
    .await
}
