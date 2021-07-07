use askama::Template;

#[derive(Template)]
#[template(path = "components/employee.html")]
pub struct Employee {
    pub fullname: String,
    pub position_held: String, // Nom du poste occupé
    pub description: String,
    pub picture: String,
}
