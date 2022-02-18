use sqlx::FromRow;

pub mod categories;
pub mod pictures;
pub mod projects;

#[derive(FromRow)]
pub struct Category {
    pub id: i16,
    pub name: String,
}

#[derive(FromRow)]
pub struct ProjectTile {
    pub id: i16,
    pub name: String,
    pub category_id: i16,
}

#[derive(FromRow)]
pub struct Project {
    pub name: String,
    pub description: Option<String>,
    pub content: String,
    pub is_seo: Option<bool>,
}
