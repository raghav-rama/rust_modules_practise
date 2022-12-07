// just a module to input data

pub mod input {
    use std::{io, str::FromStr, fmt::Debug};
    #[derive(Debug)]
    pub struct Data {
        pub input: String,
    }
    impl Data {
        pub fn new() -> Self {
            Data{input: "".to_owned()}
        }
        pub fn my_scan<T: FromStr>(&mut self) -> T where <T as FromStr>::Err: Debug {
            io::stdin()
            .read_line(&mut self.input)
            .expect("Error reading stdin");
            self.input
            .trim()
            .parse::<T>()
            .unwrap()
        }
    }
}