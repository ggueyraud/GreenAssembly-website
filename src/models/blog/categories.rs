use sqlx::{Error, FromRow, PgPool};

#[derive(FromRow)]
pub struct CategoryInformations {
    pub name: String,
    pub description: Option<String>,
}

pub async fn get(pool: &PgPool, id: i16) -> Result<CategoryInformations, Error> {
    let row = sqlx::query_as!(
        CategoryInformations,
        "SELECT name, description FROM blog_categories WHERE id = $1",
        id
    )
    .fetch_one(pool)
    .await?;

    Ok(row)
}

#[derive(FromRow)]
pub struct Category {
    pub name: String,
    pub uri: String,
}

pub async fn get_all(pool: &PgPool) -> Result<Vec<Category>, Error> {
    let rows = sqlx::query_as!(
        Category,
        r#"SELECT name, uri FROM blog_categories WHERE is_visible = TRUE ORDER BY "order""#
    )
    .fetch_all(pool)
    .await?;

    Ok(rows)
}

// pub async fn get_all<
//     T: std::marker::Unpin + std::marker::Send + for<'c> sqlx::FromRow<'c, sqlx::postgres::PgRow>,
// >(
//     pool: &PgPool,
//     fields: &str,
// ) -> Result<Vec<T>, Error> {
//     let categories = sqlx::query_as::<_, T>(&format!(
//         r#"SELECT
//             {}
//         FROM blog_categories
//         ORDER BY "order""#,
//         fields
//     ))
//     .fetch_all(pool)
//     .await?;

//     Ok(categories)
// }

pub async fn exists(pool: &PgPool, id: i16) -> bool {
    sqlx::query!("SELECT 1 AS one FROM blog_categories WHERE id = $1", id)
        .fetch_one(pool)
        .await
        .is_ok()
}
