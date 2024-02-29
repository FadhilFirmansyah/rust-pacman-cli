pub mod input_util{
    use std::io::{stdin, stdout, Write};

    pub fn input_user() -> String{
        stdout().flush().unwrap();
        
        let mut input_menu = String::new();
        stdin().read_line(&mut input_menu).unwrap();
        
        let input: Result<String, _> = input_menu.trim().parse();
        match input {
            Ok(val) => val,
            Err(_) => None.unwrap()
        }
    }
}