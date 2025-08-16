use crate::store::Store;


impl Store {
    pub fn create_user(&self) {
        print!("create user called")
    }
    pub fn get_user(&self) -> String {
        String::from("1")
    }
}