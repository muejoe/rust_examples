
pub fn main() {
    let mut prod_db = ProdDatabase{data: 0};
    let mut handler = DataHandler{data: &mut prod_db, change_index: 0};

    handler.write_something(42);

    let result = handler.read_something();
    println!("Result is {result}");
}


pub trait DatabaseTrait {
    fn read_data(&self) -> u32;
    fn write_data(&mut self, data: u32);
}

pub struct ProdDatabase {
    data: u32
}
impl DatabaseTrait for ProdDatabase {
    fn read_data(&self) -> u32 {
        self.data
    }

    fn write_data(&mut self, data: u32) {
        self.data = data
    }
}


// 'a is a lifetime. Here we say to the compiler that data
// has at least the same lifetime as the DataHandler object.
// With this the compiler can ensure that the reference stays valid.
pub struct DataHandler<'a>{
    data: &'a mut dyn DatabaseTrait,
    change_index: i32
}
impl DataHandler<'_> {
    pub fn read_something(&self) -> u32 {
        self.data.read_data()
    }

    pub fn write_something(&mut self, something: u32) {
        self.data.write_data(something);
        self.change_index += 1;
    }
}



#[cfg(test)]
mod tests {
    use std::cell::RefCell;
    use super::*; // to access the unit under test

    pub struct MockDatabase {
        pub read_data_return: u32,
        pub read_called: RefCell<u32>, // for able to change inner value with runtime borrow check
        pub write_data_given_data: u32,
        pub write_called: u32
    }
    impl MockDatabase {
        pub fn new() -> Self {
            Self{read_data_return: 0, read_called: RefCell::new(0),
            write_data_given_data: 0, write_called: 0}
        }
    }
    impl DatabaseTrait for MockDatabase {
        fn read_data(&self) -> u32 {
            *self.read_called.borrow_mut() += 1;
            self.read_data_return
        }
    
        fn write_data(&mut self, data: u32) {
            self.write_called += 1;
            self.write_data_given_data = data
        }
    }


    #[test]
    fn test_data_handler() {
        let mut mock_db = MockDatabase::new();
        mock_db.read_data_return = 77;

        let mut handler = DataHandler{data: &mut mock_db, change_index: 0};

        const TEST_INPUT : u32 = 42;
        handler.write_something(TEST_INPUT);
        let result = handler.read_something();

        assert_eq!(mock_db.write_data_given_data, TEST_INPUT);
        assert_eq!(mock_db.write_called, 1);
        assert_eq!(result, mock_db.read_data_return);
        assert_eq!(*mock_db.read_called.borrow(), 1);
    } 
}