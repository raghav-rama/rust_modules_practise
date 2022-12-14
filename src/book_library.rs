#![allow(dead_code)]
#![allow(unused_variables)]

pub mod book_library {
    use crate::book::Book;

    #[derive(Debug, Clone)]
    pub struct Books {
        pub collection: Vec<Book>,
    }
    impl Books {
        pub fn is_empty(&self) -> bool {
            self.collection.len() == 0
        }
        pub fn display_books(&self) {
            let mut i = 0;
            for title in self.collection.iter() {
                println!("{} - {:?}", i, title.get_title());
                i += 1;
            }
        }
    }
}
