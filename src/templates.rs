use askama::Template;

#[derive(Template)]
#[template(path = "components/employee.html")]
pub struct Employee {
    pub fullname: String,
    pub position_held: String, // Nom du poste occupé
    pub description: Option<String>,
    pub picture: String,
}

impl From<crate::services::employees::Employee> for Employee {
    fn from(employee: crate::services::employees::Employee) -> Self {
        Self {
            fullname: employee.fullname,
            position_held: employee.job,
            description: employee.description,
            picture: employee.picture,
        }
    }
}
