pub mod answers;
pub mod categories;

pub struct Category {
    pub id: i16,
    pub name: String,
}

pub struct Answer {
    pub id: i16,
    pub question: String,
    pub answer: String,
}
