use crate::models::users;
use askama::Template;

#[derive(Template)]
#[template(path = "components/employee.html")]
pub struct Employee {
    pub fullname: String,
    pub job: String,
    pub description: Option<String>,
    pub picture: String,
}

impl From<users::Employee> for Employee {
    fn from(employee: users::Employee) -> Self {
        let filename = employee.picture.split('.').collect::<Vec<_>>();
        let filename = filename.get(0).unwrap();

        Self {
            fullname: employee.fullname,
            job: employee.job,
            description: employee.description,
            picture: filename.to_string(),
        }
    }
}
