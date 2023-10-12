#[derive(Clone,PartialEq)]
pub struct Login {
    pub name_of_service: String,
    pub login: String,
    pub password: String,
    // date
}
impl Login{
    pub fn new(name_of_service: &str, login: &str, password: &str) ->Login{
        Self{
            name_of_service: name_of_service.to_string(),
            login: login.to_string(),
            password: password.to_string(),
        }
    }
}