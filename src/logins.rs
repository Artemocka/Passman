pub struct Login {
    name_of_service: String,
    login: String,
    password: String,
    // date
}
impl Login{
    pub fn new(name_of_service: String,login:String,password:String)->Login{
        Self{
            name_of_service,
            login,
            password,
        }
    }
}