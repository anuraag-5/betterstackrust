pub struct Store {
    // conn: Connection
}

// impl Default for Store {
//     // fn default() -> Self {
//     //     // return Self { conn: "" } 
//     // }
// }

impl Store {
    pub fn create_user(&self) {
        print!("create user called")
    }
    pub fn create_website(&self) -> String {
        String::from("1")
    }
}