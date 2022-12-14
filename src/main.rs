mod book;
mod book_library;
mod input;

use book::Book;
use book_library::book_library::Books;
use input::input::Data;

fn main() {
    println!("Welcome to library!");

    let mut fiction = Books {
        collection: Vec::new(),
    };
    loop {
        println!(
            "
    1. Add new dummy book
    2. Display author name
    3. Display book publisher
    4. Display ISBN
    5. Display Price
    6. Edit book
    7. Exit

    "
        );

        let choice = Data::new().my_scan::<i32>();

        match choice {
            1 => {
                fiction.collection.push(Book::add_new_book());
                println!("Added book with dummy values, edit using option 6");
            }
            2 => {
                if fiction.is_empty() {
                    println!("No books added")
                } else {
                    println!("Which book?");
                    fiction.display_books();

                    let choice = Data::new().my_scan::<usize>();
                    println!("{}", fiction.collection[choice].get_author());
                }
            }
            3 => {
                if fiction.is_empty() {
                    println!("No books added")
                } else {
                    println!("Which book?");
                    fiction.display_books();

                    let choice = Data::new().my_scan::<usize>();
                    println!("{}", fiction.collection[choice].get_publisher());
                }
            }
            4 => {
                if fiction.is_empty() {
                    println!("No books added")
                } else {
                    println!("Which book?");
                    fiction.display_books();

                    let choice = Data::new().my_scan::<usize>();
                    println!("{}", fiction.collection[choice].get_isbn());
                }
            }
            5 => {
                if fiction.is_empty() {
                    println!("No books added")
                } else {
                    println!("Which book?");
                    fiction.display_books();

                    let choice = Data::new().my_scan::<usize>();
                    println!("{}", fiction.collection[choice].get_price());
                }
            }
            6 => {
                if fiction.is_empty() {
                    println!("No books added");
                    continue;
                }
                println!("What do you wanna edit?");
                println!(
                    "
            1. title
            2. author
            3. publisher
            4. isbn
            5. price
                "
                );
                let choice = Data::new().my_scan::<usize>();
                println!("Which Book?");
                fiction.display_books();
                let book_choice = Data::new().my_scan::<usize>();

                match choice {
                    1 => fiction.collection[book_choice].set_title(),
                    2 => fiction.collection[book_choice].set_author(),
                    3 => fiction.collection[book_choice].set_publisher(),
                    4 => fiction.collection[book_choice].set_isbn(),
                    5 => fiction.collection[book_choice].set_price(),
                    _ => {
                        println!("Wrong choice")
                    }
                }
            }
            7 => {
                break;
            }
            _ => println!("Wrong choice"),
        }
    }
}
