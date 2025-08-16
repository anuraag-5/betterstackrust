use crate::store::Store;

impl Store {
    pub fn create_website(&self) {
        print!("create user called")
    }
    pub fn get_website(&self) -> String {
        String::from("1")
    }
}