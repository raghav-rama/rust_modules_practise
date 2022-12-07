use crate::input::input::Data;

#[derive(Debug, Clone)]
pub struct Book {
    pub title: String,
    pub author: String,
    pub publisher: String,
    pub isbn: i32,
    pub price: f32,
}

impl Book {
    // initialize book with dummy values
    pub fn add_new_book() -> Self {
        Book {
            title: "abc".to_owned(),
            author: "abc".to_owned(),
            publisher: "abc".to_owned(),
            isbn: 123,
            price: 69.69,
        }
    }
    pub fn get_title(&self) -> String {
        self.title.clone()
    }
    pub fn set_title(&mut self) {
        println!("Enter book title:");
        let buff = Data::new().my_scan::<String>();
        self.title = buff;
    }
    pub fn get_author(&self) -> String {
        self.author.clone()
    }
    pub fn set_author(&mut self) {
        println!("Enter book author:");
        let buff = Data::new().my_scan::<String>();
        self.author = buff;
    }
    pub fn get_publisher(&self) -> String {
        self.publisher.clone()
    }
    pub fn set_publisher(&mut self) {
        println!("Enter book publisher:");
        let buff = Data::new().my_scan::<String>();
        self.publisher = buff;
    }
    pub fn get_isbn(&self) -> i32 {
        self.isbn
    }
    pub fn set_isbn(&mut self) {
        println!("Enter book isbn:");
        let buff = Data::new().my_scan::<i32>();
        self.isbn = buff;
    }
    pub fn get_price(&self) -> f32 {
        self.price
    }
    pub fn set_price(&mut self) {
        println!("Enter book price:");
        let buff = Data::new().my_scan::<f32>();
        self.price = buff;
    }
}
